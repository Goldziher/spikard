<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreaming10MessagesTest extends TestCase
{
    public function testGrpcServerStreaming10Messages(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["offset" => 0, "limit" => 10]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'ListItems',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreaming10Messages($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("10 messages streamed successfully"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
