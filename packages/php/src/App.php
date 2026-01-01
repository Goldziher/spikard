<?php

declare(strict_types=1);

namespace Spikard;

use ReflectionClass;
use ReflectionMethod;
use ReflectionNamedType;
use RuntimeException;
use Spikard\Attributes\JsonRpcMethod;
use Spikard\Attributes\Middleware;
use Spikard\Attributes\Route;
use Spikard\Attributes\SchemaRef;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\Handlers\ControllerMethodHandler;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Native\TestClient as NativeClient;

/**
 * Spikard application facade for PHP bindings.
 *
 * This mirrors the API shape of other bindings; all runtime logic will be
 * implemented in Rust via ext-php-rs.
 */
final class App
{
    /** @var array<string, array{request: array<mixed>|null, parameter: array<mixed>|null}> */
    private static array $methodSchemaCache = [];

    private ?ServerConfig $config;
    private ?LifecycleHooks $hooks = null;
    private ?DependencyContainer $dependencies = null;
    private ?int $serverHandle = null;
    /** @var array<string, array<mixed>>|null */
    private ?array $requestSchemas = null;
    /** @var array<string, array<mixed>>|null */
    private ?array $responseSchemas = null;
    /** @var array<string, array<mixed>>|null */
    private ?array $parameterSchemas = null;

    /** @var array<int, array{method: string, path: string, handler: HandlerInterface, request_schema?: array<mixed>|null, response_schema?: array<mixed>|null, parameter_schema?: array<mixed>|null, jsonrpc_method?: \Spikard\Http\JsonRpcMethodInfo|null}> */
    private array $routes = [];

    /** @var array<string, WebSocketHandlerInterface> */
    private array $websocketHandlers = [];

    /** @var array<string, SseEventProducerInterface> */
    private array $sseProducers = [];

    public function __construct(?ServerConfig $config = null)
    {
        $this->config = $config;
    }

    public function withConfig(ServerConfig $config): self
    {
        $clone = clone $this;
        $clone->config = $config;
        return $clone;
    }

    public function withLifecycleHooks(LifecycleHooks $hooks): self
    {
        $clone = clone $this;
        $clone->hooks = $hooks;
        return $clone;
    }

    public function withDependencies(DependencyContainer $container): self
    {
        $clone = clone $this;
        $clone->dependencies = $container;
        return $clone;
    }

    /**
     * Provide shared schema maps for attribute-driven routes.
     *
     * @param array<string, array<mixed>> $requestSchemas
     * @param array<string, array<mixed>> $responseSchemas
     * @param array<string, array<mixed>> $parameterSchemas
     */
    public function withSchemas(array $requestSchemas, array $responseSchemas, array $parameterSchemas): self
    {
        $clone = clone $this;
        $clone->requestSchemas = $requestSchemas;
        $clone->responseSchemas = $responseSchemas;
        $clone->parameterSchemas = $parameterSchemas;
        return $clone;
    }

    public function addWebSocket(string $path, WebSocketHandlerInterface $handler): self
    {
        $clone = clone $this;
        $clone->websocketHandlers[$path] = $handler;
        return $clone;
    }

    public function addSse(string $path, SseEventProducerInterface $producer): self
    {
        $clone = clone $this;
        $clone->sseProducers[$path] = $producer;
        return $clone;
    }

