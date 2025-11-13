# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "http_methods" do
  it "DELETE - Remove resource" do
    app = E2ERubyApp.create_app_http_methods_1_delete_remove_resource
    client = Spikard::Testing.create_test_client(app)
    response = client.delete("/items/1")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({})
    client.close
  end

  it "DELETE - Resource not found" do
    app = E2ERubyApp.create_app_http_methods_2_delete_resource_not_found
    client = Spikard::Testing.create_test_client(app)
    response = client.delete("/items/999")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({})
    client.close
  end

  it "DELETE - With response body" do
    app = E2ERubyApp.create_app_http_methods_3_delete_with_response_body
    client = Spikard::Testing.create_test_client(app)
    response = client.delete("/items/1")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "message" => "Item deleted successfully", "name" => "Deleted Item"})
    client.close
  end

  it "HEAD - Get metadata without body" do
    app = E2ERubyApp.create_app_http_methods_4_head_get_metadata_without_body
    client = Spikard::Testing.create_test_client(app)
    response = client.head("/items/1")
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-length"]).to eq("85")
    expect(response_headers["content-type"]).to eq("application/json")
    client.close
  end

  it "OPTIONS - CORS preflight request" do
    app = E2ERubyApp.create_app_http_methods_5_options_cors_preflight_request
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/items/", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-headers"]).to eq("Content-Type")
    expect(response_headers["access-control-allow-methods"]).to eq("GET, POST, PUT, DELETE, OPTIONS")
    expect(response_headers["access-control-allow-origin"]).to eq("https://example.com")
    expect(response_headers["access-control-max-age"]).to eq("86400")
    client.close
  end

  it "PATCH - Partial update" do
    app = E2ERubyApp.create_app_http_methods_6_patch_partial_update
    client = Spikard::Testing.create_test_client(app)
    response = client.patch("/items/1", headers: {"Content-Type" => "application/json"}, json: {"price" => 79.99})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "in_stock" => true, "name" => "Existing Item", "price" => 79.99})
    client.close
  end

  it "PATCH - Update multiple fields" do
    app = E2ERubyApp.create_app_http_methods_7_patch_update_multiple_fields
    client = Spikard::Testing.create_test_client(app)
    response = client.patch("/items/1", headers: {"Content-Type" => "application/json"}, json: {"in_stock" => false, "name" => "Updated Name", "price" => 89.99})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "in_stock" => false, "name" => "Updated Name", "price" => 89.99})
    client.close
  end

  it "PUT - Complete resource replacement" do
    app = E2ERubyApp.create_app_http_methods_8_put_complete_resource_replacement
    client = Spikard::Testing.create_test_client(app)
    response = client.put("/items/1", headers: {"Content-Type" => "application/json"}, json: {"description" => "Completely replaced", "id" => 1, "in_stock" => true, "name" => "Updated Item", "price" => 99.99})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "Completely replaced", "id" => 1, "in_stock" => true, "name" => "Updated Item", "price" => 99.99})
    client.close
  end

  it "PUT - Create resource if doesn\'t exist" do
    app = E2ERubyApp.create_app_http_methods_9_put_create_resource_if_doesn_t_exist
    client = Spikard::Testing.create_test_client(app)
    response = client.put("/items/999", headers: {"Content-Type" => "application/json"}, json: {"id" => 999, "name" => "New Item", "price" => 49.99})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 999, "name" => "New Item", "price" => 49.99})
    client.close
  end

  it "PUT - Idempotent operation" do
    app = E2ERubyApp.create_app_http_methods_10_put_idempotent_operation
    client = Spikard::Testing.create_test_client(app)
    response = client.put("/items/1", headers: {"Content-Type" => "application/json"}, json: {"id" => 1, "name" => "Fixed Name", "price" => 50.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "name" => "Fixed Name", "price" => 50.0})
    client.close
  end

  it "PUT - Missing required field" do
    app = E2ERubyApp.create_app_http_methods_11_put_missing_required_field
    client = Spikard::Testing.create_test_client(app)
    response = client.put("/items/1", headers: {"Content-Type" => "application/json"}, json: {"id" => 1, "name" => "Item Name"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "price"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "PUT - Validation error" do
    app = E2ERubyApp.create_app_http_methods_12_put_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.put("/items/1", headers: {"Content-Type" => "application/json"}, json: {"id" => 1, "name" => "X", "price" => -10})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("2 validation errors in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

end
