  describe "gRPC: StreamWithDelay" do
    it "Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "grpc-timeout" => "1000m",
      }

      # Build request payload
      request_payload = "{\"delay_ms\":500,\"message_count\":10}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamWithDelay",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_timeout_scenario(request)

      # Verify response
      expect(response.status_code).to eq("DEADLINE_EXCEEDED")
      expect(response.metadata).not_to be_nil
    end
  end
