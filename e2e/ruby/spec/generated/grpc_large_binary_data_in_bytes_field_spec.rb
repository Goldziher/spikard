  describe "gRPC: UploadBinary" do
    it "Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"file_id\":\"binary-large-001\",\"content_type\":\"application/octet-stream\",\"binary_content\":\"/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAIBAQIBAQICAgICAgICAwUDAwwDAwYEBAMFBwYHBwcGBwcICQsJCAgKCAcHCg0KCgsMDAwMBwkODw0MDgsMDAz/2wBDAQICAgMDAwYDAwYMCAcIDAwIDAwYDAwMDAwYDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAz/wAARCAABAAEDASIAAhEBAxEB/8QAFAABAAAAAAAAAAAAAAAAAAAAA//EABQAQQAAAAAAAAAAAAAAAAAAAAr/xAAUAQEBAAAAAAAAAAAAAAAAAAAAAv/EABERAAAAAAAAAAAAAAAAAAAAAf/aAAwDAQACEQMRAD8AwA8A/9k=\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BinaryService",
        method_name: "UploadBinary",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_large_binary_data_in_bytes_field(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"file_id\":\"binary-large-001\",\"bytes_received\":512000}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
