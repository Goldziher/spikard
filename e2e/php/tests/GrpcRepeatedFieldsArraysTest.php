    public function testGrpcRepeatedFieldsArrays(): void
    {
        // Tests arrays/repeated fields for primitive types and messages. Covers repeated field serialization and deserialization.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["title" => "Getting Started with gRPC", "content" => "This is a comprehensive guide to gRPC...", "tag_ids" => [1, 2, 3]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.BlogService',
            methodName: 'CreatePost',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcRepeatedFieldsArrays($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => 789, "title" => "Getting Started with gRPC", "content" => "This is a comprehensive guide to gRPC...", "tags" => [["id" => 1, "name" => "gRPC"], ["id" => 2, "name" => "Protocol Buffers"], ["id" => 3, "name" => "RPC"]], "categories" => ["tutorial", "programming", "networking"]]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

