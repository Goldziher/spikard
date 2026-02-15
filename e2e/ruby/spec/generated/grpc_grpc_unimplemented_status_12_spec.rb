  describe "gRPC: NotYetImplemented" do
    it "Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"unimp-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UnimplService",
        method_name: "NotYetImplemented",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_unimplemented_status_12(request)

      # Verify response
      expect(response.status_code).to eq("UNIMPLEMENTED")
      expect(response.metadata).not_to be_nil
    end
  end
