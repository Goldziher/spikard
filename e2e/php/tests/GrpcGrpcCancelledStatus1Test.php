    public function testGrpcGrpcCancelledStatus1(): void
    {
        // Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "cancel-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.CancelService',
            methodName: 'Operation',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcCancelledStatus1($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('CANCELLED', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => "cancel-001"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

