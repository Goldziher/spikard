    public function testGrpcClientStreamingRpc(): void
    {
        // Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.FileService',
            methodName: 'Upload',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingRpc($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["file_id" => "file-12345", "total_bytes" => 57, "status" => "COMPLETED", "checksum" => "d8e8fca2dc0f896fd7cb4cb0031ba249"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

