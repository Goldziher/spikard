<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcUnimplementedStatus12Test extends TestCase
{
    public function testGrpcGrpcUnimplementedStatus12(): void
    {
        // Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "unimp-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UnimplService',
            methodName: 'NotYetImplemented',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcUnimplementedStatus12($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNIMPLEMENTED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
