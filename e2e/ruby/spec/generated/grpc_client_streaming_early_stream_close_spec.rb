  describe "gRPC: SendChunks" do
    it "Tests client streaming RPC where client closes stream after sending 3 messages instead of the expected 5. Server should gracefully handle partial stream." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "SendChunks",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_early_stream_close(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"session_id\":\"sess-early-001\",\"received_chunks\":3,\"expected_chunks\":5,\"status\":\"INCOMPLETE\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
