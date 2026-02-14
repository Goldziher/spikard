  describe "gRPC: UpdateProfile" do
    it "Tests optional field handling with presence semantics. Covers optional fields with and without values." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"user_id\":42,\"bio\":\"Software engineer and gRPC enthusiast\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UserService",
        method_name: "UpdateProfile",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_optional_fields(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"user_id\":42,\"username\":\"charlie_dev\",\"bio\":\"Software engineer and gRPC enthusiast\",\"updated_at\":1704067200000}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