    /**
     * Register a controller class by scanning for route attributes.
     *
     * This method uses reflection to discover methods annotated with route attributes
     * (Get, Post, Put, Delete, Patch, etc.) and registers them as routes.
     *
     * Example:
     * ```php
     * use Spikard\Attributes\{Get, Post};
     * use Spikard\Http\Params\Body;
     *
     * class UserController {
     *     #[Get('/users')]
     *     public function list(): array {
     *         return ['users' => []];
     *     }
     *
     *     #[Post('/users')]
     *     public function create(array $data = new Body()): array {
     *         return ['user' => $data];
     *     }
     * }
     *
     * $app->registerController(UserController::class);
     * // or with an instance:
     * $app->registerController(new UserController($dependency));
     * ```
     *
     * @param class-string|object $controller Controller class name or instance
     * @return self New app instance with registered routes
     * @throws \ReflectionException If the class cannot be reflected
     */
    public function registerController(string|object $controller): self
    {
        $instance = \is_object($controller) ? $controller : new $controller();
        $reflection = new ReflectionClass($instance);
        $clone = clone $this;

        foreach ($reflection->getMethods() as $method) {
            // Skip non-public methods
            if (!$method->isPublic()) {
                continue;
            }

            // Find route attributes
            $routeAttributes = $method->getAttributes(Route::class, \ReflectionAttribute::IS_INSTANCEOF);
            if (\count($routeAttributes) === 0) {
                continue;
            }

            // Get the first route attribute
            $routeAttr = $routeAttributes[0]->newInstance();

            // Collect middleware from Middleware attributes
            $middlewareAttributes = $method->getAttributes(Middleware::class);
            $middleware = \array_merge(
                $routeAttr->middleware,
                \array_map(static fn ($attr) => $attr->newInstance()->middleware, $middlewareAttributes)
            );

            // Create handler wrapper
            $handler = new ControllerMethodHandler($instance, $method);

            $jsonRpcAttributes = $method->getAttributes(JsonRpcMethod::class);
            $jsonRpcMethod = null;
            if (\count($jsonRpcAttributes) > 0) {
                $jsonRpcMethod = $jsonRpcAttributes[0]->newInstance()->toMethodInfo();
            }

            $schemaRefs = $method->getAttributes(SchemaRef::class);
            $schemaRef = \count($schemaRefs) > 0 ? $schemaRefs[0]->newInstance() : null;

            /** @var array<string, mixed>|null $requestSchema */
            $requestSchema = $routeAttr->requestSchema ?? $this->resolveSchemaRef(
                $schemaRef?->request,
                $this->requestSchemas,
                'request'
            );
            /** @var array<string, mixed>|null $responseSchema */
            $responseSchema = $routeAttr->responseSchema ?? $this->resolveSchemaRef(
                $schemaRef?->response,
                $this->responseSchemas,
                'response'
            );
            /** @var array<string, mixed>|null $parameterSchema */
            $parameterSchema = $routeAttr->parameterSchema ?? $this->resolveSchemaRef(
                $schemaRef?->parameters,
                $this->parameterSchemas,
                'parameter'
            );
            if ($requestSchema === null || $parameterSchema === null) {
                $extracted = $this->extractMethodSchemas($method, $routeAttr->path);
                if ($requestSchema === null) {
                    /** @var array<string, mixed>|null $extracted_request */
                    $extracted_request = $extracted['request'] ?? null;
                    $requestSchema = $extracted_request;
                }
                if ($parameterSchema === null) {
                    /** @var array<string, mixed>|null $extracted_parameter */
                    $extracted_parameter = $extracted['parameter'] ?? null;
                    $parameterSchema = $extracted_parameter;
                }
            }

            // Register the route
            $clone = $clone->registerRoute(
                $routeAttr->method,
                $routeAttr->path,
                $handler,
                $requestSchema,
                $responseSchema,
                $parameterSchema,
                $jsonRpcMethod
            );
        }

        return $clone;
    }

    public function config(): ?ServerConfig
    {
        return $this->config;
    }

    public function lifecycleHooks(): ?LifecycleHooks
    {
        return $this->hooks;
    }

    public function dependencies(): ?DependencyContainer
    {
        return $this->dependencies;
    }

    /**
     * @return array<int, array{method: string, path: string, handler: HandlerInterface, request_schema?: array<mixed>|null, response_schema?: array<mixed>|null, parameter_schema?: array<mixed>|null}>
     */
    public function routes(): array
    {
        return $this->routes;
    }

    /**
     * Find a handler for the given request (method/path already set).
     */
    public function findHandler(Request $request): ?HandlerInterface
    {
        $needleMethod = \strtoupper($request->method);
        $path = $request->path;
        foreach ($this->routes as $route) {
            // Strip query string from registered route path for comparison
            $routePath = \explode('?', $route['path'], 2)[0];
            if (\strtoupper($route['method']) === $needleMethod && $routePath === $path) {
                if (isset($route['jsonrpc_method'])) {
                    $jsonrpcName = $route['jsonrpc_method']->methodName ?? null;
                    $bodyMethod = null;
                    if (\is_array($request->body) && \array_key_exists('method', $request->body)) {
                        $bodyMethod = $request->body['method'];
                    }
                    if (!\is_string($bodyMethod) || $jsonrpcName !== $bodyMethod) {
                        continue;
                    }
                }
                if ($route['handler']->matches($request)) {
                    return $route['handler'];
                }
            }
        }

        return null;
    }

