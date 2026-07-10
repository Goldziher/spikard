<?php

declare(strict_types=1);

use Spikard\Php\RouteBuilder;

/**
 * Ergonomic typed-handler application layer for Spikard.
 *
 * Derives JSON Schema from PHP 8.2+ readonly-class DTOs and delegates
 * request validation to the Rust core (422 ProblemDetails on invalid body).
 * Handlers receive hydrated DTO instances — never validate in PHP.
 *
 * Include this file explicitly alongside the spikard_php extension.
 * Do NOT include it together with Service.php — both define a global App class.
 *
 * Usage:
 *   $app = new App();
 *   $app->post('/users', CreateUser::class, function (CreateUser $user) {
 *       return $user;  // serialized as JSON response body
 *   });
 *   $app->run();  // binds 127.0.0.1:8000
 */
final class App
{
    /** @var list<array{string, list<RouteBuilder>, callable}> */
    private array $registrations = [];

    public function __construct() {}

    // =========================================================================
    // HTTP verb methods
    // =========================================================================

    /**
     * Register a GET route.
     */
    public function get(string $path, callable $handler): self
    {
        return $this->addRoute('Get', $path, null, $handler);
    }

    /**
     * Register a POST route with an optional body DTO class.
     *
     * @param class-string|null $dtoClass  PHP 8.2+ readonly class for body hydration.
     *                                     Pass null for routes without a request body.
     */
    public function post(string $path, ?string $dtoClass, callable $handler): self
    {
        return $this->addRoute('Post', $path, $dtoClass, $handler);
    }

    /**
     * Register a PUT route with an optional body DTO class.
     *
     * @param class-string|null $dtoClass
     */
    public function put(string $path, ?string $dtoClass, callable $handler): self
    {
        return $this->addRoute('Put', $path, $dtoClass, $handler);
    }

    /**
     * Register a PATCH route with an optional body DTO class.
     *
     * @param class-string|null $dtoClass
     */
    public function patch(string $path, ?string $dtoClass, callable $handler): self
    {
        return $this->addRoute('Patch', $path, $dtoClass, $handler);
    }

    /**
     * Register a DELETE route.
     */
    public function delete(string $path, callable $handler): self
    {
        return $this->addRoute('Delete', $path, null, $handler);
    }

    /**
     * Register a HEAD route.
     */
    public function head(string $path, callable $handler): self
    {
        return $this->addRoute('Head', $path, null, $handler);
    }

    /**
     * Register an OPTIONS route.
     */
    public function options(string $path, callable $handler): self
    {
        return $this->addRoute('Options', $path, null, $handler);
    }

    /**
     * Register a route using a pre-built RouteBuilder.
     *
     * Low-level escape hatch for advanced configuration (schemas, CORS, etc.).
     * Use the verb methods (post, get, …) for the ergonomic path.
     *
     * @param class-string|null $dtoClass
     */
    public function route(RouteBuilder $builder, ?string $dtoClass, callable $handler): self
    {
        if ($dtoClass !== null) {
            $schema = self::deriveJsonSchema($dtoClass);
            $builder = $builder->requestSchemaJson((string) json_encode($schema, JSON_THROW_ON_ERROR));
        }
        $adapter = self::makeAdapter($dtoClass, $handler);
        $this->registrations[] = ['route', [$builder], $adapter];
        return $this;
    }

    /**
     * Run the HTTP server (binds 127.0.0.1:8000 by default).
     */
    public function run(): void
    {
        app_run($this->registrations);
    }

    // =========================================================================
    // Private helpers
    // =========================================================================

    /**
     * Build a RouteBuilder for the given method + path, attach the JSON Schema
     * derived from $dtoClass (if any), wrap the handler in a request adapter,
     * and append the registration tuple.
     *
     * @param class-string|null $dtoClass
     */
    private function addRoute(string $method, string $path, ?string $dtoClass, callable $handler): self
    {
        $builder = RouteBuilder::new($method, $path);

        if ($dtoClass !== null) {
            $schema = self::deriveJsonSchema($dtoClass);
            $builder = $builder->requestSchemaJson((string) json_encode($schema, JSON_THROW_ON_ERROR));
        }

        $adapter = self::makeAdapter($dtoClass, $handler);
        $this->registrations[] = ['route', [$builder], $adapter];

        return $this;
    }

