  describe "gRPC: GetEmptyStream" do
    it "Tests server streaming RPC that returns an empty stream. The server opens the stream but sends no messages before completing successfully." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"empty-stream-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "GetEmptyStream",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_empty_stream(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"Stream completed with no messages\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
