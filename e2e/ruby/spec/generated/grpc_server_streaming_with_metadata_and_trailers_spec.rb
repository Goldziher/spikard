  describe "gRPC: StreamWithMetadata" do
    it "Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "x-client-version" => "1.0.0",
        "x-request-id" => "metadata-stream-001",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"metadata-stream-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamWithMetadata",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_with_metadata_and_trailers(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"Stream completed with metadata and trailers\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
