  describe "gRPC: SendCompressed" do
    it "Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression." do
      # Build gRPC request from fixture
      metadata = {
        "grpc-encoding" => "gzip",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"compress-test-001\",\"data\":\"VGhpcyBpcyBhIHRlc3QgcGF5bG9hZCB0aGF0IHNob3VsZCBiZSBjb21wcmVzc2VkIHdpdGggZ3ppcC4gUmVwZWF0aW5nIHRvIGluY3JlYXNlIGNvbXByZXNzaWJpbGl0eTogVGhpcyBpcyBhIHRlc3QgcGF5bG9hZCB0aGF0IHNob3VsZCBiZSBjb21wcmVzc2VkIHdpdGggZ3ppcC4gUmVwZWF0aW5nIHRvIGluY3JlYXNlIGNvbXByZXNzaWJpbGl0eS4=\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.CompressionService",
        method_name: "SendCompressed",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_compression_test_gzip(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":\"compress-test-001\",\"compressed\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
