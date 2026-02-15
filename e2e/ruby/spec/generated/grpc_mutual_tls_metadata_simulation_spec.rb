  describe "gRPC: VerifyClient" do
    it "Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
        "x-client-cert-cn" => "client.example.com",
        "x-client-cert-fingerprint" => "AB:CD:EF:12:34:56:78:90",
      }

      # Build request payload
      request_payload = "{\"operation\":\"secure_read\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.MtlsService",
        method_name: "VerifyClient",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_mutual_tls_metadata_simulation(request)

      # Verify response
      expect(response.status_code).to eq("OK")
      expect(response.payload).to eq("{\"verified\":true,\"client_cn\":\"client.example.com\"}".to_json)
      expect(response.metadata).not_to be_nil
    end
  end
