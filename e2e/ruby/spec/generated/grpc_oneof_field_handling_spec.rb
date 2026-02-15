  describe "gRPC: ProcessOneof" do
    it "Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"oneof-test-001\",\"text_data\":\"This is text data\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.OneofService",
        method_name: "ProcessOneof",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_oneof_field_handling(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"received_type\":\"text_data\",\"data_present\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
