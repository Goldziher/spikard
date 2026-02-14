  describe "gRPC: StreamLargeData" do
    it "Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"stream_id\":\"stream-large-001\",\"chunk_size\":1048576}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamingService",
        method_name: "StreamLargeData",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_with_large_response_data(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"stream_id\":\"stream-large-001\",\"chunk_number\":1,\"is_final\":false}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
