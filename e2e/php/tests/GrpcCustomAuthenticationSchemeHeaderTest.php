<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcCustomAuthenticationSchemeHeaderTest extends TestCase
{
    public function testGrpcCustomAuthenticationSchemeHeader(): void
    {
        // Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-custom-auth" => "CustomScheme token_value_123"];
        $requestPayload = json_encode(["action" => "execute"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.CustomAuthService',
            methodName: 'Execute',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcCustomAuthenticationSchemeHeader($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["success" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
