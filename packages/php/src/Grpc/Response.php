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
    public readonly string $payload;
    /** @var array<string, string> */
    public readonly array $metadata;

    private const STATUS_CODES = [
        'OK' => '0',
        'CANCELLED' => '1',
        'UNKNOWN' => '2',
        'INVALID_ARGUMENT' => '3',
        'DEADLINE_EXCEEDED' => '4',
        'NOT_FOUND' => '5',
        'ALREADY_EXISTS' => '6',
        'PERMISSION_DENIED' => '7',
        'RESOURCE_EXHAUSTED' => '8',
        'FAILED_PRECONDITION' => '9',
        'ABORTED' => '10',
        'OUT_OF_RANGE' => '11',
        'UNIMPLEMENTED' => '12',
        'INTERNAL' => '13',
        'UNAVAILABLE' => '14',
        'DATA_LOSS' => '15',
        'UNAUTHENTICATED' => '16',
    ];

    /**
     * @param string $payload Serialized protobuf message as binary string
     * @param array<string, string> $metadata gRPC metadata (headers) to include in response
     */
    public function __construct(string $payload, array $metadata = [])
    {
        if (isset($metadata['grpc-status'])) {
            $metadata['grpc-status'] = self::normalizeStatus($metadata['grpc-status']);
        }

        $this->payload = $payload;
        $this->metadata = $metadata;
    }

    /**
     * Create a response with error status.
     *
     * @param string $message Error message
     * @param int|string|array<string, string> $statusOrMetadata Optional gRPC status alias/code
     *     or metadata array
     * @param array<string, string> $metadata Optional metadata when a status is provided
     *
     * @return static
     */
    public static function error(string $message, int|string|array $statusOrMetadata = [], array $metadata = []): self
    {
        $status = null;
        if (\is_array($statusOrMetadata)) {
            $metadata = [...$statusOrMetadata, ...$metadata];
        } else {
            $status = $statusOrMetadata;
        }

        $metadata['grpc-status'] = self::normalizeStatus($status ?? ($metadata['grpc-status'] ?? 'INTERNAL'));
        $metadata['grpc-message'] = $message;
        return new self('', $metadata);
    }

    private static function normalizeStatus(int|string $status): string
    {
        if (\is_int($status)) {
            return (string) $status;
        }

        $normalized = \trim($status);
        if (\ctype_digit($normalized)) {
            return $normalized;
        }

        $upper = \strtoupper($normalized);
        if (isset(self::STATUS_CODES[$upper])) {
            return self::STATUS_CODES[$upper];
        }

        throw new \InvalidArgumentException(\sprintf('Unknown gRPC status: %s', $status));
    }

    /**
     * Get the payload size in bytes.
     *
     * @return int Number of bytes in the payload
     */
    public function getPayloadSize(): int
    {
        return \strlen($this->payload);
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
        return \sprintf('Response(payloadSize=%d)', $this->getPayloadSize());
    }
}
