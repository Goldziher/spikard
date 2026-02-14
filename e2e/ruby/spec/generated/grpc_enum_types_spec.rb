  describe "gRPC: CreateOrder" do
    it "Tests enum definitions and serialization. Covers enum fields with named constants." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"product_name\":\"Laptop\",\"quantity\":2,\"priority\":\"HIGH\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.OrderService",
        method_name: "CreateOrder",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_enum_types(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":1001,\"product_name\":\"Laptop\",\"quantity\":2,\"status\":\"PENDING\",\"priority\":\"HIGH\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
