<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcUserAgentAndClientInfoMetadataTest extends TestCase
{
    public function testGrpcUserAgentAndClientInfoMetadata(): void
    {
        // Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "user-agent" => "grpc-client/1.2.3 (linux; amd64)"];
        $requestPayload = json_encode(["action" => "identify"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ClientService',
            methodName: 'Identify',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcUserAgentAndClientInfoMetadata($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["client_type" => "grpc-client", "client_version" => "1.2.3"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
