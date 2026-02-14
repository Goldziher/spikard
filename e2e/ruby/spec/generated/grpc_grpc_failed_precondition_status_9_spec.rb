  describe "gRPC: Proceed" do
    it "Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"operation\":\"finalize\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StateService",
        method_name: "Proceed",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_failed_precondition_status_9(request)

      # Verify response
      expect(response.status_code).to eq("FAILED_PRECONDITION")
      expect(response.metadata).not_to be_nil
    end
  end
