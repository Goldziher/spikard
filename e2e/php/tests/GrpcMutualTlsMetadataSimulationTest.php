<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcMutualTlsMetadataSimulationTest extends TestCase
{
    public function testGrpcMutualTlsMetadataSimulation(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-client-cert-fingerprint" => "AB:CD:EF:12:34:56:78:90", "x-client-cert-cn" => "client.example.com"];
        $requestPayload = json_encode(["operation" => "secure_read"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MtlsService',
            methodName: 'VerifyClient',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcMutualTlsMetadataSimulation($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["verified" => true, "client_cn" => "client.example.com"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
