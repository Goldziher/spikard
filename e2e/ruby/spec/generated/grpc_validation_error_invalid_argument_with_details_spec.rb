  describe "gRPC: ValidateInput" do
    it "Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"email\":\"invalid-email\",\"age\":-5}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ValidationService",
        method_name: "ValidateInput",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_validation_error_invalid_argument_with_details(request)

      # Verify response
      expect(response.status_code).to eq("INVALID_ARGUMENT")
      expect(response.metadata).not_to be_nil
    end
  end
