<?php

namespace Spikard\Grpc;

/**
 * Stub for IDE support - gRPC handler interface.
 *
 * @see \Spikard\Grpc\HandlerInterface
 */
interface HandlerInterface
{
    public function handleRequest(Request $request): Response;
}
