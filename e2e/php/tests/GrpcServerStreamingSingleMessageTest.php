<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingSingleMessageTest extends TestCase
{
    public function testGrpcServerStreamingSingleMessage(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["query" => "find_first"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'GetSingleMessage',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingSingleMessage($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("Stream completed with one message"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
