  describe "gRPC: ConcatenateStrings" do
    it "Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = {}.to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.TextService",
        method_name: "ConcatenateStrings",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_unicode_string_aggregation(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"fragment_id\":\"unicode-001\",\"result\":\"Hello, \u{4e16}\u{754c}! \u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} \u{1f30d}\",\"fragment_count\":4,\"total_length\":26,\"status\":\"CONCATENATED\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
