  describe "gRPC: FutureFeature" do
    it "Tests unary RPC calling an unimplemented method. Validates that UNIMPLEMENTED status is returned when the server does not support the requested RPC method. This fixture ensures proper error handling for feature requests that are not yet available in the current server implementation." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"feature_name\":\"experimental_v2\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "FutureFeature",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_unimplemented_method(request)

      # Verify response
      expect(response.status_code).to eq("UNIMPLEMENTED")
      expect(response.metadata).not_to be_nil
    end
  end
