<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcPermissionDeniedStatus7Test extends TestCase
{
    public function testGrpcGrpcPermissionDeniedStatus7(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["action" => "delete_all"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.SecureService',
            methodName: 'AdminAction',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcPermissionDeniedStatus7($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('PERMISSION_DENIED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
