<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcSpecialCharactersUnicodeAndEmojiInStringsTest extends TestCase
{
    public function testGrpcSpecialCharactersUnicodeAndEmojiInStrings(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["user_id" => "user-unicode-001", "message" => "Hello 世界 Привет שלום مرحبا", "emoji_field" => "🚀 🎉 🌟 ✨ 💻"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.EchoService',
            methodName: 'EchoSpecial',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcSpecialCharactersUnicodeAndEmojiInStrings($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["echo" => "Hello 世界 Привет שלום مرحبا"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
