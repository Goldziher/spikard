<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingWithLargeBatchRequestsTest extends TestCase
{
    public function testGrpcClientStreamingWithLargeBatchRequests(): void
    {
        // Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["item_id" => "batch-item-001", "payload" => "TGFyZ2UgYmF0Y2ggaXRlbSBwYXlsb2FkIGRhdGEgZm9yIHRlc3RpbmcgY2xpZW50IHN0cmVhbWluZyBjYXBhYmlsaXRpZXMgd2l0aCBtdWx0aXBsZSBsYXJnZSByZXF1ZXN0cy4gVGhpcyBlbnN1cmVzIHRoYXQgdGhlIHNlcnZlciBjYW4gYWNjdW11bGF0ZSBhbmQgcHJvY2VzcyBzdWNoIGJhdGNoZXMgY29ycmVjdGx5Lg==", "sequence" => 1]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.BatchService',
            methodName: 'ProcessBatch',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingWithLargeBatchRequests($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["batch_id" => "batch-large-001", "items_processed" => 100, "total_bytes" => 5242880]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
