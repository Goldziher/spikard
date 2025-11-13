# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "compression" do
  it "Compression - gzip applied" do
    app = E2ERubyApp.create_app_compression_1_compression_gzip_applied
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/compression/gzip", headers: {"Accept-Encoding" => "gzip"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Compressed payload", "payload" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["vary"]).to eq("Accept-Encoding")
    client.close
  end

  it "Compression - payload below min_size is not compressed" do
    app = E2ERubyApp.create_app_compression_2_compression_payload_below_min_size_is_not_compressed
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/compression/skip", headers: {"Accept-Encoding" => "gzip"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Small payload", "payload" => "tiny"})
    client.close
  end

end
