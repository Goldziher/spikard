  describe "gRPC: Trace" do
    it "Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "x-request-id" => "req-12345-67890",
      }

      # Build request payload
      request_payload = "{\"operation\":\"trace_test\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.TracingService",
        method_name: "Trace",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_request_id_for_distributed_tracing(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"req-12345-67890\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
