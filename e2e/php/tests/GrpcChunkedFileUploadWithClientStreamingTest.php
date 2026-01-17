<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcChunkedFileUploadWithClientStreamingTest extends TestCase
{
    public function testGrpcChunkedFileUploadWithClientStreaming(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC for chunked file uploads. Validates that multiple message chunks are properly accumulated and processed by the server.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["file_id" => "chunked-upload-test", "chunk_number" => 1, "chunk_data" => "Q2h1bmsgMQ==", "is_final" => false]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StorageService',
            methodName: 'ChunkedUpload',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcChunkedFileUploadWithClientStreaming($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["file_id" => "chunked-upload-test", "total_chunks" => 5, "total_size" => 102400, "upload_status" => "completed"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
