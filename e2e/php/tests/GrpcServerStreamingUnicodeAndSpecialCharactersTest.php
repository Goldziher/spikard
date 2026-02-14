<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingUnicodeAndSpecialCharactersTest extends TestCase
{
    public function testGrpcServerStreamingUnicodeAndSpecialCharacters(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "encoding" => "utf-8"];
        $requestPayload = json_encode(["filter" => "all"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'StreamUnicodeMessages',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingUnicodeAndSpecialCharacters($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("Unicode stream completed successfully"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
