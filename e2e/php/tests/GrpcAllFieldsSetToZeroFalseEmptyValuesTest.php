    public function testGrpcAllFieldsSetToZeroFalseEmptyValues(): void
    {
        // Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["int_field" => 0, "bool_field" => false, "string_field" => "", "bytes_field" => "", "float_field" => 0.0]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ZeroValueService',
            methodName: 'ProcessZeros',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcAllFieldsSetToZeroFalseEmptyValues($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["success" => true, "fields_received" => 5]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

