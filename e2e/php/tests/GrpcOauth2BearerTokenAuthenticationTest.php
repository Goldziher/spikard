<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcOauth2BearerTokenAuthenticationTest extends TestCase
{
    public function testGrpcOauth2BearerTokenAuthentication(): void
    {
        // Tests OAuth2 Bearer token authentication. Validates token validation and scope checking.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "authorization" => "Bearer ya29.a0AfH6SMBx..."];
        $requestPayload = json_encode(["scope" => "read:users"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.OAuth2Service',
            methodName: 'CheckScope',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcOauth2BearerTokenAuthentication($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["granted" => true, "token_info" => "oauth2_token"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
