# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "static_files" do
  it "Static file server returns text file" do
    app = E2ERubyApp.create_app_static_files_1_static_file_server_returns_text_file
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/public/hello.txt")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("Hello from static storage\n")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["cache-control"]).to eq("public, max-age=60")
    expect(response_headers["content-type"]).to eq("text/plain")
    client.close
  end

  it "Static server returns index.html for directory" do
    app = E2ERubyApp.create_app_static_files_2_static_server_returns_index_html_for_directory
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/app/")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("<!doctype html><h1>Welcome</h1>\n")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("text/html")
    client.close
  end

end
