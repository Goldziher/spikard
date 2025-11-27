<?php

declare(strict_types=1);

namespace Spikard;

use RuntimeException;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
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

    /** @var array<int, array{method: string, path: string, handler: HandlerInterface}> */
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

    /** @return array<int, array{method: string, path: string, handler: HandlerInterface}> */
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
            if (\strtoupper($route['method']) === $needleMethod && $route['path'] === $path) {
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
     * @return array<int, array{method: string, path: string, handler?: object, websocket?: bool, sse?: bool}>
     */
    public function nativeRoutes(): array
    {
        $routes = [];
        foreach ($this->routes as $route) {
            $routes[] = [
                'method' => \strtoupper($route['method']),
                'path' => $route['path'],
                'handler' => $route['handler'],
            ];
        }
        foreach ($this->websocketHandlers as $path => $handler) {
            $routes[] = [
                'method' => 'GET',
                'path' => $path,
                'handler' => $handler,
                'websocket' => true,
            ];
        }
        foreach ($this->sseProducers as $path => $producer) {
            $routes[] = [
                'method' => 'GET',
                'path' => $path,
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

    /** Start the server using the native extension (background). */
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

        // Extension entrypoint is guaranteed by the guard above; call directly.
        $routes = $this->nativeRoutes();
        // Ensure handler key exists for ws/sse entries to satisfy native expectations.
        $normalizedRoutes = \array_map(
            static fn (array $route) => $route + ['handler' => $route['handler'] ?? new \stdClass()],
            $routes
        );
        /** @var int $handle */
        $handle = spikard_start_server($normalizedRoutes, $configPayload, $lifecyclePayload);
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
     * @return array<string, mixed>
     */
    private function configToNative(ServerConfig $config): array
    {
        $payload = [];

        if ($config->compression !== null) {
            $payload['compression'] = [
                'gzip' => $config->compression->enabled,
                'brotli' => $config->compression->enabled,
                'minSize' => 1024,
                'quality' => $config->compression->quality,
            ];
        }

        if ($config->rateLimit !== null) {
            $payload['rateLimit'] = [
                'perSecond' => $config->rateLimit->refill,
                'burst' => $config->rateLimit->burst,
                'ipBased' => true,
            ];
        }

        if ($config->cors !== null && $config->cors->enabled) {
            $payload['cors'] = [
                'allowOrigins' => $config->cors->allowedOrigins,
                'allowMethods' => $config->cors->allowedMethods,
                'allowHeaders' => $config->cors->allowedHeaders,
                'exposeHeaders' => $config->cors->exposedHeaders,
                'allowCredentials' => $config->cors->allowCredentials,
                'maxAge' => $config->cors->maxAgeSeconds,
            ];
        }

        if ($config->staticFiles !== null && $config->staticFiles->enabled && $config->staticFiles->root !== null) {
            $payload['staticFiles'] = [[
                'directory' => $config->staticFiles->root,
                'routePrefix' => '/',
                'indexFile' => $config->staticFiles->indexFile !== null,
                'cacheControl' => $config->staticFiles->cache ? 'public, max-age=3600' : null,
            ]];
        }

        return $payload;
    }

    /**
     * @return array<string, callable>
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
        if ($hooks->onError !== null) {
            $payload['onError'] = $hooks->onError;
        }
        if ($hooks->onResponse !== null) {
            $payload['onResponse'] = $hooks->onResponse;
        }
        return $payload;
    }
}
