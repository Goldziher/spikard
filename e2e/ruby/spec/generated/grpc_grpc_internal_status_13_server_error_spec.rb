  describe "gRPC: Fail" do
    it "Tests INTERNAL gRPC status code. Returned when an internal server error occurs." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"internal-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.InternalService",
        method_name: "Fail",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_internal_status_13_server_error(request)

      # Verify response
      expect(response.status_code).to eq("INTERNAL")
      expect(response.metadata).not_to be_nil
    end
  end
