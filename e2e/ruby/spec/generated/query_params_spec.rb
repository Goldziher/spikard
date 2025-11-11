# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "query_params" do
  it "42_negative_integer_query_param" do
    app = E2ERubyApp.create_app_query_params_1_42_negative_integer_query_param
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/negative", query: {"offset" => "-10"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"offset" => -10})
    client.close
  end

  it "43_scientific_notation_float" do
    app = E2ERubyApp.create_app_query_params_2_43_scientific_notation_float
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/stats", query: {"threshold" => "1.5e-3"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"threshold" => 0.0015})
    client.close
  end

  it "44_string_minlength_validation_success" do
    app = E2ERubyApp.create_app_query_params_3_44_string_minlength_validation_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search", query: {"term" => "foo"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"term" => "foo"})
    client.close
  end

  it "45_string_minlength_validation_failure" do
    app = E2ERubyApp.create_app_query_params_4_45_string_minlength_validation_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search", query: {"term" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "term"])
    client.close
  end

  it "46_string_maxlength_validation_failure" do
    app = E2ERubyApp.create_app_query_params_5_46_string_maxlength_validation_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search", query: {"term" => "this_is_way_too_long"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "term"])
    client.close
  end

  it "47_pattern_validation_email_success" do
    app = E2ERubyApp.create_app_query_params_6_47_pattern_validation_email_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/subscribe", query: {"email" => "user@example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"email" => "user@example.com"})
    client.close
  end

  it "48_pattern_validation_email_failure" do
    app = E2ERubyApp.create_app_query_params_7_48_pattern_validation_email_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/subscribe", query: {"email" => "invalid-email"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "email"])
    client.close
  end

  it "49_integer_gt_constraint_success" do
    app = E2ERubyApp.create_app_query_params_8_49_integer_gt_constraint_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"limit" => "5"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"limit" => 5})
    client.close
  end

  it "50_integer_gt_constraint_failure" do
    app = E2ERubyApp.create_app_query_params_9_50_integer_gt_constraint_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"limit" => "0"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "limit"])
    client.close
  end

  it "51_integer_ge_constraint_boundary" do
    app = E2ERubyApp.create_app_query_params_10_51_integer_ge_constraint_boundary
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"offset" => "0"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"offset" => 0})
    client.close
  end

  it "52_integer_le_constraint_boundary" do
    app = E2ERubyApp.create_app_query_params_11_52_integer_le_constraint_boundary
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"limit" => "100"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"limit" => 100})
    client.close
  end

  it "53_integer_le_constraint_failure" do
    app = E2ERubyApp.create_app_query_params_12_53_integer_le_constraint_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"limit" => "101"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "limit"])
    client.close
  end

  it "54_array_minitems_constraint_success" do
    app = E2ERubyApp.create_app_query_params_13_54_array_minitems_constraint_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?ids=1&ids=2&ids=3")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ids" => [1, 2, 3]})
    client.close
  end

  it "55_array_minitems_constraint_failure" do
    app = E2ERubyApp.create_app_query_params_14_55_array_minitems_constraint_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?ids=1")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "ids"])
    client.close
  end

  it "56_array_maxitems_constraint_failure" do
    app = E2ERubyApp.create_app_query_params_15_56_array_maxitems_constraint_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?tags=a&tags=b&tags=c&tags=d&tags=e&tags=f")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "tags"])
    client.close
  end

  it "57_boolean_empty_string_coercion" do
    app = E2ERubyApp.create_app_query_params_16_57_boolean_empty_string_coercion
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"active" => ""})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"active" => false})
    client.close
  end

  it "58_format_email_success" do
    app = E2ERubyApp.create_app_query_params_17_58_format_email_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/subscribe", query: {"email" => "user@example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"email" => "user@example.com"})
    client.close
  end

  it "59_format_email_failure" do
    app = E2ERubyApp.create_app_query_params_18_59_format_email_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/subscribe", query: {"email" => "not-an-email"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "email"])
    client.close
  end

  it "60_format_ipv4_success" do
    app = E2ERubyApp.create_app_query_params_19_60_format_ipv4_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/network", query: {"ip" => "192.168.1.1"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ip" => "192.168.1.1"})
    client.close
  end

  it "61_format_ipv4_failure" do
    app = E2ERubyApp.create_app_query_params_20_61_format_ipv4_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/network", query: {"ip" => "999.999.999.999"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "ip"])
    client.close
  end

  it "62_format_ipv6_success" do
    app = E2ERubyApp.create_app_query_params_21_62_format_ipv6_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/network/ipv6", query: {"ip" => "2001:0db8:85a3:0000:0000:8a2e:0370:7334"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ip" => "2001:0db8:85a3:0000:0000:8a2e:0370:7334"})
    client.close
  end

  it "63_format_uri_success" do
    app = E2ERubyApp.create_app_query_params_22_63_format_uri_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/redirect", query: {"url" => "https://example.com/path?query=value"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"url" => "https://example.com/path?query=value"})
    client.close
  end

  it "64_format_uri_failure" do
    app = E2ERubyApp.create_app_query_params_23_64_format_uri_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/redirect", query: {"url" => "not a uri"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "url"])
    client.close
  end

  it "65_format_hostname_success" do
    app = E2ERubyApp.create_app_query_params_24_65_format_hostname_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/dns", query: {"host" => "api.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"host" => "api.example.com"})
    client.close
  end

  it "66_multipleof_constraint_success" do
    app = E2ERubyApp.create_app_query_params_25_66_multipleof_constraint_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"quantity" => "15"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"quantity" => 15})
    client.close
  end

  it "67_multipleof_constraint_failure" do
    app = E2ERubyApp.create_app_query_params_26_67_multipleof_constraint_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"quantity" => "17"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "quantity"])
    client.close
  end

  it "68_array_uniqueitems_success" do
    app = E2ERubyApp.create_app_query_params_27_68_array_uniqueitems_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?ids=1&ids=2&ids=3&ids=4")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"ids" => [1, 2, 3, 4]})
    client.close
  end

  it "69_array_uniqueitems_failure" do
    app = E2ERubyApp.create_app_query_params_28_69_array_uniqueitems_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?ids=1&ids=2&ids=2&ids=3")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "ids"])
    client.close
  end

  it "70_array_separator_pipe" do
    app = E2ERubyApp.create_app_query_params_29_70_array_separator_pipe
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?tags=python|rust|typescript", query: {"tags" => "python|rust|typescript"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"tags" => ["python", "rust", "typescript"]})
    client.close
  end

  it "71_array_separator_semicolon" do
    app = E2ERubyApp.create_app_query_params_30_71_array_separator_semicolon
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?colors=red;green;blue", query: {"colors" => "red;green;blue"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"colors" => ["red", "green", "blue"]})
    client.close
  end

  it "72_array_separator_space" do
    app = E2ERubyApp.create_app_query_params_31_72_array_separator_space
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search?keywords=rust%20web%20framework", query: {"keywords" => "rust web framework"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"keywords" => ["rust", "web", "framework"]})
    client.close
  end

  it "Array query parameter - empty array" do
    app = E2ERubyApp.create_app_query_params_32_array_query_parameter_empty_array
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/list-default", query: {})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq([])
    client.close
  end

  it "Array query parameter - single value" do
    app = E2ERubyApp.create_app_query_params_33_array_query_parameter_single_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/list-default", query: {"tags" => "apple"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq(["apple"])
    client.close
  end

  it "Boolean query parameter - numeric 1" do
    app = E2ERubyApp.create_app_query_params_34_boolean_query_parameter_numeric_1
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/bool", query: {"flag" => "1"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"flag" => true})
    client.close
  end

  it "Boolean query parameter - true" do
    app = E2ERubyApp.create_app_query_params_35_boolean_query_parameter_true
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/bool", query: {"flag" => "true"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"flag" => true})
    client.close
  end

  it "Date query parameter - success" do
    app = E2ERubyApp.create_app_query_params_36_date_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/date", query: {"event_date" => "2024-01-15"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"event_date" => "2024-01-15"})
    client.close
  end

  it "Datetime query parameter - success" do
    app = E2ERubyApp.create_app_query_params_37_datetime_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/datetime", query: {"timestamp" => "2024-01-15T10:30:00Z"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"timestamp" => "2024-01-15T10:30:00Z"})
    client.close
  end

  it "Enum query parameter - invalid value" do
    app = E2ERubyApp.create_app_query_params_38_enum_query_parameter_invalid_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/enum", query: {"model" => "vgg16"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "model"])
    expect(body['errors'].first['type']).to eq("enum")
    client.close
  end

  it "Enum query parameter - success" do
    app = E2ERubyApp.create_app_query_params_39_enum_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/enum", query: {"model" => "alexnet"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"model" => "alexnet"})
    client.close
  end

  it "Float query param with ge constraint - success" do
    app = E2ERubyApp.create_app_query_params_40_float_query_param_with_ge_constraint_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/float-ge", query: {"price" => "0.01"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"price" => 0.01})
    client.close
  end

  it "Integer query param with ge constraint - boundary" do
    app = E2ERubyApp.create_app_query_params_41_integer_query_param_with_ge_constraint_boundary
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int-ge", query: {"value" => "10"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => 10})
    client.close
  end

  it "Integer query param with gt constraint - valid" do
    app = E2ERubyApp.create_app_query_params_42_integer_query_param_with_gt_constraint_valid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int-gt", query: {"value" => "1"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => 1})
    client.close
  end

  it "Integer query param with le constraint - boundary" do
    app = E2ERubyApp.create_app_query_params_43_integer_query_param_with_le_constraint_boundary
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int-le", query: {"value" => "100"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => 100})
    client.close
  end

  it "Integer query param with lt constraint - valid" do
    app = E2ERubyApp.create_app_query_params_44_integer_query_param_with_lt_constraint_valid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int-lt", query: {"value" => "49"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => 49})
    client.close
  end

  it "Integer with default value - not provided" do
    app = E2ERubyApp.create_app_query_params_45_integer_with_default_value_not_provided
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int/default")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar 10")
    client.close
  end

  it "Integer with default value - override" do
    app = E2ERubyApp.create_app_query_params_46_integer_with_default_value_override
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int/default", query: {"query" => 50})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar 50")
    client.close
  end

  it "List of integers - multiple values" do
    app = E2ERubyApp.create_app_query_params_47_list_of_integers_multiple_values
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/list?device_ids=1&device_ids=2")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq([1, 2])
    client.close
  end

  it "List of strings - multiple values" do
    app = E2ERubyApp.create_app_query_params_48_list_of_strings_multiple_values
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/?q=foo&q=bar")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"q" => ["foo", "bar"]})
    client.close
  end

  it "List query parameter - required but missing" do
    app = E2ERubyApp.create_app_query_params_49_list_query_parameter_required_but_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/list")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "device_ids"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "List with default empty array - no values provided" do
    app = E2ERubyApp.create_app_query_params_50_list_with_default_empty_array_no_values_provided
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/list-default")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq([])
    client.close
  end

  it "Multiple query parameters with different types" do
    app = E2ERubyApp.create_app_query_params_51_multiple_query_parameters_with_different_types
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/multi-type", query: {"active" => "true", "age" => "30", "name" => "john", "score" => "95.5"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"active" => true, "age" => 30, "name" => "john", "score" => 95.5})
    client.close
  end

  it "Optional integer query parameter - missing" do
    app = E2ERubyApp.create_app_query_params_52_optional_integer_query_parameter_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int/optional")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar None")
    client.close
  end

  it "Optional query parameter with default value" do
    app = E2ERubyApp.create_app_query_params_53_optional_query_parameter_with_default_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/optional-default", query: {})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"limit" => 10})
    client.close
  end

  it "Optional string query parameter - missing" do
    app = E2ERubyApp.create_app_query_params_54_optional_string_query_parameter_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/optional")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar None")
    client.close
  end

  it "Optional string query parameter - provided" do
    app = E2ERubyApp.create_app_query_params_55_optional_string_query_parameter_provided
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/optional", query: {"query" => "baz"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar baz")
    client.close
  end

  it "Query parameter with URL encoded space" do
    app = E2ERubyApp.create_app_query_params_56_query_parameter_with_url_encoded_space
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/basic", query: {"name" => "hello world"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "hello world"})
    client.close
  end

  it "Query parameter with URL encoded special characters" do
    app = E2ERubyApp.create_app_query_params_57_query_parameter_with_url_encoded_special_characters
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/basic", query: {"name" => "test&value=123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"name" => "test&value=123"})
    client.close
  end

  it "Query parameter with special characters - URL encoding" do
    app = E2ERubyApp.create_app_query_params_58_query_parameter_with_special_characters_url_encoding
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/test", query: {"email" => "x@test.com", "special" => "&@A.ac"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"email" => "x@test.com", "special" => "&@A.ac"})
    client.close
  end

  it "Required integer query parameter - float value" do
    app = E2ERubyApp.create_app_query_params_59_required_integer_query_parameter_float_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int", query: {"query" => "42.5"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "query"])
    expect(body['errors'].first['type']).to eq("int_parsing")
    client.close
  end

  it "Required integer query parameter - invalid type" do
    app = E2ERubyApp.create_app_query_params_60_required_integer_query_parameter_invalid_type
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int", query: {"query" => "baz"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "query"])
    expect(body['errors'].first['type']).to eq("int_parsing")
    client.close
  end

  it "Required integer query parameter - missing" do
    app = E2ERubyApp.create_app_query_params_61_required_integer_query_parameter_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "query"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Required integer query parameter - success" do
    app = E2ERubyApp.create_app_query_params_62_required_integer_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/int", query: {"query" => 42})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar 42")
    client.close
  end

  it "Required string query parameter - missing" do
    app = E2ERubyApp.create_app_query_params_63_required_string_query_parameter_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "query"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Required string query parameter - success" do
    app = E2ERubyApp.create_app_query_params_64_required_string_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query", query: {"query" => "baz"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq("foo bar baz")
    client.close
  end

  it "String query param with max_length constraint - fail" do
    app = E2ERubyApp.create_app_query_params_65_string_query_param_with_max_length_constraint_fail
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/str-max-length", query: {"name" => "this_is_way_too_long"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "name"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "String query param with min_length constraint - fail" do
    app = E2ERubyApp.create_app_query_params_66_string_query_param_with_min_length_constraint_fail
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/str-min-length", query: {"name" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "name"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "String query param with regex pattern - fail" do
    app = E2ERubyApp.create_app_query_params_67_string_query_param_with_regex_pattern_fail
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/pattern", query: {"code" => "abc123"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "code"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "String validation with regex - failure" do
    app = E2ERubyApp.create_app_query_params_68_string_validation_with_regex_failure
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", query: {"item_query" => "nonregexquery"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "item_query"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "String validation with regex - success" do
    app = E2ERubyApp.create_app_query_params_69_string_validation_with_regex_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", query: {"item_query" => "fixedquery"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_query" => "fixedquery"})
    client.close
  end

  it "UUID query parameter - invalid format" do
    app = E2ERubyApp.create_app_query_params_70_uuid_query_parameter_invalid_format
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/uuid", query: {"item_id" => "not-a-uuid"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["query", "item_id"])
    expect(body['errors'].first['type']).to eq("uuid_parsing")
    client.close
  end

  it "UUID query parameter - success" do
    app = E2ERubyApp.create_app_query_params_71_uuid_query_parameter_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/query/uuid", query: {"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"})
    client.close
  end

end
