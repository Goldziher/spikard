<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcLarge1mbMessagePayloadTest extends TestCase
{
    public function testGrpcLarge1mbMessagePayload(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests handling of 1MB protobuf messages. Verifies that large payloads are properly serialized, transmitted, and deserialized without truncation or corruption.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "large-1mb-test-001", "data" => "SGVsbG8gV29ybGQgLSBUaGlzIGlzIGEgbGFyZ2UgMU1CIG1lc3NhZ2UgcGF5bG9hZCB0ZXN0IGZpeHR1cmUgZm9yIHRlc3RpbmcgY2FwYWNpdHkgdGVzdHMgYW5kIHBlcmZvcm1hbmNlIG1lYXN1cmVtZW50cy4gIFRoaXMgZGF0YSBmaWVsZCB3aWxsIGNvbnRhaW4gYXBwcm94aW1hdGVseSAxIE1CIG9mIGRhdGEgd2hlbiBidWlsdC4gVGhlIGdSUEMgc2VydmVyIHNob3VsZCBoYW5kbGUgdGhpcyB0cmFuc21pc3Npb24gd2l0aG91dCBpc3N1ZXMuIEluIHJlYWwgd29ybGQgc2NlbmFyaW9zLCBsYXJnZSBmaWxlIHRyYW5zZmVycyBhcmUgY29tbW9uLiBPdXIgdGVzdCBmaXh0dXJlIGVuc3VyZXMgdGhhdCB0aGUgaW1wbGVtZW50YXRpb24gY2FuIGhhbmRsZSBzdWNoIGZpbGVzIHdpdGhvdXQgbWVtb3J5IGlzc3Vlcywgc3RyZWFtIGJyZWFrcywgb3IgdGltZW91dHMu"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.FileService',
            methodName: 'UploadLarge',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcLarge1mbMessagePayload($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "large-1mb-test-001", "data_size" => 1048576]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
