<?php

declare(strict_types=1);

namespace Spikard\Grpc;

/**
 * Represents a gRPC response.
 *
 * Contains the serialized protobuf payload and optional metadata (headers) to
 * be sent back to the client.
 *
 * @psalm-immutable
 */
final class Response
{
    /**
     * @param string $payload Serialized protobuf message as binary string
     * @param array<string, string> $metadata gRPC metadata (headers) to include in response
     */
    public function __construct(
        public readonly string $payload,
        public readonly array $metadata = []
    ) {}

    /**
     * Create a response with error status.
     *
     * @param string $message Error message
     * @param array<string, string> $metadata Optional metadata
     *
     * @return static
     */
    public static function error(string $message, array $metadata = []): self
    {
        $metadata['grpc-status'] = 'INTERNAL';
        $metadata['grpc-message'] = $message;
        return new self('', $metadata);
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
        return sprintf('Response(payloadSize=%d)', $this->getPayloadSize());
    }
}
