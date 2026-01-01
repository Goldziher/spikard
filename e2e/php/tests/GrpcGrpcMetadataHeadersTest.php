    public function testGrpcGrpcMetadataHeaders(): void
    {
        // Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers.

        // Build gRPC request from fixture
        $metadata = ["x-custom-header" => "custom-value", "authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9", "x-trace-id" => "trace-abc123def456", "content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "req-987654321"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MetadataService',
            methodName: 'CheckMetadata',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcMetadataHeaders($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "req-987654321", "received_auth_header" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9", "received_trace_id" => "trace-abc123def456", "received_custom_header" => "custom-value", "response_time_ms" => 45]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

