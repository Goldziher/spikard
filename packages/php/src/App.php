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
     * Start the server using the configured bindings.
     *
     * For now this is a placeholder until the ext-php-rs integration is wired.
     */
    public function run(?ServerConfig $config = null): void
    {
        $configToUse = $config ?? $this->config;
        if ($configToUse === null) {
            throw new RuntimeException('ServerConfig is required to run the Spikard server.');
        }

        throw new RuntimeException('PHP runtime bindings are not implemented yet.');
    }

    /**
     * Convenience entry point for single-route applications.
     */
    public static function singleRoute(string $method, string $path, HandlerInterface $handler): self
    {
        return (new self())->addRoute($method, $path, $handler);
    }
}
