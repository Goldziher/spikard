<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcCompressionTestGzipTest extends TestCase
{
    public function testGrpcGrpcCompressionTestGzip(): void
    {
        // Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.

        // Build gRPC request from fixture
        $metadata = ["grpc-encoding" => "gzip", "content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "compress-test-001", "data" => "VGhpcyBpcyBhIHRlc3QgcGF5bG9hZCB0aGF0IHNob3VsZCBiZSBjb21wcmVzc2VkIHdpdGggZ3ppcC4gUmVwZWF0aW5nIHRvIGluY3JlYXNlIGNvbXByZXNzaWJpbGl0eTogVGhpcyBpcyBhIHRlc3QgcGF5bG9hZCB0aGF0IHNob3VsZCBiZSBjb21wcmVzc2VkIHdpdGggZ3ppcC4gUmVwZWF0aW5nIHRvIGluY3JlYXNlIGNvbXByZXNzaWJpbGl0eS4="]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.CompressionService',
            methodName: 'SendCompressed',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcCompressionTestGzip($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => "compress-test-001", "compressed" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
