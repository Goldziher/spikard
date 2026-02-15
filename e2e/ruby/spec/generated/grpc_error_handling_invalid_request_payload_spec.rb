  describe "gRPC: ValidateRequest" do
    it "Tests server streaming RPC with invalid request payload. Validates that INVALID_ARGUMENT status is returned when required field is missing from the request message. The server should reject the malformed payload before beginning the stream." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"limit\":10}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "ValidateRequest",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_invalid_request_payload(request)

      # Verify response
      expect(response.status_code).to eq("INVALID_ARGUMENT")
      expect(response.metadata).not_to be_nil
    end
  end
