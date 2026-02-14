  describe "gRPC: AdminAction" do
    it "Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"action\":\"delete_all\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.SecureService",
        method_name: "AdminAction",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_permission_denied_status_7(request)

      # Verify response
      expect(response.status_code).to eq("PERMISSION_DENIED")
      expect(response.metadata).not_to be_nil
    end
  end
