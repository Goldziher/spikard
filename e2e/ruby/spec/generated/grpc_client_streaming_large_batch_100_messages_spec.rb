  describe "gRPC: ProcessBatch" do
    it "Tests client streaming RPC with 100 messages in the stream. Validates performance with large batch aggregation." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BatchService",
        method_name: "ProcessBatch",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_large_batch_100_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"batch_id\":\"batch-large-001\",\"total_items\":100,\"total_value\":5050,\"average_value\":50.5,\"status\":\"PROCESSED\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
