<?php

declare(strict_types=1);

namespace Spikard\Grpc;

use InvalidArgumentException;
use RuntimeException;

/**
 * Registry and manager for gRPC service handlers.
 *
 * Manages the registration and lookup of gRPC handlers by service name.
 * A single Service instance can manage multiple gRPC services, each with
 * its own handler implementation.
 *
 * @example
 * ```php
 * $service = new Service();
 * $service->registerHandler('mypackage.UserService', new UserServiceHandler());
 * $service->registerHandler('mypackage.PostService', new PostServiceHandler());
 *
 * // Retrieve handlers for routing
 * $handler = $service->getHandler('mypackage.UserService');
 * $response = $handler->handleRequest($request);
 * ```
 */
final class Service
{
    /**
     * Map of fully qualified service names to handler instances.
     *
     * @var array<string, HandlerInterface>
     */
    private array $handlers = [];

    /**
     * Register a gRPC handler for a service.
     *
     * @param string $serviceName Fully qualified service name (e.g., "mypackage.MyService")
     * @param HandlerInterface $handler The handler implementation for this service
     *
     * @throws InvalidArgumentException if the service name is invalid
     *
     * @return $this For method chaining
     */
    public function registerHandler(string $serviceName, HandlerInterface $handler): self
    {
        if (empty($serviceName)) {
            throw new InvalidArgumentException('Service name cannot be empty');
        }

        // Validate service name format (must contain a dot for fully qualified name)
        if (strpos($serviceName, '.') === false) {
            throw new InvalidArgumentException(
                sprintf(
                    'Service name "%s" must be fully qualified (contain a dot)',
                    $serviceName
                )
            );
        }

        $this->handlers[$serviceName] = $handler;
        return $this;
    }

    /**
     * Get a handler by service name.
     *
     * @param string $serviceName The fully qualified service name
     *
     * @return HandlerInterface|null The handler, or null if not registered
     */
    public function getHandler(string $serviceName): ?HandlerInterface
    {
        return $this->handlers[$serviceName] ?? null;
    }

    /**
     * Check if a service is registered.
     *
     * @param string $serviceName The fully qualified service name
     *
     * @return bool True if registered, false otherwise
     */
    public function hasHandler(string $serviceName): bool
    {
        return isset($this->handlers[$serviceName]);
    }

    /**
     * Get all registered service names.
     *
     * @return list<string> Array of registered service names
     */
    public function getServiceNames(): array
    {
        return array_keys($this->handlers);
    }

    /**
     * Get the number of registered services.
     *
     * @return int Number of registered handlers
     */
    public function getHandlerCount(): int
    {
        return count($this->handlers);
    }

    /**
     * Clear all registered handlers.
     *
     * @return $this For method chaining
     */
    public function clear(): self
    {
        $this->handlers = [];
        return $this;
    }

    /**
     * Get all registered handlers as an associative array.
     *
     * @return array<string, HandlerInterface>
     */
    public function getAllHandlers(): array
    {
        return $this->handlers;
    }

    /**
     * Handle a gRPC request by routing to the appropriate handler.
     *
     * @param Request $request The incoming gRPC request
     *
     * @return Response The response from the handler
     *
     * @throws RuntimeException if no handler is registered for the service
     */
    public function handleRequest(Request $request): Response
    {
        $handler = $this->getHandler($request->serviceName);
        if ($handler === null) {
            throw new RuntimeException(
                sprintf(
                    'No gRPC handler registered for service "%s"',
                    $request->serviceName
                )
            );
        }

        return $handler->handleRequest($request);
    }
}
