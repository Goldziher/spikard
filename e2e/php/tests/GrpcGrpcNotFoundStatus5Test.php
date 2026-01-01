    public function testGrpcGrpcNotFoundStatus5(): void
    {
        // Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["resource_id" => 99999]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ResourceService',
            methodName: 'Get',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcNotFoundStatus5($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('NOT_FOUND', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

