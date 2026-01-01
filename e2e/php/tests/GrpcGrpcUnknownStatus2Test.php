    public function testGrpcGrpcUnknownStatus2(): void
    {
        // Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "unknown-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.UnknownService',
            methodName: 'Fail',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcUnknownStatus2($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNKNOWN', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

