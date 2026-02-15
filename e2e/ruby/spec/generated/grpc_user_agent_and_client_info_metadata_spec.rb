  describe "gRPC: Identify" do
    it "Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "user-agent" => "grpc-client/1.2.3 (linux; amd64)",
      }

      # Build request payload
      request_payload = "{\"action\":\"identify\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ClientService",
        method_name: "Identify",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_user_agent_and_client_info_metadata(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"client_type\":\"grpc-client\",\"client_version\":\"1.2.3\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
