  describe "gRPC: Commit" do
    it "Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"txn-conflict\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.TransactionService",
        method_name: "Commit",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_aborted_status_10(request)

      # Verify response
      expect(response.status_code).to eq("ABORTED")
      expect(response.metadata).not_to be_nil
    end
  end
