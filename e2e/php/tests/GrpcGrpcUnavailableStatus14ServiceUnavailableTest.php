<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcUnavailableStatus14ServiceUnavailableTest extends TestCase
{
    public function testGrpcGrpcUnavailableStatus14ServiceUnavailable(): void
    {
        // Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "unavail-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UnavailService',
            methodName: 'Request',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcUnavailableStatus14ServiceUnavailable($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNAVAILABLE', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
