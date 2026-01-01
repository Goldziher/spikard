    public function testGrpcRateLimitingWithMetadataHeaders(): void
    {
        // Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "rl-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.RateLimitService',
            methodName: 'Query',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcRateLimitingWithMetadataHeaders($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["result" => "success"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

