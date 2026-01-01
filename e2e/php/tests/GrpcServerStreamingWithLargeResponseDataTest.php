    public function testGrpcServerStreamingWithLargeResponseData(): void
    {
        // Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["stream_id" => "stream-large-001", "chunk_size" => 1048576]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamingService',
            methodName: 'StreamLargeData',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingWithLargeResponseData($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["stream_id" => "stream-large-001", "chunk_number" => 1, "is_final" => false]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

