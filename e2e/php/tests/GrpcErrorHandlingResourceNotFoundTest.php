<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcErrorHandlingResourceNotFoundTest extends TestCase
{
    public function testGrpcErrorHandlingResourceNotFound(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "nonexistent-123"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorTestService',
            methodName: 'GetResource',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingResourceNotFound($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('NOT_FOUND', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
