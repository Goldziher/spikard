  describe "gRPC: LogEvent" do
    it "Tests usage of google.protobuf.Timestamp and Duration types. Validates RFC 3339 timestamp serialization and duration calculations." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"event\":{\"id\":\"event-001\",\"occurred_at\":\"2024-01-15T10:30:45.123Z\",\"duration_seconds\":3600,\"duration_nanos\":500000000}}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.EventService",
        method_name: "LogEvent",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_timestamp_and_duration_well_known_types(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"event_id\":\"event-001\",\"processed_at\":\"2024-01-15T10:31:45.123Z\",\"processing_time_ms\":1000}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
