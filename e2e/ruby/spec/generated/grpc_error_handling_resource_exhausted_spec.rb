  describe "gRPC: RateLimitedChat" do
    it "Tests bidirectional streaming RPC exceeding rate limits. Expects RESOURCE_EXHAUSTED status when client attempts to send 100 messages in rapid succession, exceeding the 100 requests/second rate limit threshold." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "RateLimitedChat",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_resource_exhausted(request)

      # Verify response
      expect(response.status_code).to eq("RESOURCE_EXHAUSTED")
      expect(response.metadata).not_to be_nil
    end
  end
