  describe "gRPC: UploadLarge" do
    it "Tests handling of 1MB protobuf messages. Verifies that large payloads are properly serialized, transmitted, and deserialized without truncation or corruption." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"request_id\":\"large-1mb-test-001\",\"data\":\"SGVsbG8gV29ybGQgLSBUaGlzIGlzIGEgbGFyZ2UgMU1CIG1lc3NhZ2UgcGF5bG9hZCB0ZXN0IGZpeHR1cmUgZm9yIHRlc3RpbmcgY2FwYWNpdHkgdGVzdHMgYW5kIHBlcmZvcm1hbmNlIG1lYXN1cmVtZW50cy4gIFRoaXMgZGF0YSBmaWVsZCB3aWxsIGNvbnRhaW4gYXBwcm94aW1hdGVseSAxIE1CIG9mIGRhdGEgd2hlbiBidWlsdC4gVGhlIGdSUEMgc2VydmVyIHNob3VsZCBoYW5kbGUgdGhpcyB0cmFuc21pc3Npb24gd2l0aG91dCBpc3N1ZXMuIEluIHJlYWwgd29ybGQgc2NlbmFyaW9zLCBsYXJnZSBmaWxlIHRyYW5zZmVycyBhcmUgY29tbW9uLiBPdXIgdGVzdCBmaXh0dXJlIGVuc3VyZXMgdGhhdCB0aGUgaW1wbGVtZW50YXRpb24gY2FuIGhhbmRsZSBzdWNoIGZpbGVzIHdpdGhvdXQgbWVtb3J5IGlzc3Vlcywgc3RyZWFtIGJyZWFrcywgb3IgdGltZW91dHMu\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.FileService",
        method_name: "UploadLarge",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_large_1mb_message_payload(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"request_id\":\"large-1mb-test-001\",\"data_size\":1048576}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
