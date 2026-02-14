  describe "gRPC: ProcessAny" do
    it "Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"any-test-001\",\"any_content\":\"{\\\"type_url\\\": \\\"example.v1.Container\\\", \\\"value\\\": \\\"base64encodedvalue\\\"}\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.AnyService",
        method_name: "ProcessAny",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_google_protobuf_any_type_usage(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"any-test-001\",\"type_name\":\"example.v1.Container\",\"success\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
