  describe "gRPC: Query" do
    it "Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"circuit-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.DownstreamService",
        method_name: "Query",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_circuit_breaker_triggered_unavailable_with_metadata(request)

      # Verify response
      expect(response.status_code).to eq("UNAVAILABLE")
      expect(response.metadata).not_to be_nil
    end
  end
