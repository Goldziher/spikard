  describe "gRPC: FetchResource" do
    it "Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients." do
      # Build gRPC request from fixture
      metadata = {
        "x-api-key" => "sk_live_abc123def456",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"resource\":\"users\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ApiService",
        method_name: "FetchResource",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_api_key_authentication(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"data\":\"resource_data\",\"client_id\":\"client-api-001\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
