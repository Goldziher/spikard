<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcMaximumFieldNumber536870911Test extends TestCase
{
    public function testGrpcMaximumFieldNumber536870911(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => 42, "max_field_value" => "Testing maximum field number"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MaxFieldService',
            methodName: 'TestMaxField',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcMaximumFieldNumber536870911($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => 42, "received_max" => "Testing maximum field number"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
