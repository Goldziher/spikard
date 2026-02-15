  describe "gRPC: Fail" do
    it "Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"unknown-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UnknownService",
        method_name: "Fail",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_unknown_status_2(request)

      # Verify response
      expect(response.status_code).to eq("UNKNOWN")
      expect(response.metadata).not_to be_nil
    end
  end
