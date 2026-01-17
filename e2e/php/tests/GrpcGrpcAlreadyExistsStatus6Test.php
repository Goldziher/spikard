<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcAlreadyExistsStatus6Test extends TestCase
{
    public function testGrpcGrpcAlreadyExistsStatus6(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "duplicate-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.CreateService',
            methodName: 'Create',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcAlreadyExistsStatus6($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('ALREADY_EXISTS', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
