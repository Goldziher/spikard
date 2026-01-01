    public function testGrpcGrpcInvalidArgumentStatus3(): void
    {
        // Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["value" => -999]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ArgService',
            methodName: 'Validate',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcInvalidArgumentStatus3($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INVALID_ARGUMENT', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

