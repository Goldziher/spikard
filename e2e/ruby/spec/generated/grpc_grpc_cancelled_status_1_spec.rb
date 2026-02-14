  describe "gRPC: Operation" do
    it "Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"cancel-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.CancelService",
        method_name: "Operation",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_cancelled_status_1(request)

      # Verify response
      expect(response.status_code).to eq("CANCELLED")
      expect(response.payload).to eq("{\"id\":\"cancel-001\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
