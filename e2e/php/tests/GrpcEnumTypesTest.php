    public function testGrpcEnumTypes(): void
    {
        // Tests enum definitions and serialization. Covers enum fields with named constants.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["product_name" => "Laptop", "quantity" => 2, "priority" => "HIGH"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.OrderService',
            methodName: 'CreateOrder',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcEnumTypes($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => 1001, "product_name" => "Laptop", "quantity" => 2, "status" => "PENDING", "priority" => "HIGH"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

