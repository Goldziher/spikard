    public function testGrpcBidirectionalStreamingRpc(): void
    {
        // Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "authorization" => "Bearer user-token"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ChatService',
            methodName: 'Chat',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcBidirectionalStreamingRpc($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

