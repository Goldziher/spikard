  describe "gRPC: SlowOp" do
    it "Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"timeout-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.TimeoutService",
        method_name: "SlowOp",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_deadline_exceeded_status_4(request)

      # Verify response
      expect(response.status_code).to eq("DEADLINE_EXCEEDED")
      expect(response.metadata).not_to be_nil
    end
  end
