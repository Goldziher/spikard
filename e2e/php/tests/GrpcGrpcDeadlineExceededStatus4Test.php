    public function testGrpcGrpcDeadlineExceededStatus4(): void
    {
        // Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "timeout-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.TimeoutService',
            methodName: 'SlowOp',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcDeadlineExceededStatus4($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('DEADLINE_EXCEEDED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

