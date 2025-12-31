<?php

namespace Spikard\Grpc;

/**
 * Stub for IDE support - gRPC service registry.
 *
 * @see \Spikard\Grpc\Service
 */
final class Service
{
    public function registerHandler(string $serviceName, HandlerInterface $handler): self;

    public function getHandler(string $serviceName): ?HandlerInterface;

    public function hasHandler(string $serviceName): bool;

    /** @return list<string> */
    public function getServiceNames(): array;

    public function getHandlerCount(): int;

    public function clear(): self;

    /** @return array<string, HandlerInterface> */
    public function getAllHandlers(): array;

    public function handleRequest(Request $request): Response;
}