    /**
     * Derive a JSON Schema object from a PHP 8.2+ readonly-class constructor.
     *
     * Reflects on the constructor parameters and maps PHP scalar types to
     * JSON Schema types:
     *   string  → "string"
     *   int     → "integer"
     *   float   → "number"
     *   bool    → "boolean"
     *   array   → "array"
     *   other   → "string"
     *
     * Non-nullable, non-optional parameters without default values are added
     * to "required".  The Rust core validates the schema — do not re-validate
     * in PHP.
     *
     * @param class-string $dtoClass
     * @return array{type: string, properties: array<string, array{type: string}>|\stdClass, required: list<string>}
     */
    private static function deriveJsonSchema(string $dtoClass): array
    {
        $ref = new \ReflectionClass($dtoClass);
        $ctor = $ref->getConstructor();

        if ($ctor === null) {
            return ['type' => 'object', 'properties' => new \stdClass(), 'required' => []];
        }

        $properties = [];
        $required = [];

        foreach ($ctor->getParameters() as $param) {
            $name = $param->getName();
            $type = $param->getType();

            if ($type instanceof \ReflectionNamedType) {
                $jsonType = self::phpTypeToJsonSchemaType($type->getName());
                if (!$type->allowsNull() && !$param->isOptional() && !$param->isDefaultValueAvailable()) {
                    $required[] = $name;
                }
            } else {
                $jsonType = 'string';
            }

            $properties[$name] = ['type' => $jsonType];
        }

        return [
            'type' => 'object',
            'properties' => $properties !== [] ? $properties : new \stdClass(),
            'required' => $required,
        ];
    }

    /**
     * Map a PHP scalar type name to its JSON Schema type string.
     */
    private static function phpTypeToJsonSchemaType(string $phpType): string
    {
        return match ($phpType) {
            'int' => 'integer',
            'float' => 'number',
            'bool' => 'boolean',
            'array' => 'array',
            default => 'string',
        };
    }

    /**
     * Build the request-adapter callable that bridges Rust RequestData → DTO → handler
     * → response envelope.
     *
     * The Rust core calls the adapter with the request data serialised from
     * RequestData JSON (arrives as a PHP array).  The adapter:
     *   1. Extracts the "body" key from the request data.
     *   2. Hydrates a DTO instance from the body array (if $dtoClass is set).
     *   3. Calls the user handler with the hydrated DTO (or no arguments for bodyless routes).
     *   4. Wraps the result in the wire response envelope expected by spikard::Response.
     *
     * @param class-string|null $dtoClass
     */
    private static function makeAdapter(?string $dtoClass, callable $handler): callable
    {
        return static function (mixed $requestData) use ($dtoClass, $handler): mixed {
            /** @var array<string, mixed> $data */
            $data = match (true) {
                is_array($requestData) => $requestData,
                is_object($requestData) => (array) $requestData,
                default => [],
            };

            $args = [];

            if ($dtoClass !== null) {
                $body = $data['body'] ?? [];
                if (is_string($body)) {
                    /** @var array<string, mixed>|null $decoded */
                    $decoded = json_decode($body, true);
                    $body = is_array($decoded) ? $decoded : [];
                }
                /** @var array<string, mixed> $bodyArr */
                $bodyArr = is_array($body) ? $body : (array) $body;
                $args[] = self::hydrateDto($dtoClass, $bodyArr);
            }

            $result = $handler(...$args);

            return self::toEnvelope($result);
        };
    }

    /**
     * Hydrate a DTO class from an associative array, coercing scalar types.
     *
     * @param class-string           $dtoClass
     * @param array<string, mixed>   $data
     */
    private static function hydrateDto(string $dtoClass, array $data): object
    {
        $ref = new \ReflectionClass($dtoClass);
        $ctor = $ref->getConstructor();

        if ($ctor === null) {
            /** @var object */
            return new $dtoClass();
        }

        $args = [];
        foreach ($ctor->getParameters() as $param) {
            $name = $param->getName();

            if (array_key_exists($name, $data)) {
                $type = $param->getType();
                $args[] = $type instanceof \ReflectionNamedType
                    ? self::coerce($data[$name], $type->getName())
                    : $data[$name];
            } elseif ($param->isOptional() || $param->isDefaultValueAvailable()) {
                $args[] = $param->getDefaultValue();
            } else {
                $args[] = null;
            }
        }

        /** @var object */
        return new $dtoClass(...$args);
    }

    /**
     * Coerce a scalar value to the target PHP type.
     */
    private static function coerce(mixed $value, string $type): mixed
    {
        return match ($type) {
            'int' => (int) $value,
            'float' => (float) $value,
            'bool' => (bool) $value,
            'string' => (string) $value,
            default => $value,
        };
    }

    /**
     * Wrap a handler return value in the spikard::Response wire envelope.
     *
     * The `headers` field MUST serialize to a JSON object (not array).
     * Using \stdClass() ensures json_encode / ext-php-rs Zval::Serialize
     * produces {} rather than [], which serde_json can deserialise as
     * HashMap<String, String>.
     *
     * @return array{status_code: int, content: mixed, headers: \stdClass}
     */
    private static function toEnvelope(mixed $result): array
    {
        if ($result === null) {
            return ['status_code' => 200, 'content' => null, 'headers' => new \stdClass()];
        }

        // Already a valid response envelope — pass through.
        if (is_array($result) && isset($result['status_code'], $result['content'], $result['headers'])) {
            return $result;
        }

        // Convert objects (e.g. readonly DTOs) to plain arrays so serde_json
        // can round-trip them as serde_json::Value::Object.
        if (is_object($result)) {
            /** @var array<string, mixed>|null $content */
            $content = json_decode((string) json_encode($result, JSON_THROW_ON_ERROR), true);
        } else {
            $content = $result;
        }

        return ['status_code' => 200, 'content' => $content, 'headers' => new \stdClass()];
    }
}
