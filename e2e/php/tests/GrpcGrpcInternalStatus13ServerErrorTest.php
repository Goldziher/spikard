<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcInternalStatus13ServerErrorTest extends TestCase
{
    public function testGrpcGrpcInternalStatus13ServerError(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests INTERNAL gRPC status code. Returned when an internal server error occurs.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "internal-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.InternalService',
            methodName: 'Fail',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcInternalStatus13ServerError($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INTERNAL', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
