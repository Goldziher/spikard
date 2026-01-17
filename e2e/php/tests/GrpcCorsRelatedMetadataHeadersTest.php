<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcCorsRelatedMetadataHeadersTest extends TestCase
{
    public function testGrpcCorsRelatedMetadataHeaders(): void
    {
        // Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling.

        // Build gRPC request from fixture
        $metadata = ["origin" => "https://example.com", "access-control-request-method" => "POST", "content-type" => "application/grpc"];
        $requestPayload = json_encode(["resource" => "data"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.CorsService',
            methodName: 'CheckCors',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcCorsRelatedMetadataHeaders($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["allowed" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
