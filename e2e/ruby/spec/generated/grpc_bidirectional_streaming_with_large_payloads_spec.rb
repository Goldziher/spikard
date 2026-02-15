  describe "gRPC: Exchange" do
    it "Tests bidirectional streaming RPC with large messages in both directions. Validates concurrent read/write handling and proper message ordering." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"message_id\":\"bi-large-001\",\"sequence\":1,\"data\":\"QmlkaXJlY3Rpb25hbCBzdHJlYW1pbmcgdGVzdCB3aXRoIGxhcmdlIHBheWxvYWRzIGluIGJvdGggZGlyZWN0aW9ucy4gVGhpcyB2YWxpZGF0ZXMgdGhlIHNlcnZlcidzIGFiaWxpdHkgdG8gaGFuZGxlIGNvbmN1cnJlbnQgcmVhZHMgYW5kIHdyaXRlcywgYXMgd2VsbCBhcyBwcm9wZXIgbWVzc2FnZSBvcmRlcmluZy4=\",\"direction\":\"client-to-server\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BiDirectionalService",
        method_name: "Exchange",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_with_large_payloads(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"message_id\":\"bi-large-001\",\"sequence\":1,\"direction\":\"server-to-client\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
