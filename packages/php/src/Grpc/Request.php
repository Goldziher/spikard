<?php

declare(strict_types=1);

namespace Spikard\Grpc;

/**
 * Represents a gRPC request.
 *
 * Contains the fully qualified service name, method name, serialized protobuf payload,
 * and metadata (gRPC headers) for an incoming gRPC request.
 *
 * @psalm-immutable
 */
final class Request
{
    /**
     * @param string $serviceName Fully qualified service name (e.g., "mypackage.MyService")
     * @param string $methodName Method name (e.g., "GetUser")
     * @param string $payload Serialized protobuf message as binary string
     * @param array<string, string> $metadata gRPC metadata (headers)
     */
    public function __construct(
        public readonly string $serviceName,
        public readonly string $methodName,
        public readonly string $payload,
        public readonly array $metadata = []
    ) {}

    /**
     * Get a metadata value by key.
     *
     * @return string|null The metadata value, or null if not found
     */
    public function getMetadata(string $key): ?string
    {
        return $this->metadata[$key] ?? null;
    }

    /**
     * Check if a metadata key exists.
     *
     * @return bool True if the key exists
     */
    public function hasMetadata(string $key): bool
    {
        return isset($this->metadata[$key]);
    }

    /**
     * Get the payload size in bytes.
     *
     * @return int Number of bytes in the payload
     */
    public function getPayloadSize(): int
    {
        return strlen($this->payload);
    }

    /**
     * Get all metadata as an associative array.
     *
     * @return array<string, string>
     */
    public function getAllMetadata(): array
    {
        return $this->metadata;
    }

    /**
     * String representation for debugging.
     */
    public function __toString(): string
    {
        return sprintf(
            'Request(serviceName=%s, methodName=%s, payloadSize=%d)',
            $this->serviceName,
            $this->methodName,
            $this->getPayloadSize()
        );
    }
}
