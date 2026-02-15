  describe "gRPC: ProcessZeros" do
    it "Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"int_field\":0,\"bool_field\":false,\"string_field\":\"\",\"bytes_field\":\"\",\"float_field\":0.0}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ZeroValueService",
        method_name: "ProcessZeros",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_all_fields_set_to_zero_false_empty_values(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"success\":true,\"fields_received\":5}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
