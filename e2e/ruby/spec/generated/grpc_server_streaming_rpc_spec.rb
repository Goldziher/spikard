  describe "gRPC: ListItems" do
    it "Tests server streaming where the server sends multiple responses. Covers streaming response patterns." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"category_id\":5,\"limit\":100}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ItemService",
        method_name: "ListItems",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_server_streaming_rpc(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.metadata).not_to be_nil
    end
  end
