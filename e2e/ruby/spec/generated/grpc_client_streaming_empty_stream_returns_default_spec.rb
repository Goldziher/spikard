  describe "gRPC: ProcessOptional" do
    it "Tests client streaming RPC where client sends no messages (empty stream). Server gracefully handles empty input and returns default response." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.OptionalService",
        method_name: "ProcessOptional",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_empty_stream_returns_default(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"empty-stream-req\",\"message_count\":0,\"result\":\"DEFAULT_RESULT\",\"is_default\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
