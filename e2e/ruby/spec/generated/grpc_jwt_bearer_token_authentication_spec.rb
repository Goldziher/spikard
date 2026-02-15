  describe "gRPC: SecureAction" do
    it "Tests JWT authentication via gRPC metadata. Validates that JWT tokens are properly extracted and validated from authorization header." do
      # Build gRPC request from fixture
      metadata = {
        "authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTEyMyIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"action\":\"read\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.AuthService",
        method_name: "SecureAction",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_jwt_bearer_token_authentication(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"user_id\":\"user-123\",\"action\":\"read\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
