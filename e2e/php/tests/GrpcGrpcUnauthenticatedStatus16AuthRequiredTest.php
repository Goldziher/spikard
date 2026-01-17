<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcUnauthenticatedStatus16AuthRequiredTest extends TestCase
{
    public function testGrpcGrpcUnauthenticatedStatus16AuthRequired(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["action" => "sensitive"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AuthService',
            methodName: 'SecureOp',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcUnauthenticatedStatus16AuthRequired($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNAUTHENTICATED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
