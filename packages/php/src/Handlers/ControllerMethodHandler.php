<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use function is_array;

use ReflectionMethod;
use ReflectionNamedType;
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
    /** @var array<string, array<int, array{name: string, kind: string, default: mixed, has_default: bool, allows_null: bool, lookup_key?: string, is_optional?: bool, type?: string}>> */
    private static array $parameterPlanCache = [];

    /** @var array<int, array{name: string, kind: string, default: mixed, has_default: bool, allows_null: bool, lookup_key?: string, is_optional?: bool, type?: string}> */
    private array $parameterPlan;

    /**
     * @param object $controller Controller instance
     * @param ReflectionMethod $method Method to invoke
     */
    public function __construct(
        private readonly object $controller,
        private readonly ReflectionMethod $method,
    ) {
        $cacheKey = $method->getDeclaringClass()->getName() . '::' . $method->getName();
        if (!isset(self::$parameterPlanCache[$cacheKey])) {
            self::$parameterPlanCache[$cacheKey] = $this->buildParameterPlan($method);
        }
        $this->parameterPlan = self::$parameterPlanCache[$cacheKey];
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
     * Make handler callable for Rust FFI compatibility.
     *
     * Implements the __invoke magic method so that handler instances
     * can be passed as PHP callables to the Rust extension.
     */
    public function __invoke(Request $request): Response
    {
        return $this->handle($request);
    }

    /**
     * Resolve method parameters from the request.
     *
     * @return array<int, mixed>
     */
    private function resolveParameters(Request $request): array
    {
        $params = [];
        foreach ($this->parameterPlan as $plan) {
            $params[] = $this->resolvePlannedParameter($plan, $request);
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
    /**
     * @param array{name: string, kind: string, default: mixed, has_default: bool, allows_null: bool, lookup_key?: string, is_optional?: bool, type?: string} $plan
     */
    private function resolvePlannedParameter(array $plan, Request $request): mixed
    {
        $name = $plan['name'];
        $kind = $plan['kind'];
        $defaultValue = $plan['default'];
        $lookupKey = $plan['lookup_key'] ?? $name;

        if ($kind === 'body') {
            \assert($defaultValue instanceof Body);
            return $request->body ?? $defaultValue->getDefault();
        }

        if ($kind === 'request') {
            return $request;
        }

        if ($kind === 'query' || $kind === 'path' || $kind === 'header' || $kind === 'cookie') {
            if ($request->validatedParams !== null && \array_key_exists($lookupKey, $request->validatedParams)) {
                return $request->validatedParams[$lookupKey];
            }
        }

        if ($kind === 'query') {
            \assert($defaultValue instanceof Query);
            $value = $request->queryParams[$name] ?? null;
            if ($value === null) {
                return $defaultValue->getDefault();
            }
            return \count($value) === 1 ? $value[0] : $value;
        }

        if ($kind === 'path') {
            \assert($defaultValue instanceof Path);
            return $request->pathParams[$name] ?? $defaultValue->getDefault();
        }

        if ($kind === 'header') {
            \assert($defaultValue instanceof Header);
            return $request->headers[$lookupKey] ?? $defaultValue->getDefault();
        }

        if ($kind === 'cookie') {
            \assert($defaultValue instanceof Cookie);
            return $request->cookies[$name] ?? $defaultValue->getDefault();
        }

        if ($kind === 'complex') {
            if ($plan['is_optional'] ?? false) {
                return $plan['has_default'] ? $defaultValue : null;
            }
            $typeName = \is_string($plan['type'] ?? null) ? $plan['type'] : 'unknown';
            throw new RuntimeException(
                "Cannot resolve parameter '{$name}' of type '{$typeName}'"
            );
        }

        // Check if it's a path parameter by name
        if (isset($request->pathParams[$name])) {
            return $request->pathParams[$name];
        }

        if ($request->validatedParams !== null && \array_key_exists($lookupKey, $request->validatedParams)) {
            return $request->validatedParams[$lookupKey];
        }

        // Check if it's a query parameter by name
        if (isset($request->queryParams[$name])) {
            $value = $request->queryParams[$name];
            return \count($value) === 1 ? $value[0] : $value;
        }

        // Check if it has a default value
        if ($plan['has_default']) {
            return $defaultValue;
        }

        // Check if it's nullable
        if ($plan['allows_null']) {
            return null;
        }

        throw new RuntimeException(
            "Cannot resolve required parameter '{$name}' for method '{$this->method->getName()}'"
        );
    }

    /**
     * @return array<int, array{name: string, kind: string, default: mixed, has_default: bool, allows_null: bool, lookup_key?: string, is_optional?: bool, type?: string}>
     */
    private function buildParameterPlan(ReflectionMethod $method): array
    {
        $plan = [];

        foreach ($method->getParameters() as $param) {
            $name = $param->getName();
            $hasDefault = $param->isDefaultValueAvailable();
            $defaultValue = $hasDefault ? $param->getDefaultValue() : null;

            if ($defaultValue instanceof Body) {
                $plan[] = [
                    'name' => $name,
                    'kind' => 'body',
                    'default' => $defaultValue,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                ];
                continue;
            }

            if ($defaultValue instanceof Query) {
                $plan[] = [
                    'name' => $name,
                    'kind' => 'query',
                    'default' => $defaultValue,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                ];
                continue;
            }

            if ($defaultValue instanceof Path) {
                $plan[] = [
                    'name' => $name,
                    'kind' => 'path',
                    'default' => $defaultValue,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                ];
                continue;
            }

            if ($defaultValue instanceof Header) {
                $lookupKey = $defaultValue->getAlias() ?? $name;
                if ($defaultValue->shouldConvertUnderscores()) {
                    $lookupKey = \str_replace('_', '-', $lookupKey);
                }
                $lookupKey = \strtolower($lookupKey);
                $plan[] = [
                    'name' => $name,
                    'kind' => 'header',
                    'default' => $defaultValue,
                    'lookup_key' => $lookupKey,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                ];
                continue;
            }

            if ($defaultValue instanceof Cookie) {
                $plan[] = [
                    'name' => $name,
                    'kind' => 'cookie',
                    'default' => $defaultValue,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                ];
                continue;
            }

            $paramType = $param->getType();
            if ($paramType instanceof ReflectionNamedType && !$paramType->isBuiltin()) {
                if ($paramType->getName() === Request::class) {
                    $plan[] = [
                        'name' => $name,
                        'kind' => 'request',
                        'default' => null,
                        'has_default' => false,
                        'allows_null' => false,
                    ];
                    continue;
                }

                $plan[] = [
                    'name' => $name,
                    'kind' => 'complex',
                    'default' => $defaultValue,
                    'has_default' => $hasDefault,
                    'allows_null' => $param->allowsNull(),
                    'is_optional' => $param->isOptional(),
                    'type' => $paramType->getName(),
                ];
                continue;
            }

            $plan[] = [
                'name' => $name,
                'kind' => 'basic',
                'default' => $defaultValue,
                'has_default' => $hasDefault,
                'allows_null' => $param->allowsNull(),
            ];
        }

        return $plan;
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
