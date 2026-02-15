  describe "gRPC: StreamData" do
    it "Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"stream_id\":\"stream-001\",\"fail_after\":5}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamData",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_mid_stream_error(request)

      # Verify response
      expect(response.status_code).to eq("INTERNAL")
      expect(response.metadata).not_to be_nil
    end
  end
