  describe "gRPC: ProcessWrapper" do
    it "Tests usage of google.protobuf wrapper types (StringValue, Int32Value, BoolValue) for nullable scalar types. Validates proper null/present distinction." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"wrapper-test-001\",\"optional_name\":\"Test Name\",\"optional_count\":42}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.WrapperService",
        method_name: "ProcessWrapper",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_well_known_wrapper_types_stringvalue_int32value_etc(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":\"wrapper-test-001\",\"name_present\":true,\"name_value\":\"Test Name\",\"count_present\":true,\"count_value\":42}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
