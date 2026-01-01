    public function testGrpcJwtBearerTokenAuthentication(): void
    {
        // Tests JWT authentication via gRPC metadata. Validates that JWT tokens are properly extracted and validated from authorization header.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTEyMyIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"];
        $requestPayload = json_encode(["action" => "read"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AuthService',
            methodName: 'SecureAction',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcJwtBearerTokenAuthentication($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["user_id" => "user-123", "action" => "read"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

