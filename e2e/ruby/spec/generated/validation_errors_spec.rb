# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "validation_errors" do
  it "09_multiple_validation_errors" do
    app = E2ERubyApp.create_app_validation_errors_1_09_multiple_validation_errors
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"age" => 15, "email" => "invalid-email", "name" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("3 validation errors in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "age"])
    expect(body['errors'].first['type']).to eq("greater_than_equal")
    client.close
  end

  it "10_nested_error_path" do
    app = E2ERubyApp.create_app_validation_errors_2_10_nested_error_path
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/profiles", json: {"profile" => {"contact" => {"email" => "invalid"}}})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "profile", "contact", "email"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "Array item validation error" do
    app = E2ERubyApp.create_app_validation_errors_3_array_item_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 10.0, "tags" => ["tag1", "tag2", 123]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "tags", "2"])
    expect(body['errors'].first['type']).to eq("type_error")
    client.close
  end

  it "Array max_items constraint violation" do
    app = E2ERubyApp.create_app_validation_errors_4_array_max_items_constraint_violation
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 10.0, "tags" => ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"]})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "tags"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "Array min_items constraint violation" do
    app = E2ERubyApp.create_app_validation_errors_5_array_min_items_constraint_violation
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 10.0, "tags" => []})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "tags"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "Body field type error - string for float" do
    app = E2ERubyApp.create_app_validation_errors_6_body_field_type_error_string_for_float
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => "not_a_float"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "price"])
    expect(body['errors'].first['type']).to eq("float_parsing")
    client.close
  end

  it "Header validation error" do
    app = E2ERubyApp.create_app_validation_errors_7_header_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=test")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["header", "x-token"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Invalid UUID format" do
    app = E2ERubyApp.create_app_validation_errors_8_invalid_uuid_format
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/not-a-uuid")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "item_id"])
    expect(body['errors'].first['type']).to eq("uuid_parsing")
    client.close
  end

  it "Invalid boolean value" do
    app = E2ERubyApp.create_app_validation_errors_9_invalid_boolean_value
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=test&is_active=maybe", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "is_active"])
    expect(body['errors'].first['type']).to eq("bool_parsing")
    client.close
  end

  it "Invalid datetime format" do
    app = E2ERubyApp.create_app_validation_errors_10_invalid_datetime_format
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"created_at" => "not-a-datetime", "name" => "Item", "price" => 10.0})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "created_at"])
    expect(body['errors'].first['type']).to eq("datetime_parsing")
    client.close
  end

  it "Invalid enum value" do
    app = E2ERubyApp.create_app_validation_errors_11_invalid_enum_value
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/models/invalid_model")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "model_name"])
    expect(body['errors'].first['type']).to eq("enum")
    client.close
  end

  it "Malformed JSON body" do
    app = E2ERubyApp.create_app_validation_errors_12_malformed_json_body
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: "{\"name\": \"Item\", \"price\": }")
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"detail" => "Invalid request format"})
    client.close
  end

  it "Missing required body field" do
    app = E2ERubyApp.create_app_validation_errors_13_missing_required_body_field
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item"})
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

  it "Missing required query parameter" do
    app = E2ERubyApp.create_app_validation_errors_14_missing_required_query_parameter
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "q"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Multiple validation errors" do
    app = E2ERubyApp.create_app_validation_errors_15_multiple_validation_errors
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "X", "price" => -10, "quantity" => "not_a_number"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("3 validation errors in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "Nested object validation error" do
    app = E2ERubyApp.create_app_validation_errors_16_nested_object_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Product", "price" => 10.0, "seller" => {"address" => {"city" => "SF", "zip_code" => "123"}, "name" => "Jo"}})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("3 validation errors in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "seller", "address", "city"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "Numeric constraint violation - gt (greater than)" do
    app = E2ERubyApp.create_app_validation_errors_17_numeric_constraint_violation_gt_greater_than
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=test&price=0", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "price"])
    expect(body['errors'].first['type']).to eq("greater_than")
    client.close
  end

  it "Numeric constraint violation - le (less than or equal)" do
    app = E2ERubyApp.create_app_validation_errors_18_numeric_constraint_violation_le_less_than_or_equal
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=test&limit=101", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "limit"])
    expect(body['errors'].first['type']).to eq("less_than_equal")
    client.close
  end

  it "Query param type error - string provided for int" do
    app = E2ERubyApp.create_app_validation_errors_19_query_param_type_error_string_provided_for_int
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=test&skip=not_a_number", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "skip"])
    expect(body['errors'].first['type']).to eq("int_parsing")
    client.close
  end

  it "String max_length constraint violation" do
    app = E2ERubyApp.create_app_validation_errors_20_string_max_length_constraint_violation
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "q"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "String min_length constraint violation" do
    app = E2ERubyApp.create_app_validation_errors_21_string_min_length_constraint_violation
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=ab", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "q"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "String regex pattern mismatch" do
    app = E2ERubyApp.create_app_validation_errors_22_string_regex_pattern_mismatch
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/?q=invalid!", headers: {"x-token" => "test-token"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "q"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

end
