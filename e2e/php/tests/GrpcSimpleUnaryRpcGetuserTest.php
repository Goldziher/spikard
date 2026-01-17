<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcSimpleUnaryRpcGetuserTest extends TestCase
{
    public function testGrpcSimpleUnaryRpcGetuser(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.

        // Build gRPC request from fixture
        $metadata = ["authorization" => "Bearer test-token", "content-type" => "application/grpc"];
        $requestPayload = json_encode(["user_id" => 123]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UserService',
            methodName: 'GetUser',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcSimpleUnaryRpcGetuser($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => 123, "name" => "Alice Johnson", "email" => "alice@example.com"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
