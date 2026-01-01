    public function testGrpcFullAuthorizationContextWithRoleBasedAccessControl(): void
    {
        // Tests complete authorization context including user roles, permissions, and resource-level access control.

        // Build gRPC request from fixture
        $metadata = ["x-user-roles" => "admin,editor", "x-user-permissions" => "read,write,delete", "x-user-id" => "user-admin-001", "content-type" => "application/grpc", "authorization" => "Bearer token123"];
        $requestPayload = json_encode(["resource_id" => "resource-456", "operation" => "write"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AuthzService',
            methodName: 'CheckAccess',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcFullAuthorizationContextWithRoleBasedAccessControl($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["authorized" => true, "message" => "Access granted with admin privileges"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

