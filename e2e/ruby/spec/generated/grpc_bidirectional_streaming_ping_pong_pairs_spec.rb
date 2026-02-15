  describe "gRPC: PingPong" do
    it "Tests bidirectional streaming RPC with request-response pairs (ping-pong pattern)." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.PingService",
        method_name: "PingPong",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_bidirectional_streaming_ping_pong_pairs(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.metadata).not_to be_nil
    end
  end
