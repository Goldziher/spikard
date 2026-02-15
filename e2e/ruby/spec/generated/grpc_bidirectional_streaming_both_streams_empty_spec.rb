  describe "gRPC: Empty" do
    it "Tests bidirectional streaming RPC where both request and response streams are empty." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.EmptyBothService",
        method_name: "Empty",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_both_streams_empty(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.metadata).not_to be_nil
    end
  end
