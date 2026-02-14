  describe "gRPC: AllocateMemory" do
    it "Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.)." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"size\":9999999999}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.ResourceService",
        method_name: "AllocateMemory",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_resource_exhausted_status_8(request)

      # Verify response
      expect(response.status_code).to eq("RESOURCE_EXHAUSTED")
      expect(response.metadata).not_to be_nil
    end
  end
