  describe "gRPC: Get" do
    it "Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"resource_id\":99999}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ResourceService",
        method_name: "Get",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_not_found_status_5(request)

      # Verify response
      expect(response.status_code).to eq("NOT_FOUND")
      expect(response.metadata).not_to be_nil
    end
  end
