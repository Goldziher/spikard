  describe "gRPC: CreateUser" do
    it "Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"name\":\"Bob Smith\",\"email\":\"bob@example.com\",\"address\":{\"street\":\"123 Main St\",\"city\":\"Springfield\",\"zip_code\":\"12345\"}}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UserService",
        method_name: "CreateUser",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_nested_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"user_id\":456,\"name\":\"Bob Smith\",\"email\":\"bob@example.com\",\"address\":{\"street\":\"123 Main St\",\"city\":\"Springfield\",\"zip_code\":\"12345\"}}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
