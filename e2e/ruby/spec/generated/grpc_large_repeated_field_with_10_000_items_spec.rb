  describe "gRPC: IngestTimeSeries" do
    it "Tests handling of repeated fields containing thousands of elements. Validates efficient serialization and deserialization of large arrays without memory bloat." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"series_id\":\"metrics-large-series\",\"data_points\":[{\"timestamp\":1000000,\"value\":42.5},{\"timestamp\":1000001,\"value\":43.2},{\"timestamp\":1000002,\"value\":41.8}]}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MetricsService",
        method_name: "IngestTimeSeries",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_large_repeated_field_with_10_000_items(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"series_id\":\"metrics-large-series\",\"point_count\":10000,\"min_value\":10.5,\"max_value\":99.9}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
