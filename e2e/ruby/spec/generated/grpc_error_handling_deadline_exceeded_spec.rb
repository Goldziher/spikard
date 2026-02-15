  describe "gRPC: SlowStream" do
    it "Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"delay_ms\":500}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "SlowStream",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_deadline_exceeded(request)

      # Verify response
      expect(response.status_code).to eq("DEADLINE_EXCEEDED")
      expect(response.metadata).not_to be_nil
    end
  end
