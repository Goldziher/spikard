  describe "gRPC: ProcessMap" do
    it "Tests protobuf map fields with string keys and message values. Validates proper key-value pair serialization and access patterns." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"map-test-001\",\"data_map\":[{\"key\":\"key1\",\"value\":\"value1\"},{\"key\":\"key2\",\"value\":\"value2\"},{\"key\":\"key3\",\"value\":\"value3\"}]}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MapService",
        method_name: "ProcessMap",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_map_field_handling_map_string_message(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":\"map-test-001\",\"map_count\":3,\"keys\":[\"key1\",\"key2\",\"key3\"]}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
