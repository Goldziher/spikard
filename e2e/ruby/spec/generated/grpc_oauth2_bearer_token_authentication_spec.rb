  describe "gRPC: CheckScope" do
    it "Tests OAuth2 Bearer token authentication. Validates token validation and scope checking." do
      # Build gRPC request from fixture
      metadata = {
        "authorization" => "Bearer ya29.a0AfH6SMBx...",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"scope\":\"read:users\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.OAuth2Service",
        method_name: "CheckScope",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_oauth2_bearer_token_authentication(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"granted\":true,\"token_info\":\"oauth2_token\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
