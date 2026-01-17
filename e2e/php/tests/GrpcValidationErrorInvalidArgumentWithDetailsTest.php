<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcValidationErrorInvalidArgumentWithDetailsTest extends TestCase
{
    public function testGrpcValidationErrorInvalidArgumentWithDetails(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["email" => "invalid-email", "age" => -5]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ValidationService',
            methodName: 'ValidateInput',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcValidationErrorInvalidArgumentWithDetails($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INVALID_ARGUMENT', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
