    public function testGrpcGrpcOutOfRangeStatus11(): void
    {
        // Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["value" => 1000]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.RangeService',
            methodName: 'Check',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcOutOfRangeStatus11($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OUT_OF_RANGE', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

