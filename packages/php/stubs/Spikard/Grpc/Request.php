<?php

namespace Spikard\Grpc;

/**
 * Stub for IDE support - gRPC request class.
 *
 * @see \Spikard\Grpc\Request
 */
final class Request
{
    public readonly string $serviceName;
    public readonly string $methodName;
    public readonly string $payload;
    /** @var array<string, string> */
    public readonly array $metadata;

    /**
     * @param array<string, string> $metadata
     */
    public function __construct(
        string $serviceName,
        string $methodName,
        string $payload,
        array $metadata = []
    );

    public function getMetadata(string $key): ?string;

    public function hasMetadata(string $key): bool;

    public function getPayloadSize(): int;

    /** @return array<string, string> */
    public function getAllMetadata(): array;

    public function __toString(): string;
}
