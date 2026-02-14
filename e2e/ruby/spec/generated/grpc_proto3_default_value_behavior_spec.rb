  describe "gRPC: CheckDefaults" do
    it "Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":1}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.DefaultService",
        method_name: "CheckDefaults",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_proto3_default_value_behavior(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":1,\"name\":\"\",\"active\":false,\"has_id\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
