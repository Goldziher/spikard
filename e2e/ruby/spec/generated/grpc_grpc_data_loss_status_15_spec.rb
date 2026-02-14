  describe "gRPC: Process" do
    it "Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred." do
      # Build gRPC request from fixture
      metadata = {
        "content-type" => "application/grpc",
      }

      # Build request payload
      request_payload = "{\"id\":\"dataloss-001\"}".to_json

      request = Spikard::Grpc::Request.new(
        service_name: "example.v1.DataService",
        method_name: "Process",
        payload: request_payload,
        metadata: metadata
      )

      # Call handler
      response = handle_grpc_grpc_data_loss_status_15(request)

      # Verify response
      expect(response.status_code).to eq("DATA_LOSS")
      expect(response.metadata).not_to be_nil
    end
  end
