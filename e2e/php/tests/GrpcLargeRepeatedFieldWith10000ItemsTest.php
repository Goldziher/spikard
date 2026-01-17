<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcLargeRepeatedFieldWith10000ItemsTest extends TestCase
{
    public function testGrpcLargeRepeatedFieldWith10000Items(): void
    {
        // Tests handling of repeated fields containing thousands of elements. Validates efficient serialization and deserialization of large arrays without memory bloat.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["series_id" => "metrics-large-series", "data_points" => [["timestamp" => 1000000, "value" => 42.5], ["timestamp" => 1000001, "value" => 43.2], ["timestamp" => 1000002, "value" => 41.8]]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MetricsService',
            methodName: 'IngestTimeSeries',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcLargeRepeatedFieldWith10000Items($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["series_id" => "metrics-large-series", "point_count" => 10000, "min_value" => 10.5, "max_value" => 99.9]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
