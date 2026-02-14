  describe "gRPC: Check" do
    it "Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"value\":1000}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.RangeService",
        method_name: "Check",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_out_of_range_status_11(request)

      # Verify response
      expect(response.status_code).to eq("OUT_OF_RANGE")
      expect(response.metadata).not_to be_nil
    end
  end
