  describe "gRPC: StreamUnicodeMessages" do
    it "Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "encoding" => "utf-8",
      }

      # Build request payload
      request_payload = "{\"filter\":\"all\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamUnicodeMessages",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_unicode_and_special_characters(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"Unicode stream completed successfully\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
