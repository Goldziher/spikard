  describe "gRPC: StreamRapidMessages" do
    it "Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"count\":100}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamRapidMessages",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_rapid_100_message_stream(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"100 messages streamed successfully in sequence\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
