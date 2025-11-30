<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use function is_array;

use ReflectionMethod;
use ReflectionNamedType;
use ReflectionParameter;
use RuntimeException;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Handler wrapper for controller methods discovered via attributes.
 *
 * This handler adapts controller methods to the HandlerInterface by:
 * 1. Extracting parameters from the request based on parameter attributes
 * 2. Invoking the controller method with resolved parameters
 * 3. Converting the return value to a Response
 *
 * @internal
 */
final class ControllerMethodHandler implements HandlerInterface
{
    /**
     * @param object $controller Controller instance
     * @param ReflectionMethod $method Method to invoke
     */
    public function __construct(
        private readonly object $controller,
        private readonly ReflectionMethod $method,
    ) {
    }

    public function matches(Request $request): bool
    {
        // Controllers always match once routed
        return true;
    }

    public function handle(Request $request): Response
    {
        $params = $this->resolveParameters($request);
        $result = $this->method->invokeArgs($this->controller, $params);

        return $this->convertToResponse($result);
    }

    /**
     * Resolve method parameters from the request.
     *
     * @return array<int, mixed>
     */
    private function resolveParameters(Request $request): array
    {
        $params = [];
        foreach ($this->method->getParameters() as $param) {
            $params[] = $this->resolveParameter($param, $request);
        }
        return $params;
    }

    /**
     * Resolve a single parameter from the request.
     *
     * Parameters are resolved in this order:
     * 1. Check if default value is a Param instance (Body, Query, Path, etc.) - use its extraction logic
     * 2. Check if parameter name matches a path parameter
     * 3. Check if parameter name matches a query parameter
     * 4. Use default value if available
     * 5. Return null if nullable
     * 6. Throw exception for required parameters
     */
    private function resolveParameter(ReflectionParameter $param, Request $request): mixed
    {
        $name = $param->getName();

        // Check if default value is a Param instance (Body, Query, Path, Header, Cookie)
        if ($param->isDefaultValueAvailable()) {
            try {
                $defaultValue = $param->getDefaultValue();

                if ($defaultValue instanceof Body) {
                    return $request->body ?? $defaultValue->getDefault();
                }

                if ($defaultValue instanceof Query) {
                    $value = $request->queryParams[$name] ?? null;
                    if ($value === null) {
                        return $defaultValue->getDefault();
                    }
                    return \count($value) === 1 ? $value[0] : $value;
                }

                if ($defaultValue instanceof Path) {
                    return $request->pathParams[$name] ?? $defaultValue->getDefault();
                }

                if ($defaultValue instanceof Header) {
                    return $request->headers[$name] ?? $defaultValue->getDefault();
                }

                if ($defaultValue instanceof Cookie) {
                    return $request->cookies[$name] ?? $defaultValue->getDefault();
                }
            } catch (\ReflectionException $e) {
                // Default value might not be available at this point, continue with other resolution
            }
        }

        // Check for complex types (DI container resolution)
        $paramType = $param->getType();
        if ($paramType instanceof ReflectionNamedType && !$paramType->isBuiltin()) {
            // Complex type - could be DI container resolution
            // For now, just return default if optional
            if ($param->isOptional()) {
                return $param->isDefaultValueAvailable() ? $param->getDefaultValue() : null;
            }
            throw new RuntimeException(
                "Cannot resolve parameter '{$name}' of type '{$paramType->getName()}'"
            );
        }

        // Check if it's a path parameter by name
        if (isset($request->pathParams[$name])) {
            return $request->pathParams[$name];
        }

        // Check if it's a query parameter by name
        if (isset($request->queryParams[$name])) {
            $value = $request->queryParams[$name];
            return \count($value) === 1 ? $value[0] : $value;
        }

        // Check if it has a default value
        if ($param->isDefaultValueAvailable()) {
            return $param->getDefaultValue();
        }

        // Check if it's nullable
        if ($param->allowsNull()) {
            return null;
        }

        throw new RuntimeException(
            "Cannot resolve required parameter '{$name}' for method '{$this->method->getName()}'"
        );
    }

    /**
     * Convert controller method result to Response.
     */
    private function convertToResponse(mixed $result): Response
    {
        if ($result instanceof Response) {
            return $result;
        }

        // Array or object - convert to JSON response
        if (\is_array($result) || \is_object($result)) {
            return new Response(
                statusCode: 200,
                body: $result,
                headers: ['Content-Type' => 'application/json'],
            );
        }

        // String - text response
        if (\is_string($result)) {
            return new Response(
                statusCode: 200,
                body: $result,
                headers: ['Content-Type' => 'text/plain'],
            );
        }

        // Null - 204 No Content
        if ($result === null) {
            return new Response(statusCode: 204);
        }

        // Scalar types - convert to JSON for consistency
        return new Response(
            statusCode: 200,
            body: ['result' => $result],
            headers: ['Content-Type' => 'application/json'],
        );
    }
}
