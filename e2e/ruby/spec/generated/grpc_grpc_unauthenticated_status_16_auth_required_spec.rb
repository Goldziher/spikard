  describe "gRPC: SecureOp" do
    it "Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"action\":\"sensitive\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.AuthService",
        method_name: "SecureOp",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_unauthenticated_status_16_auth_required(request)

      # Verify response
      expect(response.status_code).to eq("UNAUTHENTICATED")
      expect(response.metadata).not_to be_nil
    end
  end
