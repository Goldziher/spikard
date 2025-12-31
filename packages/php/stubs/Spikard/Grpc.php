<?php

namespace Spikard;

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Spikard\Grpc\Service;

/**
 * Stub for IDE support - gRPC facade.
 *
 * @see \Spikard\Grpc
 */
final class Grpc
{
    public static function createService(): Service;

    /**
     * @param array<string, string> $metadata
     */
    public static function createRequest(
        string $serviceName,
        string $methodName,
        string $payload,
        array $metadata = []
    ): Request;

    /**
     * @param array<string, string> $metadata
     */
    public static function createResponse(string $payload, array $metadata = []): Response;

    /**
     * @param array<string, string> $metadata
     */
    public static function createErrorResponse(string $message, array $metadata = []): Response;
}
