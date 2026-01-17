<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcWellKnownWrapperTypesStringvalueInt32valueEtcTest extends TestCase
{
    public function testGrpcWellKnownWrapperTypesStringvalueInt32valueEtc(): void
    {
        // Tests usage of google.protobuf wrapper types (StringValue, Int32Value, BoolValue) for nullable scalar types. Validates proper null/present distinction.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "wrapper-test-001", "optional_name" => "Test Name", "optional_count" => 42]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.WrapperService',
            methodName: 'ProcessWrapper',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcWellKnownWrapperTypesStringvalueInt32valueEtc($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => "wrapper-test-001", "name_present" => true, "name_value" => "Test Name", "count_present" => true, "count_value" => 42]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
