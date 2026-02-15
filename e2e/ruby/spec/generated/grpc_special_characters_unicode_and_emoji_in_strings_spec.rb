  describe "gRPC: EchoSpecial" do
    it "Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"user_id\":\"user-unicode-001\",\"message\":\"Hello \u{4e16}\u{754c} \u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{645}\u{631}\u{62d}\u{628}\u{627}\",\"emoji_field\":\"\u{1f680} \u{1f389} \u{1f31f} \u{2728} \u{1f4bb}\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.EchoService",
        method_name: "EchoSpecial",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_special_characters_unicode_and_emoji_in_strings(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"echo\":\"Hello \u{4e16}\u{754c} \u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{645}\u{631}\u{62d}\u{628}\u{627}\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