    /**
     * Routes formatted for the native (Rust) test client.
     *
     * @return array<int, array{method: string, path: string, handler_name: string, handler?: object, jsonrpc_method?: array<string, mixed>, websocket?: bool, sse?: bool}>
     */
    public function nativeRoutes(): array
    {
        $routes = [];
        foreach ($this->routes as $route) {
            $routeData = [
                'method' => \strtoupper($route['method']),
                'path' => $route['path'],
                'handler_name' => \spl_object_hash($route['handler']),
                'handler' => $route['handler'],
            ];

            // Add JSON-RPC method info if present
            if (isset($route['jsonrpc_method'])) {
                $routeData['jsonrpc_method'] = $route['jsonrpc_method']->toArray();
            }

            // Add request/response/parameter schemas if present
            if (isset($route['request_schema'])) {
                $routeData['request_schema'] = $route['request_schema'];
            }
            if (isset($route['response_schema'])) {
                $routeData['response_schema'] = $route['response_schema'];
            }
            if (isset($route['parameter_schema'])) {
                $routeData['parameter_schema'] = $route['parameter_schema'];
            }

            $routes[] = $routeData;
        }
        foreach ($this->websocketHandlers as $path => $handler) {
            $routes[] = [
                'method' => 'GET',
                'path' => $path,
                'handler_name' => \spl_object_hash($handler),
                'handler' => $handler,
                'websocket' => true,
            ];
        }
        foreach ($this->sseProducers as $path => $producer) {
            $routes[] = [
                'method' => 'GET',
                'path' => $path,
                'handler_name' => \spl_object_hash($producer),
                'handler' => $producer,
                'sse' => true,
            ];
        }
        return $routes;
    }

    /** @return array<string, mixed> */
    public function nativeConfig(): array
    {
        $config = $this->config ?? ServerConfig::builder()->build();
        return $this->configToNative($config);
    }

    /** @return array<string, WebSocketHandlerInterface> */
    public function websocketHandlers(): array
    {
        return $this->websocketHandlers;
    }

    /** @return array<string, SseEventProducerInterface> */
    public function sseProducers(): array
    {
        return $this->sseProducers;
    }

    /**
     * Start the server using the native extension (background).
     *
     * This implementation follows Python's pattern (crates/spikard-py/src/lib.rs:287-463):
     * 1. Converts PHP ServerConfig to native array via configToNative()
     * 2. Passes array directly to spikard_start_server() FFI function
     * 3. Rust extracts fields manually using extract_server_config_from_php()
     * 4. Constructs ServerConfig struct directly, avoiding JSON deserialization
     * 5. This properly handles non-serializable fields like lifecycle_hooks
     *
     * Implementation:
     * - PHP side: packages/php/src/App.php:282-350 (configToNative)
     * - Rust side: crates/spikard-php/src/php/start.rs:48-465 (extract_server_config_from_php)
     */
    public function run(?ServerConfig $config = null): void
    {
        $configToUse = $config ?? $this->config;
        if ($configToUse === null) {
            throw new RuntimeException('ServerConfig is required to run the Spikard server.');
        }

        if (!\function_exists('spikard_version') || !\function_exists('spikard_start_server')) {
            throw new RuntimeException('Spikard PHP extension is not loaded; build with extension-module feature.');
        }

        $configPayload = $this->configToNative($configToUse);
        /** @var array<string, callable> $lifecyclePayload */
        $lifecyclePayload = $this->hooks ? $this->hooksToNative($this->hooks) : [];
        /** @var array<string, mixed> $dependenciesPayload */
        $dependenciesPayload = [
            'dependencies' => $this->dependencies ? $this->dependencies->getDependencies() : [],
        ];

        // Extension entrypoint is guaranteed by the guard above; call directly.
        $routes = $this->nativeRoutes();
        // Ensure handler key exists for ws/sse entries to satisfy native expectations.
        $normalizedRoutes = \array_map(
            static fn (array $route) => $route + ['handler' => $route['handler'] ?? new \stdClass()],
            $routes
        );
        /** @var int $handle */
        $handle = spikard_start_server($normalizedRoutes, $configPayload, $lifecyclePayload, $dependenciesPayload);
        $this->serverHandle = $handle;
    }

