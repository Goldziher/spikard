  describe "gRPC: Execute" do
    it "Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated." do
      # Build gRPC request from fixture
      metadata = {
        "x-custom-auth" => "CustomScheme token_value_123",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"action\":\"execute\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.CustomAuthService",
        method_name: "Execute",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_custom_authentication_scheme_header(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"success\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
