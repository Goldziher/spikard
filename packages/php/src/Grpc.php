<?php

declare(strict_types=1);

namespace Spikard;

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Spikard\Grpc\Service;

/**
 * Facade for gRPC functionality in Spikard.
 *
 * Provides convenient static methods for creating gRPC services and managing handlers.
 *
 * @example
 * ```php
 * // Create a service
 * $grpcService = Grpc::createService();
 * $grpcService->registerHandler('mypackage.UserService', new UserServiceHandler());
 *
 * // Create request/response objects
 * $request = Grpc::createRequest('mypackage.UserService', 'GetUser', $payload);
 * $response = new Grpc\Response($serialized);
 * ```
 */
final class Grpc
{
    /**
     * Create a new gRPC service registry.
     *
     * @return Service A new service instance for managing handlers
     */
    public static function createService(): Service
    {
        return new Service();
    }

    /**
     * Create a new gRPC request.
     *
     * @param string $serviceName Fully qualified service name
     * @param string $methodName Method name
     * @param string $payload Serialized protobuf payload
     * @param array<string, string> $metadata Optional gRPC metadata
     *
     * @return Request
     */
    public static function createRequest(
        string $serviceName,
        string $methodName,
        string $payload,
        array $metadata = []
    ): Request {
        return new Request($serviceName, $methodName, $payload, $metadata);
    }

    /**
     * Create a new gRPC response.
     *
     * @param string $payload Serialized protobuf payload
     * @param array<string, string> $metadata Optional gRPC metadata
     *
     * @return Response
     */
    public static function createResponse(string $payload, array $metadata = []): Response
    {
        return new Response($payload, $metadata);
    }

    /**
     * Create an error response.
     *
     * @param string $message Error message
     * @param array<string, string> $metadata Optional gRPC metadata
     *
     * @return Response
     */
    public static function createErrorResponse(string $message, array $metadata = []): Response
    {
        return Response::error($message, $metadata);
    }
}
