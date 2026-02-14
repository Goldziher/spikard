  describe "gRPC: GetSingleMessage" do
    it "Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"query\":\"find_first\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "GetSingleMessage",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_single_message(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"Stream completed with one message\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
