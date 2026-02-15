  describe "gRPC: RegisterPerson" do
    it "Tests deeply nested protobuf messages with complex hierarchies. Validates that nested message serialization handles proper field numbering and recursive structures." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"person\":{\"name\":\"John Doe\",\"address\":{\"street\":\"123 Main St\",\"city\":\"Springfield\",\"zip\":\"12345\"},\"company\":{\"name\":\"Tech Corp\",\"address\":{\"street\":\"456 Tech Ave\",\"city\":\"Silicon Valley\",\"zip\":\"94025\"},\"employee_count\":500}}}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.RegistryService",
        method_name: "RegisterPerson",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_deeply_nested_large_structure(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"success\":true,\"person\":{\"name\":\"John Doe\",\"address\":{\"street\":\"123 Main St\",\"city\":\"Springfield\"}}}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
