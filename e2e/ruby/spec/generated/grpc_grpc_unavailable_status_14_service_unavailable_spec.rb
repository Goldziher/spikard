  describe "gRPC: Request" do
    it "Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"unavail-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UnavailService",
        method_name: "Request",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_unavailable_status_14_service_unavailable(request)

      # Verify response
      expect(response.status_code).to eq("UNAVAILABLE")
      expect(response.metadata).not_to be_nil
    end
  end
