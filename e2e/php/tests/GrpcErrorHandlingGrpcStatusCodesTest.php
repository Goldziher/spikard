    public function testGrpcErrorHandlingGrpcStatusCodes(): void
    {
        // Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["product_id" => -1]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ProductService',
            methodName: 'GetProduct',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingGrpcStatusCodes($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INVALID_ARGUMENT', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

