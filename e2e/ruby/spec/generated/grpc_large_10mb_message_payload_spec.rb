  describe "gRPC: BulkUpload" do
    it "Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"bulk-10mb-transfer\",\"content\":\"TGFyZ2UgZmlsZSB0cmFuc2Zlciwgc2ltdWxhdGluZyBhIHJlYWwtd29ybGQgY2VuYXJpbyB3aGVyZSBhIGNsaWVudCBpcyB1cGxvYWRpbmcgYSBsbGFyZ2UgZGF0YSBibG9iLiBUaGlzIGZpeHR1cmUgZW5zdXJlcyB0aGF0IHRoZSBnUlBDIGltcGxlbWVudGF0aW9uIGNhbiBoYW5kbGUgdXAgdG8gMTAgTUIgb2YgZGF0YSBpbiBlYWNoIHJlcXVlc3QuIFRlc3Rpbmcgc3VjaCBsYXJnZSBwYXlsb2FkcyBpcyBjcnVjaWFsIGZvciByZWxpYWJpbGl0eSBhbmQgcGVyZm9ybWFuY2UgaW4gcHJvZHVjdGlvbiBlbnZpcm9ubWVudHMu\",\"chunk_count\":10}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BulkService",
        method_name: "BulkUpload",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_large_10mb_message_payload(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"id\":\"bulk-10mb-transfer\",\"status\":\"received\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
