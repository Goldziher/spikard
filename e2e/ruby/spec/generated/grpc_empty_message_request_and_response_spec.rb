  describe "gRPC: Ping" do
    it "Tests handling of empty protobuf messages with no fields. Validates that the protocol correctly handles minimal payloads." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.PingService",
        method_name: "Ping",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_empty_message_request_and_response(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
