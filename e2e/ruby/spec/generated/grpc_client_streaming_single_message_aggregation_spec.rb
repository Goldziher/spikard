  describe "gRPC: AggregateData" do
    it "Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.AggregateService",
        method_name: "AggregateData",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_single_message_aggregation(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"count\":1,\"total\":42,\"average\":42.0,\"status\":\"AGGREGATED\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
