  describe "gRPC: GetProduct" do
    it "Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"product_id\":-1}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ProductService",
        method_name: "GetProduct",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_error_handling_grpc_status_codes(request)

      # Verify response
      expect(response.status_code).to eq("INVALID_ARGUMENT")
      expect(response.metadata).not_to be_nil
    end
  end