    /** Stop a running server (no-op if not started). */
    public function close(): void
    {
        if ($this->serverHandle !== null && \function_exists('spikard_stop_server')) {
            spikard_stop_server($this->serverHandle);
        }
        $this->serverHandle = null;
    }

    /**
     * Convert ServerConfig to the native array expected by the extension.
     *
     * Serializes PHP ServerConfig to a format matching Rust's ServerConfig struct
     * in crates/spikard-http/src/lib.rs:112-149.
     *
     * IMPORTANT: All field names MUST use snake_case (Rust serde default).
     * Reference: Python implementation in crates/spikard-py/src/lib.rs:287-463
     *
     * @return array<string, mixed>
     */
    private function configToNative(ServerConfig $config): array
    {
        // Basic server settings (using actual config values)
        $payload = [
            'host' => $config->host,
            'port' => $config->port,
            'workers' => $config->workers,
            'enable_request_id' => $config->enableRequestId,
            'max_body_size' => $config->maxBodySize,
            'request_timeout' => $config->requestTimeout,
            'graceful_shutdown' => $config->gracefulShutdown,
            'shutdown_timeout' => $config->shutdownTimeout,
        ];

        // Compression middleware (snake_case field names)
        if ($config->compression !== null) {
            $payload['compression'] = [
                'gzip' => $config->compression->gzip ?? true,
                'brotli' => $config->compression->brotli ?? true,
                'min_size' => $config->compression->minSize ?? 1024,
                'quality' => $config->compression->quality ?? 6,
            ];
        }

        // Rate limiting middleware (snake_case field names)
        if ($config->rateLimit !== null) {
            $payload['rate_limit'] = [
                'per_second' => $config->rateLimit->perSecond,
                'burst' => $config->rateLimit->burst,
                'ip_based' => $config->rateLimit->ipBased ?? true,
            ];
        }

        // JWT authentication (snake_case field names)
        if ($config->jwtAuth !== null) {
            $payload['jwt_auth'] = [
                'secret' => $config->jwtAuth->secret,
                'algorithm' => $config->jwtAuth->algorithm,
                'audience' => $config->jwtAuth->audience,
                'issuer' => $config->jwtAuth->issuer,
                'leeway' => $config->jwtAuth->leeway,
            ];
        }

        // API Key authentication (snake_case field names)
        if ($config->apiKeyAuth !== null) {
            $payload['api_key_auth'] = [
                'keys' => $config->apiKeyAuth->keys,
                'header_name' => $config->apiKeyAuth->headerName,
            ];
        }

        // CORS middleware (snake_case field names)
        // Maps to spikard_core::http::CorsConfig
        if ($config->cors !== null && $config->cors->enabled) {
            $payload['cors'] = [
                'allowed_origins' => $config->cors->allowedOrigins,
                'allowed_methods' => $config->cors->allowedMethods,
                'allowed_headers' => $config->cors->allowedHeaders,
                'expose_headers' => $config->cors->exposedHeaders,
                'max_age' => $config->cors->maxAgeSeconds,
                'allow_credentials' => $config->cors->allowCredentials,
            ];
        }

        // Static files serving (snake_case field names)
        // Maps to Vec<StaticFilesConfig> in Rust
        if ($config->staticFiles !== null && $config->staticFiles->enabled && $config->staticFiles->root !== null) {
            $payload['static_files'] = [[
                'directory' => $config->staticFiles->root,
                'route_prefix' => '/',
                'index_file' => $config->staticFiles->indexFile !== null,
                'cache_control' => $config->staticFiles->cache ? 'public, max-age=3600' : null,
            ]];
        } else {
            $payload['static_files'] = [];
        }

        // OpenAPI configuration (snake_case field names)
        if ($config->openapi !== null) {
            $payload['openapi'] = [
                'enabled' => $config->openapi->enabled,
                'title' => $config->openapi->title,
                'version' => $config->openapi->version,
                'description' => $config->openapi->description,
                'swagger_ui_path' => $config->openapi->swaggerUiPath,
                'redoc_path' => $config->openapi->redocPath,
                'openapi_json_path' => $config->openapi->openapiJsonPath,
            ];
        }

        // Background tasks configuration uses default
        // (BackgroundTaskConfig::default() in Rust)

        return $payload;
    }

