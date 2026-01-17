<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcApiKeyAuthenticationTest extends TestCase
{
    public function testGrpcApiKeyAuthentication(): void
    {
        // Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-api-key" => "sk_live_abc123def456"];
        $requestPayload = json_encode(["resource" => "users"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ApiService',
            methodName: 'FetchResource',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcApiKeyAuthentication($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["data" => "resource_data", "client_id" => "client-api-001"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
