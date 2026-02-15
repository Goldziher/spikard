  describe "gRPC: RetryableOperation" do
    it "Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"retry-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.RetryService",
        method_name: "RetryableOperation",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_timeout_with_retry_metadata(request)

      # Verify response
      expect(response.status_code).to eq("DEADLINE_EXCEEDED")
      expect(response.metadata).not_to be_nil
    end
  end