    /**
     * Convert LifecycleHooks to native format.
     *
     * Lifecycle hooks are passed as PHP callables to the Rust extension.
     * The Rust side stores them as Zval in thread_local! registries and
     * reconstructs ZendCallable when invoking hooks. See:
     * crates/spikard-php/src/php/hooks.rs
     *
     * @return array<string, mixed>
     */
    private function hooksToNative(LifecycleHooks $hooks): array
    {
        $payload = [];

        if ($hooks->onRequest !== null) {
            $payload['onRequest'] = $hooks->onRequest;
        }

        if ($hooks->preValidation !== null) {
            $payload['preValidation'] = $hooks->preValidation;
        }

        if ($hooks->preHandler !== null) {
            $payload['preHandler'] = $hooks->preHandler;
        }

        if ($hooks->onResponse !== null) {
            $payload['onResponse'] = $hooks->onResponse;
        }

        if ($hooks->onError !== null) {
            $payload['onError'] = $hooks->onError;
        }

        return $payload;
    }

    /**
     * Register an HTTP route discovered from controller attributes.
     *
     * @param array<string, mixed>|null $requestSchema
     * @param array<string, mixed>|null $responseSchema
     * @param array<string, mixed>|null $parameterSchema
     */
    private function registerRoute(
        string $method,
        string $path,
        HandlerInterface $handler,
        ?array $requestSchema,
        ?array $responseSchema,
        ?array $parameterSchema,
        ?\Spikard\Http\JsonRpcMethodInfo $jsonRpcMethod
    ): self {
        $clone = clone $this;
        $pathOnly = \explode('?', $path, 2)[0];
        $route = [
            'method' => $method,
            'path' => $pathOnly,
            'handler' => $handler,
        ];

        if ($requestSchema !== null) {
            $route['request_schema'] = $requestSchema;
        }
        if ($responseSchema !== null) {
            $route['response_schema'] = $responseSchema;
        }
        if ($parameterSchema !== null) {
            $route['parameter_schema'] = $parameterSchema;
        }
        if ($jsonRpcMethod !== null) {
            $route['jsonrpc_method'] = $jsonRpcMethod;
        }

        $clone->routes[] = $route;
        return $clone;
    }

    /**
     * @param array<string, array<mixed>>|null $schemas
     * @return array<mixed>|null
     */
    private function resolveSchemaRef(?string $key, ?array $schemas, string $label): ?array
    {
        if ($key === null) {
            return null;
        }

        if ($schemas === null) {
            throw new RuntimeException(\sprintf('Missing %s schema registry; cannot resolve "%s".', $label, $key));
        }

        if (!\array_key_exists($key, $schemas)) {
            throw new RuntimeException(\sprintf('Missing %s schema for key: %s', $label, $key));
        }

        return $schemas[$key];
    }

