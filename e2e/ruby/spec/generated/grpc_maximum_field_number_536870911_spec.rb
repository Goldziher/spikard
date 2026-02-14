  describe "gRPC: TestMaxField" do
    it "Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":42,\"max_field_value\":\"Testing maximum field number\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MaxFieldService",
        method_name: "TestMaxField",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_maximum_field_number_536870911(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":42,\"received_max\":\"Testing maximum field number\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
