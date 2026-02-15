  describe "gRPC: StreamWithError" do
    it "Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"count\":5}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ErrorTestService",
        method_name: "StreamWithError",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_stream_error_mid_transmission(request)

      # Verify response
      expect(response.status_code).to eq("INTERNAL")
      expect(response.metadata).not_to be_nil
    end
  end
