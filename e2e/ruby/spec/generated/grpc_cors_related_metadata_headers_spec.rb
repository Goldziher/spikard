  describe "gRPC: CheckCors" do
    it "Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "access-control-request-method" => "POST",
        "origin" => "https://example.com",
      }

      # Build request payload
      request_payload = "{\"resource\":\"data\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.CorsService",
        method_name: "CheckCors",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_cors_related_metadata_headers(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"allowed\":true}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
