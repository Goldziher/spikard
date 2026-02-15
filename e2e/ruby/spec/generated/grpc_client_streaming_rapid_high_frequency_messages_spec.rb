  describe "gRPC: ProcessEvents" do
    it "Tests client streaming RPC with rapid-fire message delivery. Server handles 50 messages in quick succession and returns aggregated metrics." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MetricsService",
        method_name: "ProcessEvents",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_rapid_high_frequency_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"event_id\":\"rapid-batch-001\",\"event_count\":50,\"min_value\":0.1,\"max_value\":5.0,\"avg_value\":2.55,\"throughput_mps\":500.0,\"status\":\"PROCESSED\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
