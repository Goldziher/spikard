<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcOptionalFieldsTest extends TestCase
{
    public function testGrpcOptionalFields(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests optional field handling with presence semantics. Covers optional fields with and without values.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["user_id" => 42, "bio" => "Software engineer and gRPC enthusiast"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UserService',
            methodName: 'UpdateProfile',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcOptionalFields($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["user_id" => 42, "username" => "charlie_dev", "bio" => "Software engineer and gRPC enthusiast", "updated_at" => 1704067200000]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
