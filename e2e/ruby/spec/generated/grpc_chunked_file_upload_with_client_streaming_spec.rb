  describe "gRPC: ChunkedUpload" do
    it "Tests client streaming RPC for chunked file uploads. Validates that multiple message chunks are properly accumulated and processed by the server." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"file_id\":\"chunked-upload-test\",\"chunk_number\":1,\"chunk_data\":\"Q2h1bmsgMQ==\",\"is_final\":false}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StorageService",
        method_name: "ChunkedUpload",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_chunked_file_upload_with_client_streaming(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"file_id\":\"chunked-upload-test\",\"total_chunks\":5,\"total_size\":102400,\"upload_status\":\"completed\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
