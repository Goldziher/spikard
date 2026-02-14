  describe "gRPC: Validate" do
    it "Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"value\":-999}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ArgService",
        method_name: "Validate",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_invalid_argument_status_3(request)

      # Verify response
      expect(response.status_code).to eq("INVALID_ARGUMENT")
      expect(response.metadata).not_to be_nil
    end
  end
