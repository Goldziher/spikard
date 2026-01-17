<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcNestedMessagesTest extends TestCase
{
    public function testGrpcNestedMessages(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["name" => "Bob Smith", "email" => "bob@example.com", "address" => ["street" => "123 Main St", "city" => "Springfield", "zip_code" => "12345"]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UserService',
            methodName: 'CreateUser',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcNestedMessages($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["user_id" => 456, "name" => "Bob Smith", "email" => "bob@example.com", "address" => ["street" => "123 Main St", "city" => "Springfield", "zip_code" => "12345"]]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
