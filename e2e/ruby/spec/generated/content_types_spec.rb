# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "content_types" do
  it "13_json_with_charset_utf16" do
    app = E2ERubyApp.create_app_content_types_1_13_json_with_charset_utf16
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", headers: {"Content-Type" => "application/json; charset=utf-16"}, json: {"value" => "test"})
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"error" => "Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported."})
    client.close
  end

  it "14_content_type_case_insensitive" do
    app = E2ERubyApp.create_app_content_types_2_14_content_type_case_insensitive
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", headers: {"Content-Type" => "APPLICATION/JSON"}, raw_body: "{\"name\":\"test\"}")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"name" => "test"})
    client.close
  end

  it "15_multipart_boundary_required" do
    app = E2ERubyApp.create_app_content_types_3_15_multipart_boundary_required
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/upload", headers: {"Content-Type" => "multipart/form-data"})
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"error" => "multipart/form-data requires \'boundary\' parameter"})
    client.close
  end

  it "16_text_plain_not_accepted" do
    app = E2ERubyApp.create_app_content_types_4_16_text_plain_not_accepted
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", headers: {"Content-Type" => "text/plain"}, raw_body: "{\"data\": \"value\"}")
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"error" => "Unsupported Media Type. Expected application/json"})
    client.close
  end

  it "17_vendor_json_accepted" do
    app = E2ERubyApp.create_app_content_types_5_17_vendor_json_accepted
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/api/v1/resource", headers: {"Content-Type" => "application/vnd.api+json"}, raw_body: "{\"data\":\"value\"}")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"data" => "value"})
    client.close
  end

  it "18_content_type_with_multiple_params" do
    app = E2ERubyApp.create_app_content_types_6_18_content_type_with_multiple_params
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", headers: {"Content-Type" => "application/json; charset=utf-8; boundary=something"}, json: {"value" => "test"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"value" => "test"})
    client.close
  end

  it "19_missing_content_type_default_json" do
    app = E2ERubyApp.create_app_content_types_7_19_missing_content_type_default_json
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", json: {"name" => "test"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"name" => "test"})
    client.close
  end

  it "20_content_length_mismatch" do
    app = E2ERubyApp.create_app_content_types_8_20_content_length_mismatch
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", headers: {"Content-Length" => "100", "Content-Type" => "application/json"}, json: {"value" => "short"})
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"error" => "Content-Length header does not match actual body size"})
    client.close
  end

  it "415 Unsupported Media Type" do
    app = E2ERubyApp.create_app_content_types_9_415_unsupported_media_type
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/items/", headers: {"Content-Type" => "application/xml"}, json: "<?xml version=\"1.0\"?><item><name>Item</name></item>")
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"detail" => "Unsupported media type"})
    client.close
  end

  it "Binary response - application/octet-stream" do
    app = E2ERubyApp.create_app_content_types_10_binary_response_application_octet_stream
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/download/file.bin")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("binary_data_placeholder")
    client.close
  end

  it "CSV response - text/csv" do
    app = E2ERubyApp.create_app_content_types_11_csv_response_text_csv
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/export/data.csv")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("id,name,price\n1,Item A,10.0\n2,Item B,20.0")
    client.close
  end

  it "Content negotiation - Accept header" do
    app = E2ERubyApp.create_app_content_types_12_content_negotiation_accept_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/accept-test/1", headers: {"Accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "name" => "Item"})
    client.close
  end

  it "HTML response - text/html" do
    app = E2ERubyApp.create_app_content_types_13_html_response_text_html
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/html")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("<html><body><h1>Hello</h1></body></html>")
    client.close
  end

  it "JPEG image response - image/jpeg" do
    app = E2ERubyApp.create_app_content_types_14_jpeg_image_response_image_jpeg
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/images/photo.jpg")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("jpeg_binary_data")
    client.close
  end

  it "JSON response - application/json" do
    app = E2ERubyApp.create_app_content_types_15_json_response_application_json
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/json")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Item", "price" => 42.0})
    client.close
  end

  it "JSON with UTF-8 charset" do
    app = E2ERubyApp.create_app_content_types_16_json_with_utf_8_charset
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/unicode")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"emoji" => "\u{2615}", "name" => "Caf\u{e9}"})
    client.close
  end

  it "PDF response - application/pdf" do
    app = E2ERubyApp.create_app_content_types_17_pdf_response_application_pdf
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/download/document.pdf")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("pdf_binary_data")
    client.close
  end

  it "PNG image response - image/png" do
    app = E2ERubyApp.create_app_content_types_18_png_image_response_image_png
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/images/logo.png")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("png_binary_data")
    client.close
  end

  it "Plain text response - text/plain" do
    app = E2ERubyApp.create_app_content_types_19_plain_text_response_text_plain
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/text")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("Hello, World!")
    client.close
  end

  it "XML response - application/xml" do
    app = E2ERubyApp.create_app_content_types_20_xml_response_application_xml
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/xml")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>")
    client.close
  end

end
