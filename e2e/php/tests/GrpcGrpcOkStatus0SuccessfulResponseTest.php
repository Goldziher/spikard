<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcOkStatus0SuccessfulResponseTest extends TestCase
{
    public function testGrpcGrpcOkStatus0SuccessfulResponse(): void
    {
        // Tests successful gRPC response with OK status code. Validates basic request-response completion.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "status-ok-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StatusService',
            methodName: 'CheckStatus',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcOkStatus0SuccessfulResponse($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "status-ok-001", "status" => "success"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
