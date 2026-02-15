  describe "gRPC: ListItems" do
    it "Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"offset\":0,\"limit\":10}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "ListItems",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_10_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"10 messages streamed successfully\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
