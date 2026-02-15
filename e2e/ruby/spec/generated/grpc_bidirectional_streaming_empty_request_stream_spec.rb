  describe "gRPC: HandleEmpty" do
    it "Tests bidirectional streaming RPC with empty request stream but server sends response." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.EmptyService",
        method_name: "HandleEmpty",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_empty_request_stream(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.metadata).not_to be_nil
    end
  end
