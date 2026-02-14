  describe "gRPC: GetUser" do
    it "Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern." do
      # Build gRPC request from fixture
      metadata = {
        "authorization" => "Bearer test-token",
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"user_id\":123}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.UserService",
        method_name: "GetUser",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_simple_unary_rpc_getuser(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":123,\"name\":\"Alice Johnson\",\"email\":\"alice@example.com\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
