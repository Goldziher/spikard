  describe "gRPC: ProcessBatch" do
    it "Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"item_id\":\"batch-item-001\",\"payload\":\"TGFyZ2UgYmF0Y2ggaXRlbSBwYXlsb2FkIGRhdGEgZm9yIHRlc3RpbmcgY2xpZW50IHN0cmVhbWluZyBjYXBhYmlsaXRpZXMgd2l0aCBtdWx0aXBsZSBsYXJnZSByZXF1ZXN0cy4gVGhpcyBlbnN1cmVzIHRoYXQgdGhlIHNlcnZlciBjYW4gYWNjdW11bGF0ZSBhbmQgcHJvY2VzcyBzdWNoIGJhdGNoZXMgY29ycmVjdGx5Lg==\",\"sequence\":1}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.BatchService",
        method_name: "ProcessBatch",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_client_streaming_with_large_batch_requests(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"batch_id\":\"batch-large-001\",\"items_processed\":100,\"total_bytes\":5242880}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
