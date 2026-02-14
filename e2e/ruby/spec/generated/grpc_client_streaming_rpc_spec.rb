  describe "gRPC: Upload" do
    it "Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.FileService",
        method_name: "Upload",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_rpc(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"file_id\":\"file-12345\",\"total_bytes\":57,\"status\":\"COMPLETED\",\"checksum\":\"d8e8fca2dc0f896fd7cb4cb0031ba249\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
