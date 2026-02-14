  describe "gRPC: StreamPeople" do
    it "Tests server streaming RPC with complex nested message structures. Validates proper serialization and deserialization of deeply nested protobuf objects in streaming context." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"depth\":3}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.StreamService",
        method_name: "StreamPeople",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_nested_object_messages(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("\"3 people with nested objects streamed successfully\"".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
