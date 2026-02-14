  describe "gRPC: ProcessPayloads" do
    it "Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "grpc-max-message-size" => "4096",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.PayloadService",
        method_name: "ProcessPayloads",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_message_size_limit_exceeded(request)

      # Verify response
      expect(response.status_code).to eq("RESOURCE_EXHAUSTED")
      expect(response.payload).to eq("{\"message_id\":\"payload-002\",\"processed_count\":1,\"status\":\"FAILED\",\"error_detail\":\"Message payload size 10240 exceeds maximum allowed size 4096\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
