  describe "gRPC: Create" do
    it "Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"duplicate-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.CreateService",
        method_name: "Create",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_already_exists_status_6(request)

      # Verify response
      expect(response.status_code).to eq("ALREADY_EXISTS")
      expect(response.metadata).not_to be_nil
    end
  end
