    public function testGrpcLargeBinaryDataInBytesField(): void
    {
        // Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["file_id" => "binary-large-001", "content_type" => "application/octet-stream", "binary_content" => "/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAIBAQIBAQICAgICAgICAwUDAwwDAwYEBAMFBwYHBwcGBwcICQsJCAgKCAcHCg0KCgsMDAwMBwkODw0MDgsMDAz/2wBDAQICAgMDAwYDAwYMCAcIDAwIDAwYDAwMDAwYDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAz/wAARCAABAAEDASIAAhEBAxEB/8QAFAABAAAAAAAAAAAAAAAAAAAAA//EABQAQQAAAAAAAAAAAAAAAAAAAAr/xAAUAQEBAAAAAAAAAAAAAAAAAAAAAv/EABERAAAAAAAAAAAAAAAAAAAAAf/aAAwDAQACEQMRAD8AwA8A/9k="]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.BinaryService',
            methodName: 'UploadBinary',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcLargeBinaryDataInBytesField($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["file_id" => "binary-large-001", "bytes_received" => 512000]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

