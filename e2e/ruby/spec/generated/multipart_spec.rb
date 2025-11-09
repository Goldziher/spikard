# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "multipart" do
  it "17_file_magic_number_png_success" do
    app = E2ERubyApp.create_app_multipart_1_17_file_magic_number_png_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"image" => ["test.png", "", "image/png"]})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "18_file_magic_number_jpeg_success" do
    app = E2ERubyApp.create_app_multipart_2_18_file_magic_number_jpeg_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"image" => ["test.jpg", "", "image/jpeg"]})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "19_file_mime_spoofing_png_as_jpeg" do
    app = E2ERubyApp.create_app_multipart_3_19_file_mime_spoofing_png_as_jpeg
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"image" => ["fake.jpg", "", "image/jpeg"]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["files", "image"])
    client.close
  end

  it "20_file_mime_spoofing_jpeg_as_png" do
    app = E2ERubyApp.create_app_multipart_4_20_file_mime_spoofing_jpeg_as_png
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"image" => ["fake.png", "", "image/png"]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["files", "image"])
    client.close
  end

  it "21_file_pdf_magic_number_success" do
    app = E2ERubyApp.create_app_multipart_5_21_file_pdf_magic_number_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"document" => ["test.pdf", "", "application/pdf"]})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "22_file_empty_buffer" do
    app = E2ERubyApp.create_app_multipart_6_22_file_empty_buffer
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/upload", files: {"file" => ["empty.txt", "", "text/plain"]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["files", "file"])
    client.close
  end

  it "Content-Type validation - invalid type" do
    app = E2ERubyApp.create_app_multipart_7_content_type_validation_invalid_type
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/images-only", files: {"file" => ["script.sh", "#!/bin/bash\necho hello", "application/x-sh"]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body.keys).to include('errors').or include('detail')
    client.close
  end

  it "Empty file upload" do
    app = E2ERubyApp.create_app_multipart_8_empty_file_upload
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/upload", files: {"file" => ["empty.txt", "", "text/plain"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"filename" => "empty.txt", "size" => 0})
    client.close
  end

  it "File list upload (array of files)" do
    app = E2ERubyApp.create_app_multipart_9_file_list_upload_array_of_files
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/list", files: {"files" => [["file1.txt", "content of file 1", "text/plain"], ["file2.txt", "content of file 2", "text/plain"]]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"filenames" => ["file1.txt", "file2.txt"], "total_size" => 35})
    client.close
  end

  it "File size validation - too large" do
    app = E2ERubyApp.create_app_multipart_10_file_size_validation_too_large
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/validated", files: {"file" => ["large.txt", "x", "text/plain"]})
    expect(response.status_code).to eq(413)
    expect(response.json).to eq({"detail" => "File too large. Maximum size is 1MB"})
    client.close
  end

  it "File upload with custom headers" do
    app = E2ERubyApp.create_app_multipart_11_file_upload_with_custom_headers
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"test2" => ["test2.txt", "<file2 content>", "text/plain"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"test2" => {"content" => "<file2 content>", "content_type" => "text/plain", "filename" => "test2.txt", "headers" => [["content-disposition", "form-data; name=\"test2\"; filename=\"test2.txt\""], ["content-type", "text/plain"], ["x-custom", "f2"]], "size" => 15}})
    client.close
  end

  it "File upload without filename" do
    app = E2ERubyApp.create_app_multipart_12_file_upload_without_filename
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"test1" => ["file.txt", "<file1 content>"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"test1" => "<file1 content>"})
    client.close
  end

  it "Form data without files" do
    app = E2ERubyApp.create_app_multipart_13_form_data_without_files
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", data: {"some" => "data"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"some" => "data"})
    client.close
  end

  it "Image file upload" do
    app = E2ERubyApp.create_app_multipart_14_image_file_upload
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/image", files: {"image" => ["photo.jpg", "fake_jpeg_content_here", "image/jpeg"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"content_type" => "image/jpeg", "filename" => "photo.jpg", "size" => 22})
    client.close
  end

  it "Mixed files and form data" do
    app = E2ERubyApp.create_app_multipart_15_mixed_files_and_form_data
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"file" => ["upload.txt", "file data here", "text/plain"]}, data: {"active" => "true", "age" => "25", "username" => "testuser"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"active" => "true", "age" => "25", "file" => {"content" => "file data here", "content_type" => "text/plain", "filename" => "upload.txt", "size" => 14}, "username" => "testuser"})
    client.close
  end

  it "Multiple file uploads" do
    app = E2ERubyApp.create_app_multipart_16_multiple_file_uploads
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"test1" => ["test1.txt", "<file1 content>", "text/plain"], "test2" => ["test2.txt", "<file2 content>", "text/plain"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"test1" => {"content" => "<file1 content>", "content_type" => "text/plain", "filename" => "test1.txt", "size" => 15}, "test2" => {"content" => "<file2 content>", "content_type" => "text/plain", "filename" => "test2.txt", "size" => 15}})
    client.close
  end

  it "Multiple values for same field name" do
    app = E2ERubyApp.create_app_multipart_17_multiple_values_for_same_field_name
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"files" => [["file1.txt", "first file", "text/plain"], ["file2.txt", "second file", "text/plain"]]}, data: {"tags" => ["python", "rust", "web"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"files" => [{"content" => "first file", "content_type" => "text/plain", "filename" => "file1.txt", "size" => 10}, {"content" => "second file", "content_type" => "text/plain", "filename" => "file2.txt", "size" => 11}], "tags" => ["python", "rust", "web"]})
    client.close
  end

  it "Optional file upload - missing" do
    app = E2ERubyApp.create_app_multipart_18_optional_file_upload_missing
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/optional", data: {})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"file" => nil})
    client.close
  end

  it "Optional file upload - provided" do
    app = E2ERubyApp.create_app_multipart_19_optional_file_upload_provided
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/optional", files: {"file" => ["optional.txt", "optional file content", "text/plain"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"content_type" => "text/plain", "filename" => "optional.txt", "size" => 21})
    client.close
  end

  it "PDF file upload" do
    app = E2ERubyApp.create_app_multipart_20_pdf_file_upload
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/document", files: {"document" => ["report.pdf", "fake_pdf_content", "application/pdf"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"content_type" => "application/pdf", "filename" => "report.pdf", "size" => 16})
    client.close
  end

  it "Required file upload - missing" do
    app = E2ERubyApp.create_app_multipart_21_required_file_upload_missing
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/files/required", data: {})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "file"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Simple file upload" do
    app = E2ERubyApp.create_app_multipart_22_simple_file_upload
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/", files: {"test" => ["test.txt", "<file content>", "text/plain"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"test" => {"content" => "<file content>", "content_type" => "text/plain", "filename" => "test.txt", "size" => 14}})
    client.close
  end

end
