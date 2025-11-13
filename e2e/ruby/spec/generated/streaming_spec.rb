# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "streaming" do
  it "Binary log download" do
    app = E2ERubyApp.create_app_streaming_1_binary_log_download
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/stream/logfile")
    expect(response.status_code).to eq(200)
    expected_body = "LOG:\x00\x01\x02\x03|TAIL|\x07\\n"
    expect(response.body_bytes).to eq(expected_body)
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/octet-stream")
    client.close
  end

  it "Chunked CSV export" do
    app = E2ERubyApp.create_app_streaming_2_chunked_csv_export
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/stream/csv-report")
    expect(response.status_code).to eq(200)
    expected_body = "id,name,value\\n1,Alice,42\\n2,Bob,7\\n"
    expect(response.body_bytes).to eq(expected_body)
    expect(response.text).to eq(expected_body)
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("text/csv")
    client.close
  end

  it "Stream JSON lines" do
    app = E2ERubyApp.create_app_streaming_3_stream_json_lines
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/stream/json-lines")
    expect(response.status_code).to eq(200)
    expected_body = "{\"index\":0,\"payload\":\"alpha\"}\\n{\"index\":1,\"payload\":\"beta\"}\\n{\"index\":2,\"payload\":\"gamma\"}\\n"
    expect(response.body_bytes).to eq(expected_body)
    expect(response.text).to eq(expected_body)
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/x-ndjson")
    client.close
  end

end