    /**
     * @return array{request: array<string, mixed>|null, parameter: array<string, mixed>|null}
     */
    private function extractMethodSchemas(ReflectionMethod $method, string $path): array
    {
        $cacheKey = $method->getDeclaringClass()->getName() . '::' . $method->getName() . '|' . $path;
        if (isset(self::$methodSchemaCache[$cacheKey])) {
            /** @var array{request: array<string, mixed>|null, parameter: array<string, mixed>|null} $cached */
            $cached = self::$methodSchemaCache[$cacheKey];
            return $cached;
        }

        $pathParams = $this->extractPathParams($path);
        $properties = [];
        $required = [];
        $requestSchema = null;

        foreach ($method->getParameters() as $param) {
            $name = $param->getName();
            if ($name === 'request' || $name === 'req') {
                continue;
            }

            $hasDefault = $param->isDefaultValueAvailable();
            $defaultValue = $hasDefault ? $param->getDefaultValue() : null;
            $isOptional = $param->isOptional() || $param->allowsNull();

            if ($defaultValue instanceof \Spikard\Http\Params\Body) {
                if ($requestSchema === null) {
                    $requestSchema = $defaultValue->getSchema();
                }
                continue;
            }

            $source = null;
            $schema = null;

            if ($defaultValue instanceof \Spikard\Http\Params\Query) {
                $source = 'query';
                $schema = $defaultValue->getSchema();
            } elseif ($defaultValue instanceof \Spikard\Http\Params\Path) {
                $source = 'path';
                $schema = $defaultValue->getSchema();
            } elseif ($defaultValue instanceof \Spikard\Http\Params\Header) {
                $source = 'header';
                $schema = $defaultValue->getSchema();
                $name = $this->normalizeHeaderKey($name, $defaultValue);
            } elseif ($defaultValue instanceof \Spikard\Http\Params\Cookie) {
                $source = 'cookie';
                $schema = $defaultValue->getSchema();
            } elseif (\in_array($name, $pathParams, true)) {
                $source = 'path';
            } else {
                $source = 'query';
            }

            $typeSchema = $this->schemaForType($param->getType());
            if ($schema === null) {
                $schema = $typeSchema;
            } elseif ($typeSchema !== null) {
                foreach ($typeSchema as $key => $value) {
                    if (!\array_key_exists($key, $schema)) {
                        $schema[$key] = $value;
                    }
                }
            }

            if ($schema === null) {
                $schema = [];
            }

            $schema['source'] = $source;
            if ($isOptional || ($defaultValue instanceof \Spikard\Http\Params\ParamBase && $defaultValue->hasDefault())) {
                $schema['optional'] = true;
            }

            $properties[$name] = $schema;
            if (!$isOptional && !($defaultValue instanceof \Spikard\Http\Params\ParamBase && $defaultValue->hasDefault())) {
                $required[] = $name;
            }
        }

        $parameterSchema = null;
        if (!empty($properties)) {
            $parameterSchema = [
                'type' => 'object',
                'properties' => $properties,
            ];
            if (!empty($required)) {
                $parameterSchema['required'] = $required;
            }
        }

        /**
         * @var array{request: array<string, mixed>|null, parameter: array<string, mixed>|null}
         */
        $result = [
            'request' => $requestSchema,
            'parameter' => $parameterSchema,
        ];

        self::$methodSchemaCache[$cacheKey] = $result;
        return $result;
    }

    /**
     * @return array<int, string>
     */
    private function extractPathParams(string $path): array
    {
        $params = [];
        if (\preg_match_all('/\\{([A-Za-z_][A-Za-z0-9_]*)(?::[^}]+)?\\}/', $path, $matches)) {
            $params = \array_merge($params, $matches[1]);
        }
        if (\preg_match_all('/:([A-Za-z_][A-Za-z0-9_]*)/', $path, $matches)) {
            $params = \array_merge($params, $matches[1]);
        }
        return \array_values(\array_unique($params));
    }

    /**
     * @return array<string, mixed>|null
     */
    private function schemaForType(?\ReflectionType $type): ?array
    {
        if ($type instanceof \ReflectionUnionType) {
            $schemas = [];
            foreach ($type->getTypes() as $inner) {
                if ($inner instanceof ReflectionNamedType && $inner->getName() === 'null') {
                    continue;
                }
                if (!$inner instanceof ReflectionNamedType || !$inner->isBuiltin()) {
                    return null;
                }
                $innerSchema = $this->schemaForType($inner);
                if ($innerSchema === null) {
                    return null;
                }
                $schemas[] = $innerSchema;
            }
            if (empty($schemas)) {
                return null;
            }
            if (\count($schemas) == 1) {
                return $schemas[0];
            }
            return ['anyOf' => $schemas];
        }

        if (!$type instanceof ReflectionNamedType || !$type->isBuiltin()) {
            return null;
        }

        return match ($type->getName()) {
            'int' => ['type' => 'integer'],
            'float' => ['type' => 'number'],
            'string' => ['type' => 'string'],
            'bool' => ['type' => 'boolean'],
            'array' => ['type' => 'array'],
            default => null,
        };
    }

    private function normalizeHeaderKey(string $name, \Spikard\Http\Params\Header $header): string
    {
        $key = $header->getAlias() ?? $name;
        if ($header->shouldConvertUnderscores()) {
            $key = \str_replace('_', '-', $key);
        }
        return \strtolower($key);
    }
}
