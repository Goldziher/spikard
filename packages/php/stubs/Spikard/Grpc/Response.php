<?php

namespace Spikard\Grpc;

/**
 * Stub for IDE support - gRPC response class.
 *
 * @see \Spikard\Grpc\Response
 */
final class Response
{
    public readonly string $payload;
    /** @var array<string, string> */
    public readonly array $metadata;

    /**
     * @param array<string, string> $metadata
     */
    public function __construct(
        string $payload,
        array $metadata = []
    );

    /**
     * @param array<string, string> $metadata
     */
    public static function error(string $message, array $metadata = []): self;

    public function getPayloadSize(): int;

    public function getMetadata(string $key): ?string;

    public function hasMetadata(string $key): bool;

    /** @return array<string, string> */
    public function getAllMetadata(): array;

    public function __toString(): string;
}
