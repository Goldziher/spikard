  describe "gRPC: ValidateUsers" do
    it "Tests client streaming RPC where a message fails validation in the middle of the stream. Server rejects the stream and returns error." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ValidationService",
        method_name: "ValidateUsers",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_validation_failure_mid_stream(request)

      # Verify response
      expect(response.status_code).to eq("INVALID_ARGUMENT")
      expect(response.payload).to eq("{\"processed\":2,\"status\":\"VALIDATION_FAILED\",\"error_message\":\"Invalid email format at message index 2: invalid-email\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
