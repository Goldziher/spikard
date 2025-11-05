# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "cookies" do
  it "24_cookie_samesite_strict" do
    app = E2ERubyApp.create_app_cookies_1_24_cookie_samesite_strict
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"session_id" => "abc123xyz789"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "25_cookie_samesite_lax" do
    app = E2ERubyApp.create_app_cookies_2_25_cookie_samesite_lax
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"tracking" => "track123"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "26_cookie_secure_flag" do
    app = E2ERubyApp.create_app_cookies_3_26_cookie_secure_flag
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"auth_token" => "secure_token_xyz"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "27_cookie_httponly_flag" do
    app = E2ERubyApp.create_app_cookies_4_27_cookie_httponly_flag
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"session" => "session_abc123"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "APIKey cookie authentication - missing" do
    app = E2ERubyApp.create_app_cookies_5_apikey_cookie_authentication_missing
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["cookie", "key"])
    client.close
  end

  it "APIKey cookie authentication - success" do
    app = E2ERubyApp.create_app_cookies_6_apikey_cookie_authentication_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"key" => "secret"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "secret"})
    client.close
  end

  it "Cookie regex pattern validation - fail" do
    app = E2ERubyApp.create_app_cookies_7_cookie_regex_pattern_validation_fail
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"tracking_id" => "invalid-format"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["cookie", "tracking_id"])
    client.close
  end

  it "Cookie regex pattern validation - success" do
    app = E2ERubyApp.create_app_cookies_8_cookie_regex_pattern_validation_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"tracking_id" => "ABC12345"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"tracking_id" => "ABC12345"})
    client.close
  end

  it "Cookie validation - max_length constraint fail" do
    app = E2ERubyApp.create_app_cookies_9_cookie_validation_max_length_constraint_fail
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"session_id" => "this_cookie_value_is_way_too_long"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["cookie", "session_id"])
    client.close
  end

  it "Cookie validation - min_length constraint success" do
    app = E2ERubyApp.create_app_cookies_10_cookie_validation_min_length_constraint_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"token" => "abc"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"token" => "abc"})
    client.close
  end

  it "Cookie validation - min_length failure" do
    app = E2ERubyApp.create_app_cookies_11_cookie_validation_min_length_failure
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"tracking_id" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["cookie", "tracking_id"])
    client.close
  end

  it "Multiple cookies - success" do
    app = E2ERubyApp.create_app_cookies_12_multiple_cookies_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"fatebook_tracker" => "tracker456", "googall_tracker" => "ga789", "session_id" => "session123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"fatebook_tracker" => "tracker456", "googall_tracker" => "ga789", "session_id" => "session123"})
    client.close
  end

  it "Optional APIKey cookie - missing" do
    app = E2ERubyApp.create_app_cookies_13_optional_apikey_cookie_missing
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"msg" => "Create an account first"})
    client.close
  end

  it "Optional cookie parameter - missing" do
    app = E2ERubyApp.create_app_cookies_14_optional_cookie_parameter_missing
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ads_id" => nil})
    client.close
  end

  it "Optional cookie parameter - success" do
    app = E2ERubyApp.create_app_cookies_15_optional_cookie_parameter_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"ads_id" => "abc123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ads_id" => "abc123"})
    client.close
  end

  it "Required cookie - missing" do
    app = E2ERubyApp.create_app_cookies_16_required_cookie_missing
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, cookies: {"fatebook_tracker" => "tracker456"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["cookie", "session_id"])
    client.close
  end

  it "Response - delete cookie" do
    app = E2ERubyApp.create_app_cookies_17_response_delete_cookie
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, cookies: {"session" => "old_session_123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie deleted"})
    client.close
  end

  it "Response - multiple cookies" do
    app = E2ERubyApp.create_app_cookies_18_response_multiple_cookies
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"session" => "session123", "user" => "john"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Multiple cookies set"})
    client.close
  end

  it "Response - session cookie (no max_age)" do
    app = E2ERubyApp.create_app_cookies_19_response_session_cookie_no_max_age
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "session_abc123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Session cookie set"})
    client.close
  end

  it "Response cookie with SameSite=Lax" do
    app = E2ERubyApp.create_app_cookies_20_response_cookie_with_samesite_lax
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "lax_cookie"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set with SameSite=Lax"})
    client.close
  end

  it "Response cookie with SameSite=None" do
    app = E2ERubyApp.create_app_cookies_21_response_cookie_with_samesite_none
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "none_cookie"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set with SameSite=None"})
    client.close
  end

  it "Response cookie with SameSite=Strict" do
    app = E2ERubyApp.create_app_cookies_22_response_cookie_with_samesite_strict
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "strict_cookie"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set with SameSite=Strict"})
    client.close
  end

  it "Response cookie with attributes" do
    app = E2ERubyApp.create_app_cookies_23_response_cookie_with_attributes
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set"})
    client.close
  end

  it "Response cookie with domain attribute" do
    app = E2ERubyApp.create_app_cookies_24_response_cookie_with_domain_attribute
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "domain_test"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set with domain"})
    client.close
  end

  it "Response cookie with path attribute" do
    app = E2ERubyApp.create_app_cookies_25_response_cookie_with_path_attribute
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"value" => "path_test"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Cookie set with path"})
    client.close
  end

  it "Response set cookie - basic" do
    app = E2ERubyApp.create_app_cookies_26_response_set_cookie_basic
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Come to the dark side, we have cookies"})
    client.close
  end

end
