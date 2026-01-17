<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcTimestampAndDurationWellKnownTypesTest extends TestCase
{
    public function testGrpcTimestampAndDurationWellKnownTypes(): void
    {
        // Tests usage of google.protobuf.Timestamp and Duration types. Validates RFC 3339 timestamp serialization and duration calculations.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["event" => ["id" => "event-001", "occurred_at" => "2024-01-15T10:30:45.123Z", "duration_seconds" => 3600, "duration_nanos" => 500000000]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.EventService',
            methodName: 'LogEvent',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcTimestampAndDurationWellKnownTypes($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["event_id" => "event-001", "processed_at" => "2024-01-15T10:31:45.123Z", "processing_time_ms" => 1000]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
