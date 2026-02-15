<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcErrorHandlingUnauthenticatedServerStreamingRequestTest extends TestCase
{
    public function testGrpcErrorHandlingUnauthenticatedServerStreamingRequest(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["resource" => "protected_data"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorTestService',
            methodName: 'SecureStream',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNAUTHENTICATED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
