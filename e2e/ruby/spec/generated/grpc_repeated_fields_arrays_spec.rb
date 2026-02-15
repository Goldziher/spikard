  describe "gRPC: CreatePost" do
    it "Tests arrays/repeated fields for primitive types and messages. Covers repeated field serialization and deserialization." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"title\":\"Getting Started with gRPC\",\"content\":\"This is a comprehensive guide to gRPC...\",\"tag_ids\":[1,2,3]}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BlogService",
        method_name: "CreatePost",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_repeated_fields_arrays(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":789,\"title\":\"Getting Started with gRPC\",\"content\":\"This is a comprehensive guide to gRPC...\",\"tags\":[{\"id\":1,\"name\":\"gRPC\"},{\"id\":2,\"name\":\"Protocol Buffers\"},{\"id\":3,\"name\":\"RPC\"}],\"categories\":[\"tutorial\",\"programming\",\"networking\"]}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
