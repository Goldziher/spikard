<?php

declare(strict_types=1);

namespace Spikard;

use ReflectionClass;
use RuntimeException;
use Spikard\Attributes\Middleware;
use Spikard\Attributes\Route;
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
    private ?ServerConfig $config;
    private ?LifecycleHooks $hooks = null;
    private ?DependencyContainer $dependencies = null;
    private ?int $serverHandle = null;

    /** @var array<int, array{method: string, path: string, handler: HandlerInterface, request_schema?: array<mixed>|null, response_schema?: array<mixed>|null, parameter_schema?: array<mixed>|null}> */
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
     * Register an HTTP route with a handler.
     */
    public function addRoute(string $method, string $path, HandlerInterface $handler): self
    {
        $clone = clone $this;
        $clone->routes[] = [
            'method' => $method,
            'path' => $path,
            'handler' => $handler,
        ];
        return $clone;
    }

    /**
     * Register an HTTP route with JSON schemas (request/response/parameters).
     * Schemas must already match the fixture schema.json shapes.
     *
     * @param array<string,mixed>|null $requestSchema
     * @param array<string,mixed>|null $responseSchema
     * @param array<string,mixed>|null $parameterSchema
     */
    public function addRouteWithSchemas(
        string $method,
        string $path,
        HandlerInterface $handler,
        ?array $requestSchema,
        ?array $responseSchema,
        ?array $parameterSchema
    ): self {
        $clone = clone $this;
        $clone->routes[] = [
            'method' => $method,
            'path' => $path,
            'handler' => $handler,
            'request_schema' => $requestSchema,
            'response_schema' => $responseSchema,
            'parameter_schema' => $parameterSchema,
        ];
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
     *     public function create(#[Body] array $data): array {
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

            // Register the route
            if ($routeAttr->requestSchema !== null || $routeAttr->responseSchema !== null || $routeAttr->parameterSchema !== null) {
                $clone = $clone->addRouteWithSchemas(
                    $routeAttr->method,
                    $routeAttr->path,
                    $handler,
                    $routeAttr->requestSchema,
                    $routeAttr->responseSchema,
                    $routeAttr->parameterSchema
                );
            } else {
                $clone = $clone->addRoute(
                    $routeAttr->method,
                    $routeAttr->path,
                    $handler
                );
            }
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
     * @return array<int, array{method: string, path: string, handler_name: string, handler?: object, websocket?: bool, sse?: bool}>
     */
    public function nativeRoutes(): array
    {
        $routes = [];
        foreach ($this->routes as $route) {
            $routes[] = [
                'method' => \strtoupper($route['method']),
                'path' => $route['path'],
                'handler_name' => \spl_object_hash($route['handler']),
                'handler' => $route['handler'],
            ];
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
        $lifecyclePayload = $this->hooks ? $this->hooksToNative($this->hooks) : [];
        $dependenciesPayload = $this->dependencies ?? null;

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
     * Convenience entry point for single-route applications.
     */
    public static function singleRoute(string $method, string $path, HandlerInterface $handler): self
    {
        return (new self())->addRoute($method, $path, $handler);
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
}
