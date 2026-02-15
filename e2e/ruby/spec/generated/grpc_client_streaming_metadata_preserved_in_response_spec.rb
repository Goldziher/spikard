  describe "gRPC: ProcessWithMetadata" do
    it "Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline." do
      # Build gRPC request from fixture
      metadata = {
        "authorization" => "Bearer token-xyz123",
        "x-user-id" => "user-789",
        "x-trace-id" => "trace-abc456",
        "content-type" => "application/grpc",
        "custom-header" => "custom-value",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MetadataService",
        method_name: "ProcessWithMetadata",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_metadata_preserved_in_response(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"req-meta-001\",\"processed_by\":\"grpc-handler-1\",\"received_user_id\":\"user-789\",\"message_count\":3,\"trace_id\":\"trace-abc456\",\"status\":\"COMPLETE_WITH_METADATA\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
