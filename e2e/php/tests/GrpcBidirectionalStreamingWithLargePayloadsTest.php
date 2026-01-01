    public function testGrpcBidirectionalStreamingWithLargePayloads(): void
    {
        // Tests bidirectional streaming RPC with large messages in both directions. Validates concurrent read/write handling and proper message ordering.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["message_id" => "bi-large-001", "sequence" => 1, "data" => "QmlkaXJlY3Rpb25hbCBzdHJlYW1pbmcgdGVzdCB3aXRoIGxhcmdlIHBheWxvYWRzIGluIGJvdGggZGlyZWN0aW9ucy4gVGhpcyB2YWxpZGF0ZXMgdGhlIHNlcnZlcidzIGFiaWxpdHkgdG8gaGFuZGxlIGNvbmN1cnJlbnQgcmVhZHMgYW5kIHdyaXRlcywgYXMgd2VsbCBhcyBwcm9wZXIgbWVzc2FnZSBvcmRlcmluZy4=", "direction" => "client-to-server"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.BiDirectionalService',
            methodName: 'Exchange',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcBidirectionalStreamingWithLargePayloads($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["message_id" => "bi-large-001", "sequence" => 1, "direction" => "server-to-client"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

