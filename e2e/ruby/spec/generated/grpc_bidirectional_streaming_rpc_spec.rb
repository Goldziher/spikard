  describe "gRPC: Chat" do
    it "Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "authorization" => "Bearer user-token",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ChatService",
        method_name: "Chat",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_rpc(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.metadata).not_to be_nil
    end
  end
