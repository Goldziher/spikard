  describe "gRPC: StreamLargeMessages" do
    it "Tests server streaming RPC with large message payloads (approximately 1MB each). Validates that the streaming framework can handle large individual messages without truncation or memory issues." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"large-stream-001\",\"message_count\":3}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamLargeMessages",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_1mb_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"3 large messages streamed successfully\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
