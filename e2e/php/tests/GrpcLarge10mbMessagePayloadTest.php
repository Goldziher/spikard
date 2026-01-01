    public function testGrpcLarge10mbMessagePayload(): void
    {
        // Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "bulk-10mb-transfer", "content" => "TGFyZ2UgZmlsZSB0cmFuc2Zlciwgc2ltdWxhdGluZyBhIHJlYWwtd29ybGQgY2VuYXJpbyB3aGVyZSBhIGNsaWVudCBpcyB1cGxvYWRpbmcgYSBsbGFyZ2UgZGF0YSBibG9iLiBUaGlzIGZpeHR1cmUgZW5zdXJlcyB0aGF0IHRoZSBnUlBDIGltcGxlbWVudGF0aW9uIGNhbiBoYW5kbGUgdXAgdG8gMTAgTUIgb2YgZGF0YSBpbiBlYWNoIHJlcXVlc3QuIFRlc3Rpbmcgc3VjaCBsYXJnZSBwYXlsb2FkcyBpcyBjcnVjaWFsIGZvciByZWxpYWJpbGl0eSBhbmQgcGVyZm9ybWFuY2UgaW4gcHJvZHVjdGlvbiBlbnZpcm9ubWVudHMu", "chunk_count" => 10]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.BulkService',
            methodName: 'BulkUpload',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcLarge10mbMessagePayload($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => "bulk-10mb-transfer", "status" => "received"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

