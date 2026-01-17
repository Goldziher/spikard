<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcTimeoutWithRetryMetadataTest extends TestCase
{
    public function testGrpcTimeoutWithRetryMetadata(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "retry-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.RetryService',
            methodName: 'RetryableOperation',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcTimeoutWithRetryMetadata($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('DEADLINE_EXCEEDED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
