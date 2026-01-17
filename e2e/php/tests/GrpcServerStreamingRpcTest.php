<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingRpcTest extends TestCase
{
    public function testGrpcServerStreamingRpc(): void
    {
        // Tests server streaming where the server sends multiple responses. Covers streaming response patterns.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["category_id" => 5, "limit" => 100]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ItemService',
            methodName: 'ListItems',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingRpc($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
