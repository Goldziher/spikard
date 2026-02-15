  describe "gRPC: CheckStatus" do
    it "Tests successful gRPC response with OK status code. Validates basic request-response completion." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"status-ok-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StatusService",
        method_name: "CheckStatus",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_ok_status_0_successful_response(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"status-ok-001\",\"status\":\"success\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
