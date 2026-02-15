  describe "gRPC: ProcessWithError" do
    it "Tests bidirectional streaming RPC where server returns error after processing some messages." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorService",
        method_name: "ProcessWithError",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_error_mid_stream(request)

      # Verify response
      expect(response.status_code).to eq("INTERNAL")
      expect(response.payload).to eq("\"Error after processing 2 messages\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
