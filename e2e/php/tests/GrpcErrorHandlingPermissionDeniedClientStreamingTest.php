<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcErrorHandlingPermissionDeniedClientStreamingTest extends TestCase
{
    public function testGrpcErrorHandlingPermissionDeniedClientStreaming(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorTestService',
            methodName: 'UploadRestricted',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingPermissionDeniedClientStreaming($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('PERMISSION_DENIED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
