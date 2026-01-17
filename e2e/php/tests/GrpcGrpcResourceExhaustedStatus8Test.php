<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcResourceExhaustedStatus8Test extends TestCase
{
    public function testGrpcGrpcResourceExhaustedStatus8(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.).

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["size" => 9999999999]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ResourceService',
            methodName: 'AllocateMemory',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcResourceExhaustedStatus8($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('RESOURCE_EXHAUSTED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
