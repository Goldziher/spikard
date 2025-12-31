<?php

declare(strict_types=1);

namespace Spikard\Grpc;

/**
 * Interface for gRPC request handlers.
 *
 * Implementations of this interface handle incoming gRPC requests and return
 * appropriate responses. The handler receives the deserialized request and
 * should return a response with the serialized protobuf payload.
 *
 * @example
 * ```php
 * class UserServiceHandler implements HandlerInterface
 * {
 *     public function handleRequest(Request $request): Response
 *     {
 *         // Deserialize request payload using google/protobuf
 *         $userRequest = new \Example\GetUserRequest();
 *         $userRequest->mergeFromString($request->payload);
 *
 *         // Process the request
 *         $user = $this->getUserFromDatabase($userRequest->getId());
 *
 *         // Serialize response
 *         return new Response(
 *             payload: $user->serializeToString()
 *         );
 *     }
 * }
 * ```
 */
interface HandlerInterface
{
    /**
     * Handle a gRPC request and return a response.
     *
     * The request payload is a serialized protobuf message. Your handler should:
     * 1. Deserialize the payload using google/protobuf
     * 2. Process the request logic
     * 3. Serialize the response back to protobuf
     * 4. Return a Response object with the serialized payload
     *
     * @param Request $request The incoming gRPC request
     *
     * @return Response The response with serialized payload
     *
     * @throws \Exception If the request cannot be processed
     */
    public function handleRequest(Request $request): Response;
}
