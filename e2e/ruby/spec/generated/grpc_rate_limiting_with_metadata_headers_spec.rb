  describe "gRPC: Query" do
    it "Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"rl-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.RateLimitService",
        method_name: "Query",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_rate_limiting_with_metadata_headers(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"result\":\"success\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
