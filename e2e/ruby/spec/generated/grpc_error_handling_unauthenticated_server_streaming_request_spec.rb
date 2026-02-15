  describe "gRPC: SecureStream" do
    it "Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"resource\":\"protected_data\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "SecureStream",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_unauthenticated_server_streaming_request(request)

      # Verify response
      expect(response.status_code).to eq("UNAUTHENTICATED")
      expect(response.metadata).not_to be_nil
    end
  end
