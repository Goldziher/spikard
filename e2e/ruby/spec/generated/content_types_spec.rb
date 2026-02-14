# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "content_types" do
  it "13_json_with_charset_utf16" do
    app = E2ERubyApp.create_app_content_types_1_13_json_with_charset_utf16
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", headers: {"Content-Type" => "application/json; charset=utf-16"}, json: {"value" => "test"})
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"detail" => "Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported.", "status" => 415, "title" => "Unsupported Charset", "type" => "https://spikard.dev/errors/unsupported-charset"})
    client.close
  end

  it "14_content_type_case_insensitive" do
    app = E2ERubyApp.create_app_content_types_2_14_content_type_case_insensitive
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", headers: {"Content-Type" => "APPLICATION/JSON"}, raw_body: "{\"name\":\"test\"}")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"name" => "test"})
    client.close
  end

  it "15_multipart_boundary_required" do
    app = E2ERubyApp.create_app_content_types_3_15_multipart_boundary_required
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", headers: {"Content-Type" => "multipart/form-data"})
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"error" => "multipart/form-data requires \'boundary\' parameter"})
    client.close
  end

  it "16_text_plain_not_accepted" do
    app = E2ERubyApp.create_app_content_types_4_16_text_plain_not_accepted
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", headers: {"Content-Type" => "text/plain"}, raw_body: "{\"data\": \"value\"}")
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"detail" => "Unsupported media type", "status" => 415, "title" => "Unsupported Media Type", "type" => "https://spikard.dev/errors/unsupported-media-type"})
    client.close
  end

  it "17_vendor_json_accepted" do
    app = E2ERubyApp.create_app_content_types_5_17_vendor_json_accepted
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/v1/resource", headers: {"Content-Type" => "application/vnd.api+json"}, raw_body: "{\"data\":\"value\"}")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"data" => "value"})
    client.close
  end

  it "18_content_type_with_multiple_params" do
    app = E2ERubyApp.create_app_content_types_6_18_content_type_with_multiple_params
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", headers: {"Content-Type" => "application/json; charset=utf-8; boundary=something"}, json: {"value" => "test"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"value" => "test"})
    client.close
  end

  it "19_missing_content_type_default_json" do
    app = E2ERubyApp.create_app_content_types_7_19_missing_content_type_default_json
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", json: {"name" => "test"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"name" => "test"})
    client.close
  end

  xit "20_content_length_mismatch" do
    skip "Not supported by the Ruby in-memory client"
  end

  it "415 Unsupported Media Type" do
    app = E2ERubyApp.create_app_content_types_9_415_unsupported_media_type
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/xml"}, json: "<?xml version=\"1.0\"?><item><name>Item</name></item>")
    expect(response.status_code).to eq(415)
    expect(response.json).to eq({"detail" => "Unsupported media type", "status" => 415, "title" => "Unsupported Media Type", "type" => "https://spikard.dev/errors/unsupported-media-type"})
    client.close
  end

  it "Binary response - application/octet-stream" do
    app = E2ERubyApp.create_app_content_types_10_binary_response_application_octet_stream
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/download/file.bin")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("binary_data_placeholder")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-disposition"]).to eq("attachment; filename=file.bin")
    expect(response_headers["content-type"]).to eq("application/octet-stream")
    client.close
  end

  it "CSV response - text/csv" do
    app = E2ERubyApp.create_app_content_types_11_csv_response_text_csv
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/export/data.csv")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("id,name,price\n1,Item A,10.0\n2,Item B,20.0")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-disposition"]).to eq("attachment; filename=data.csv")
    expect(response_headers["content-type"]).to eq("text/csv; charset=utf-8")
    client.close
  end

  it "Content negotiation - Accept header" do
    app = E2ERubyApp.create_app_content_types_12_content_negotiation_accept_header
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/accept-test/1", headers: {"Accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "name" => "Item"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json")
    client.close
  end

  it "HTML response - text/html" do
    app = E2ERubyApp.create_app_content_types_13_html_response_text_html
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/html")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("<html><body><h1>Hello</h1></body></html>")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("text/html; charset=utf-8")
    client.close
  end

  it "JPEG image response - image/jpeg" do
    app = E2ERubyApp.create_app_content_types_14_jpeg_image_response_image_jpeg
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/images/photo.jpg")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("jpeg_binary_data")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("image/jpeg")
    client.close
  end

  it "JSON response - application/json" do
    app = E2ERubyApp.create_app_content_types_15_json_response_application_json
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/json")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Item", "price" => 42.0})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json")
    client.close
  end

  it "JSON with UTF-8 charset" do
    app = E2ERubyApp.create_app_content_types_16_json_with_utf_8_charset
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/unicode")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"emoji" => "\u{2615}", "name" => "Caf\u{e9}"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json; charset=utf-8")
    client.close
  end

  it "PDF response - application/pdf" do
    app = E2ERubyApp.create_app_content_types_17_pdf_response_application_pdf
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/download/document.pdf")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("pdf_binary_data")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-disposition"]).to eq("attachment; filename=document.pdf")
    expect(response_headers["content-type"]).to eq("application/pdf")
    client.close
  end

  it "PNG image response - image/png" do
    app = E2ERubyApp.create_app_content_types_18_png_image_response_image_png
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/images/logo.png")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("png_binary_data")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("image/png")
    client.close
  end

  it "Plain text response - text/plain" do
    app = E2ERubyApp.create_app_content_types_19_plain_text_response_text_plain
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/text")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("Hello, World!")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("text/plain; charset=utf-8")
    client.close
  end

  it "XML response - application/xml" do
    app = E2ERubyApp.create_app_content_types_20_xml_response_application_xml
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/xml")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to eq("<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>")
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/xml")
    client.close
  end

end
