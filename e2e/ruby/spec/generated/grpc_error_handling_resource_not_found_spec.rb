  describe "gRPC: GetResource" do
    it "Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"nonexistent-123\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "GetResource",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_resource_not_found(request)

      # Verify response
      expect(response.status_code).to eq("NOT_FOUND")
      expect(response.metadata).not_to be_nil
    end
  end
