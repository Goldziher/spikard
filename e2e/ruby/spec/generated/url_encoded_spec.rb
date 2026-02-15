# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "url_encoded" do
  it "13_array_field_success" do
    app = E2ERubyApp.create_app_url_encoded_1_13_array_field_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/register", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "tags[]=python&tags[]=rust&tags[]=typescript")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"tags" => ["python", "rust", "typescript"]})
    client.close
  end

  it "14_nested_object_bracket_notation" do
    app = E2ERubyApp.create_app_url_encoded_2_14_nested_object_bracket_notation
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/profile", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"user" => {"age" => 30, "email" => "john@example.com", "name" => "John Doe"}})
    client.close
  end

  it "15_special_characters_field_names" do
    app = E2ERubyApp.create_app_url_encoded_3_15_special_characters_field_names
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/data", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "user-name=JohnDoe&contact.email=john%40example.com")
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"contact.email" => "john@example.com", "user-name" => "JohnDoe"})
    client.close
  end

  it "16_minlength_validation_failure" do
    app = E2ERubyApp.create_app_url_encoded_4_16_minlength_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/users", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "username=ab")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "17_pattern_validation_failure" do
    app = E2ERubyApp.create_app_url_encoded_5_17_pattern_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/accounts", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "account_id=INVALID123")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "account_id"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "18_integer_minimum_validation_failure" do
    app = E2ERubyApp.create_app_url_encoded_6_18_integer_minimum_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/products", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "quantity=0")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "quantity"])
    expect(body['errors'].first['type']).to eq("greater_than_equal")
    client.close
  end

  it "19_array_minitems_validation_failure" do
    app = E2ERubyApp.create_app_url_encoded_7_19_array_minitems_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/tags", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "tags[]=single")
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

  it "20_format_email_validation_failure" do
    app = E2ERubyApp.create_app_url_encoded_8_20_format_email_validation_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/subscribe", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "email=not-an-email")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "email"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "21_integer_type_coercion_failure" do
    app = E2ERubyApp.create_app_url_encoded_9_21_integer_type_coercion_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/products", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "price=not-a-number")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "price"])
    expect(body['errors'].first['type']).to eq("int_parsing")
    client.close
  end

  it "22_additional_properties_strict_failure" do
    app = E2ERubyApp.create_app_url_encoded_10_22_additional_properties_strict_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/settings", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "theme=dark&unknown_field=value")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "unknown_field"])
    client.close
  end

  it "Boolean field conversion" do
    app = E2ERubyApp.create_app_url_encoded_11_boolean_field_conversion
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"subscribe" => "true", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"subscribe" => true, "username" => "johndoe"})
    client.close
  end

  it "Empty string value" do
    app = E2ERubyApp.create_app_url_encoded_12_empty_string_value
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"description" => "", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "", "username" => "johndoe"})
    client.close
  end

  it "Multiple values for same field" do
    app = E2ERubyApp.create_app_url_encoded_13_multiple_values_for_same_field
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/tags", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"tags" => ["python", "fastapi", "web"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"tags" => ["python", "fastapi", "web"]})
    client.close
  end

  it "Numeric field type conversion" do
    app = E2ERubyApp.create_app_url_encoded_14_numeric_field_type_conversion
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"age" => "30", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"age" => 30, "username" => "johndoe"})
    client.close
  end

  it "OAuth2 password grant flow" do
    app = E2ERubyApp.create_app_url_encoded_15_oauth2_password_grant_flow
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/token", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"grant_type" => "password", "password" => "secret", "scope" => "", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"access_token" => "johndoe", "token_type" => "bearer"})
    client.close
  end

  it "Optional field missing - success" do
    app = E2ERubyApp.create_app_url_encoded_16_optional_field_missing_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/register/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"password" => "secret", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"email" => nil, "username" => "johndoe"})
    client.close
  end

  it "Pattern validation - fail" do
    app = E2ERubyApp.create_app_url_encoded_17_pattern_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/validated", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"username" => "john doe"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "Required field missing - validation error" do
    app = E2ERubyApp.create_app_url_encoded_18_required_field_missing_validation_error
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/login/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"password" => "secret"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Simple form submission - success" do
    app = E2ERubyApp.create_app_url_encoded_19_simple_form_submission_success
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/login/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"password" => "secret", "username" => "johndoe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "johndoe"})
    client.close
  end

  it "Special characters encoding" do
    app = E2ERubyApp.create_app_url_encoded_20_special_characters_encoding
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"description" => "Test & Development", "name" => "John Doe"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "Test & Development", "name" => "John Doe"})
    client.close
  end

  it "String max_length validation - fail" do
    app = E2ERubyApp.create_app_url_encoded_21_string_max_length_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/validated", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"username" => "this_is_a_very_long_username_that_exceeds_limit"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "String min_length validation - fail" do
    app = E2ERubyApp.create_app_url_encoded_22_string_min_length_validation_fail
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/form/validated", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, data: {"username" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "username"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

end
