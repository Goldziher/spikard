  describe "gRPC: CheckMetadata" do
    it "Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers." do
      # Build gRPC request from fixture
      metadata = {
        "x-trace-id" => "trace-abc123def456",
        "content-type" => "application/grpc",
        "authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        "x-custom-header" => "custom-value",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"req-987654321\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MetadataService",
        method_name: "CheckMetadata",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_metadata_headers(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"req-987654321\",\"received_auth_header\":\"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\",\"received_trace_id\":\"trace-abc123def456\",\"received_custom_header\":\"custom-value\",\"response_time_ms\":45}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
