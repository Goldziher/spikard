<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcAbortedStatus10Test extends TestCase
{
    public function testGrpcGrpcAbortedStatus10(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "txn-conflict"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.TransactionService',
            methodName: 'Commit',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcAbortedStatus10($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('ABORTED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
