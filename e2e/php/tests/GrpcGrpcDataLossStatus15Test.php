<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcDataLossStatus15Test extends TestCase
{
    public function testGrpcGrpcDataLossStatus15(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "dataloss-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.DataService',
            methodName: 'Process',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcDataLossStatus15($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('DATA_LOSS', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
