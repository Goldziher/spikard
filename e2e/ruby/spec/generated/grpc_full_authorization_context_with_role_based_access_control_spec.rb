  describe "gRPC: CheckAccess" do
    it "Tests complete authorization context including user roles, permissions, and resource-level access control." do
      # Build gRPC request from fixture
      metadata = {
        "authorization" => "Bearer token123",
        "x-user-roles" => "admin,editor",
        "x-user-permissions" => "read,write,delete",
        "x-user-id" => "user-admin-001",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"resource_id\":\"resource-456\",\"operation\":\"write\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.AuthzService",
        method_name: "CheckAccess",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_full_authorization_context_with_role_based_access_control(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"authorized\":true,\"message\":\"Access granted with admin privileges\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
