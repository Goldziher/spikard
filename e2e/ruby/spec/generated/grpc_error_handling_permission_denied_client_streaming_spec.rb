  describe "gRPC: UploadRestricted" do
    it "Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "UploadRestricted",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_permission_denied_client_streaming(request)

      # Verify response
      expect(response.status_code).to eq("PERMISSION_DENIED")
      expect(response.metadata).not_to be_nil
    end
  end
