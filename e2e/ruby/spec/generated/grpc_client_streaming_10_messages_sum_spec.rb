  describe "gRPC: SumNumbers" do
    it "Tests client streaming RPC where client sends 10 integer values. Server sums all values and returns result." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MathService",
        method_name: "SumNumbers",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_10_messages_sum(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"sequence_id\":\"seq-001\",\"count\":10,\"sum\":550,\"status\":\"COMPLETE\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
