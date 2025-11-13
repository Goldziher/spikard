# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "json_bodies" do
  it "29_nested_object_validation_success" do
    app = E2ERubyApp.create_app_json_bodies_1_29_nested_object_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"profile" => {"email" => "john@example.com", "name" => "John Doe"}})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "30_nested_object_missing_field" do
    app = E2ERubyApp.create_app_json_bodies_2_30_nested_object_missing_field
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"profile" => {"name" => "John Doe"}})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "profile", "email"])
    client.close
  end

  it "31_nullable_property_null_value" do
    app = E2ERubyApp.create_app_json_bodies_3_31_nullable_property_null_value
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"description" => nil, "name" => "Test User"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "32_schema_ref_definitions" do
    app = E2ERubyApp.create_app_json_bodies_4_32_schema_ref_definitions
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/products", json: {"product" => {"name" => "Widget", "price" => 9.99}})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "33_allof_schema_composition" do
    app = E2ERubyApp.create_app_json_bodies_5_33_allof_schema_composition
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items", json: {"name" => "Product", "price" => 29.99})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "34_additional_properties_false" do
    app = E2ERubyApp.create_app_json_bodies_6_34_additional_properties_false
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"email" => "john@example.com", "extra_field" => "should fail", "name" => "John"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "extra_field"])
    client.close
  end

  it "35_oneof_schema_success" do
    app = E2ERubyApp.create_app_json_bodies_7_35_oneof_schema_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/payment", json: {"credit_card" => "1234567812345678"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "36_oneof_schema_multiple_match_failure" do
    app = E2ERubyApp.create_app_json_bodies_8_36_oneof_schema_multiple_match_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/payment", json: {"credit_card" => "1234567812345678", "paypal_email" => "user@example.com"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "37_oneof_schema_no_match_failure" do
    app = E2ERubyApp.create_app_json_bodies_9_37_oneof_schema_no_match_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/payment", json: {"bitcoin_address" => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "38_anyof_schema_success" do
    app = E2ERubyApp.create_app_json_bodies_10_38_anyof_schema_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/contact", json: {"email" => "john@example.com", "name" => "John Doe"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "39_anyof_schema_multiple_match_success" do
    app = E2ERubyApp.create_app_json_bodies_11_39_anyof_schema_multiple_match_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/contact", json: {"email" => "john@example.com", "name" => "John Doe", "phone" => "+1-555-0100"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "40_anyof_schema_failure" do
    app = E2ERubyApp.create_app_json_bodies_12_40_anyof_schema_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/contact", json: {"name" => "John Doe"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "41_not_schema_success" do
    app = E2ERubyApp.create_app_json_bodies_13_41_not_schema_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"username" => "john_doe"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "42_not_schema_failure" do
    app = E2ERubyApp.create_app_json_bodies_14_42_not_schema_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", json: {"username" => "admin"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    client.close
  end

  it "43_const_validation_success" do
    app = E2ERubyApp.create_app_json_bodies_15_43_const_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/v1/data", json: {"data" => "test", "version" => "1.0"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "44_const_validation_failure" do
    app = E2ERubyApp.create_app_json_bodies_16_44_const_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/v1/data", json: {"data" => "test", "version" => "2.0"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "version"])
    client.close
  end

  it "45_minproperties_validation_success" do
    app = E2ERubyApp.create_app_json_bodies_17_45_minproperties_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/config", json: {"host" => "localhost", "port" => 8080})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "46_minproperties_validation_failure" do
    app = E2ERubyApp.create_app_json_bodies_18_46_minproperties_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/config", json: {"host" => "localhost"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "47_maxproperties_validation_failure" do
    app = E2ERubyApp.create_app_json_bodies_19_47_maxproperties_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/config", json: {"debug" => false, "host" => "localhost", "port" => 8080, "ssl" => true})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "48_dependencies_validation_success" do
    app = E2ERubyApp.create_app_json_bodies_20_48_dependencies_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/billing", json: {"billing_address" => "123 Main St", "credit_card" => "1234567812345678", "name" => "John Doe"})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "49_dependencies_validation_failure" do
    app = E2ERubyApp.create_app_json_bodies_21_49_dependencies_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/billing", json: {"credit_card" => "1234567812345678", "name" => "John Doe"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body"])
    client.close
  end

  it "50_deep_nesting_4_levels" do
    app = E2ERubyApp.create_app_json_bodies_22_50_deep_nesting_4_levels
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", json: {"user" => {"profile" => {"contact" => {"address" => {"street" => "123 Main St"}}}}})
    expect(response.status_code).to eq(201)
    expect(response.body_text).to be_nil
    client.close
  end

  it "Array of objects - success" do
    app = E2ERubyApp.create_app_json_bodies_23_array_of_objects_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/list", headers: {"Content-Type" => "application/json"}, json: {"images" => [{"name" => "Front", "url" => "https://example.com/img1.jpg"}, {"name" => "Back", "url" => "https://example.com/img2.jpg"}], "name" => "Product Bundle", "tags" => ["electronics", "gadget"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"images" => [{"name" => "Front", "url" => "https://example.com/img1.jpg"}, {"name" => "Back", "url" => "https://example.com/img2.jpg"}], "name" => "Product Bundle", "tags" => ["electronics", "gadget"]})
    client.close
  end

  it "Array of primitive values" do
    app = E2ERubyApp.create_app_json_bodies_24_array_of_primitive_values
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Product", "ratings" => [4.5, 4.8, 5.0, 4.2], "tags" => ["electronics", "gadget", "new"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Product", "ratings" => [4.5, 4.8, 5.0, 4.2], "tags" => ["electronics", "gadget", "new"]})
    client.close
  end

  it "Body with query parameters" do
    app = E2ERubyApp.create_app_json_bodies_25_body_with_query_parameters
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/?limit=10", query: {"limit" => 10}, headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 42.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item" => {"name" => "Item", "price" => 42.0}, "limit" => 10})
    client.close
  end

  it "Boolean field - success" do
    app = E2ERubyApp.create_app_json_bodies_26_boolean_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"in_stock" => true, "name" => "Item", "price" => 42.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"in_stock" => true, "name" => "Item", "price" => 42.0})
    client.close
  end

  it "Date field - success" do
    app = E2ERubyApp.create_app_json_bodies_27_date_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/events/", headers: {"Content-Type" => "application/json"}, json: {"event_date" => "2024-03-15", "name" => "Conference"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"event_date" => "2024-03-15", "name" => "Conference"})
    client.close
  end

  it "Datetime field - success" do
    app = E2ERubyApp.create_app_json_bodies_28_datetime_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/events/", headers: {"Content-Type" => "application/json"}, json: {"created_at" => "2024-03-15T10:30:00Z", "name" => "Meeting"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"created_at" => "2024-03-15T10:30:00Z", "name" => "Meeting"})
    client.close
  end

  it "Deeply nested objects" do
    app = E2ERubyApp.create_app_json_bodies_29_deeply_nested_objects
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/nested", headers: {"Content-Type" => "application/json"}, json: {"name" => "Product", "price" => 100.0, "seller" => {"address" => {"city" => "Springfield", "country" => {"code" => "US", "name" => "USA"}, "street" => "123 Main St"}, "name" => "John Doe"}})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Product", "price" => 100.0, "seller" => {"address" => {"city" => "Springfield", "country" => {"code" => "US", "name" => "USA"}, "street" => "123 Main St"}, "name" => "John Doe"}})
    client.close
  end

  it "Empty JSON object" do
    app = E2ERubyApp.create_app_json_bodies_30_empty_json_object
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/optional-all", headers: {"Content-Type" => "application/json"}, json: {})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => nil, "name" => nil, "price" => nil, "tax" => nil})
    client.close
  end

  it "Empty array validation - fail" do
    app = E2ERubyApp.create_app_json_bodies_31_empty_array_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/list-validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "Product", "tags" => []})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "tags"])
    expect(body['errors'].first['type']).to eq("too_short")
    client.close
  end

  it "Enum field - invalid value" do
    app = E2ERubyApp.create_app_json_bodies_32_enum_field_invalid_value
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"category" => "furniture", "name" => "Item"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "category"])
    expect(body['errors'].first['type']).to eq("enum")
    client.close
  end

  it "Enum field - success" do
    app = E2ERubyApp.create_app_json_bodies_33_enum_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"category" => "electronics", "name" => "Item"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"category" => "electronics", "name" => "Item"})
    client.close
  end

  it "Extra fields ignored (no additionalProperties)" do
    app = E2ERubyApp.create_app_json_bodies_34_extra_fields_ignored_no_additionalproperties
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"another_extra" => 123, "extra_field" => "this should be ignored", "name" => "Item", "price" => 42.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Item", "price" => 42.0})
    client.close
  end

  it "Field type validation - invalid type" do
    app = E2ERubyApp.create_app_json_bodies_35_field_type_validation_invalid_type
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"description" => "A very nice Item", "name" => "Foo", "price" => "not a number", "tax" => 3.2})
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

  it "Nested object - success" do
    app = E2ERubyApp.create_app_json_bodies_36_nested_object_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/nested", headers: {"Content-Type" => "application/json"}, json: {"image" => {"name" => "Product Image", "url" => "https://example.com/image.jpg"}, "name" => "Foo", "price" => 42.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"image" => {"name" => "Product Image", "url" => "https://example.com/image.jpg"}, "name" => "Foo", "price" => 42.0})
    client.close
  end

  it "Null value for optional field" do
    app = E2ERubyApp.create_app_json_bodies_37_null_value_for_optional_field
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"description" => nil, "name" => "Item", "price" => 42.0, "tax" => nil})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => nil, "name" => "Item", "price" => 42.0, "tax" => nil})
    client.close
  end

  it "Numeric ge validation - fail" do
    app = E2ERubyApp.create_app_json_bodies_38_numeric_ge_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 0.5})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "price"])
    expect(body['errors'].first['type']).to eq("greater_than_equal")
    client.close
  end

  it "Numeric le validation - success" do
    app = E2ERubyApp.create_app_json_bodies_39_numeric_le_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "price" => 100.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Item", "price" => 100.0})
    client.close
  end

  it "Optional fields - omitted" do
    app = E2ERubyApp.create_app_json_bodies_40_optional_fields_omitted
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"name" => "Foo", "price" => 35.4})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => nil, "name" => "Foo", "price" => 35.4, "tax" => nil})
    client.close
  end

  it "PATCH partial update" do
    app = E2ERubyApp.create_app_json_bodies_41_patch_partial_update
    client = Spikard::Testing.create_test_client(app)
    response = client.patch("/items/1", headers: {"Content-Type" => "application/json"}, json: {"price" => 45.0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "Original description", "name" => "Original Item", "price" => 45.0})
    client.close
  end

  it "Required field missing - validation error" do
    app = E2ERubyApp.create_app_json_bodies_42_required_field_missing_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"description" => "A very nice Item", "price" => 35.4})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Simple JSON object - success" do
    app = E2ERubyApp.create_app_json_bodies_43_simple_json_object_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"description" => "A very nice Item", "name" => "Foo", "price" => 35.4, "tax" => 3.2})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "A very nice Item", "name" => "Foo", "price" => 35.4, "tax" => 3.2})
    client.close
  end

  it "String max_length validation - fail" do
    app = E2ERubyApp.create_app_json_bodies_44_string_max_length_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "This is a very long name that exceeds the maximum length", "price" => 35.4})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "String min_length validation - fail" do
    app = E2ERubyApp.create_app_json_bodies_45_string_min_length_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "ab", "price" => 35.4})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "String pattern validation - fail" do
    app = E2ERubyApp.create_app_json_bodies_46_string_pattern_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "sku" => "ABC-123"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "sku"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "String pattern validation - success" do
    app = E2ERubyApp.create_app_json_bodies_47_string_pattern_validation_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/validated", headers: {"Content-Type" => "application/json"}, json: {"name" => "Item", "sku" => "ABC1234"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "Item", "sku" => "ABC1234"})
    client.close
  end

  it "UUID field - invalid format" do
    app = E2ERubyApp.create_app_json_bodies_48_uuid_field_invalid_format
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"item_id" => "not-a-valid-uuid", "name" => "Item"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "item_id"])
    expect(body['errors'].first['type']).to eq("uuid_parsing")
    client.close
  end

  it "UUID field - success" do
    app = E2ERubyApp.create_app_json_bodies_49_uuid_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/items/", headers: {"Content-Type" => "application/json"}, json: {"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716", "name" => "Item"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716", "name" => "Item"})
    client.close
  end

end
