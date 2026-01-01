    public function testGrpcOneofFieldHandling(): void
    {
        // Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "oneof-test-001", "text_data" => "This is text data"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.OneofService',
            methodName: 'ProcessOneof',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcOneofFieldHandling($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["received_type" => "text_data", "data_present" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

