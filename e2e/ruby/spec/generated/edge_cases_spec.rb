# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "edge_cases" do
  it "11_utf8_query_parameter" do
    app = E2ERubyApp.create_app_edge_cases_1_11_utf8_query_parameter
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search", query: {"term" => "caf\u{e9}"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"term" => "caf\u{e9}"})
    client.close
  end

  it "12_percent_encoded_special_chars" do
    app = E2ERubyApp.create_app_edge_cases_2_12_percent_encoded_special_chars
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/search?term=hi%20there", query: {"term" => "hi there"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"term" => "hi there"})
    client.close
  end

  it "13_empty_string_query_param_preserved" do
    app = E2ERubyApp.create_app_edge_cases_3_13_empty_string_query_param_preserved
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items?filter=", query: {"filter" => ""})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"filter" => ""})
    client.close
  end

  it "14_large_integer_boundary" do
    app = E2ERubyApp.create_app_edge_cases_4_14_large_integer_boundary
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items", query: {"id" => "9007199254740991"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 9007199254740991})
    client.close
  end

  it "15_float_precision_preservation" do
    app = E2ERubyApp.create_app_edge_cases_5_15_float_precision_preservation
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/calculate", json: {"value" => 3.141592653589793})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"value" => 3.141592653589793})
    client.close
  end

  it "16_negative_zero_handling" do
    app = E2ERubyApp.create_app_edge_cases_6_16_negative_zero_handling
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", json: {"offset" => -0.0})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"offset" => 0})
    client.close
  end

  it "17_extremely_long_string" do
    app = E2ERubyApp.create_app_edge_cases_7_17_extremely_long_string
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/text", json: {"content" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "content"])
    client.close
  end

  it "18_unicode_normalization" do
    app = E2ERubyApp.create_app_edge_cases_8_18_unicode_normalization
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/users", json: {"name" => "caf\u{e9}"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"name" => "caf\u{e9}"})
    client.close
  end

  it "19_emoji_in_strings" do
    app = E2ERubyApp.create_app_edge_cases_9_19_emoji_in_strings
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/messages", json: {"text" => "Hello \u{1f44b} World \u{1f30d}"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"text" => "Hello \u{1f44b} World \u{1f30d}"})
    client.close
  end

  it "20_null_byte_in_string" do
    app = E2ERubyApp.create_app_edge_cases_10_20_null_byte_in_string
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/files", json: {"filename" => "file\u{0}.txt"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "filename"])
    client.close
  end

  it "21_scientific_notation_number" do
    app = E2ERubyApp.create_app_edge_cases_11_21_scientific_notation_number
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/calculate", json: {"value" => 123000.0})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"value" => 123000})
    client.close
  end

  it "22_leading_zeros_integer" do
    app = E2ERubyApp.create_app_edge_cases_12_22_leading_zeros_integer
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/data", query: {"value" => "0123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => 123})
    client.close
  end

  it "23_deeply_nested_json_limit" do
    app = E2ERubyApp.create_app_edge_cases_13_23_deeply_nested_json_limit
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/data", json: {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"nested" => {"value" => "deep"}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}})
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"error" => "Request body exceeds maximum nesting depth of 32"})
    client.close
  end

  it "24_array_with_holes" do
    app = E2ERubyApp.create_app_edge_cases_14_24_array_with_holes
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/items", headers: {"Content-Type" => "application/x-www-form-urlencoded"}, raw_body: "items[0]=first&items[2]=third&items[5]=sixth")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"items" => ["first", "third", "sixth"]})
    client.close
  end

  it "Deeply nested structure (10+ levels)" do
    app = E2ERubyApp.create_app_edge_cases_15_deeply_nested_structure_10_levels
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/nested/", headers: {"Content-Type" => "application/json"}, json: {"level1" => {"level2" => {"level3" => {"level4" => {"level5" => {"level6" => {"level7" => {"level8" => {"level9" => {"level10" => {"depth" => 10, "value" => "deep"}}}}}}}}}}})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"max_depth" => 10, "message" => "Processed deeply nested structure", "value_found" => "deep"})
    client.close
  end

  it "Empty and null value handling" do
    app = E2ERubyApp.create_app_edge_cases_16_empty_and_null_value_handling
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/nulls/", headers: {"Content-Type" => "application/json"}, json: {"empty_array" => [], "empty_object" => {}, "empty_string" => "", "explicit_null" => nil, "false_boolean" => false, "zero_number" => 0})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"empty_array_length" => 0, "empty_object_keys" => 0, "empty_string_length" => 0, "explicit_null_is_null" => true, "false_is_false" => true, "zero_is_falsy" => true})
    client.close
  end

  it "Float precision and rounding" do
    app = E2ERubyApp.create_app_edge_cases_17_float_precision_and_rounding
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/calculations/", headers: {"Content-Type" => "application/json"}, json: {"expected_sum" => 0.3, "precise_value" => 3.141592653589793, "value1" => 0.1, "value2" => 0.2, "very_large" => 1.7976931348623157e308, "very_small" => 1e-10})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"precise_value" => 3.141592653589793, "sum" => 0.30000000000000004, "very_large" => 1.7976931348623157e308, "very_small" => 1e-10})
    client.close
  end

  it "Large integer boundary values" do
    app = E2ERubyApp.create_app_edge_cases_18_large_integer_boundary_values
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/numbers/", headers: {"Content-Type" => "application/json"}, json: {"large_int" => 9223372036854775807, "max_safe_int" => 9007199254740991, "negative_large" => -9223372036854775808})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"large_int" => 9223372036854775807, "max_safe_int" => 9007199254740991, "negative_large" => -9223372036854775808})
    client.close
  end

  it "Special string values and escaping" do
    app = E2ERubyApp.create_app_edge_cases_19_special_string_values_and_escaping
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/strings/", headers: {"Content-Type" => "application/json"}, json: {"backslashes" => "C:\\\\Users\\\\Path", "empty_string" => "", "quotes" => "He said \"hello\" and \'goodbye\'", "special_chars" => "!@#$%^&*()_+-=[]{}|;\':\",./<>?", "tabs_newlines" => "line1\n\tline2\r\nline3", "unicode_escapes" => "\\u0048\\u0065\\u006c\\u006c\\u006f", "whitespace" => "   "})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"backslashes" => "C:\\\\Users\\\\Path", "empty_string" => "", "quotes" => "He said \"hello\" and \'goodbye\'", "special_chars" => "!@#$%^&*()_+-=[]{}|;\':\",./<>?", "tabs_newlines" => "line1\n\tline2\r\nline3", "unicode_escapes" => "Hello", "whitespace" => "   "})
    client.close
  end

  it "Unicode and emoji handling" do
    app = E2ERubyApp.create_app_edge_cases_20_unicode_and_emoji_handling
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/items/", headers: {"Content-Type" => "application/json; charset=utf-8"}, json: {"description" => "Best caf\u{e9} in M\u{fc}nchen \u{1f1e9}\u{1f1ea}", "emoji_reactions" => "\u{1f44d}\u{2764}\u{fe0f}\u{1f602}\u{1f389}", "name" => "Coffee Shop \u{2615}", "tags" => ["\u{98df}\u{3079}\u{7269}", "\u{97f3}\u{697d}", "\u{1f4b0}"]})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"description" => "Best caf\u{e9} in M\u{fc}nchen \u{1f1e9}\u{1f1ea}", "emoji_reactions" => "\u{1f44d}\u{2764}\u{fe0f}\u{1f602}\u{1f389}", "id" => 1, "name" => "Coffee Shop \u{2615}", "tags" => ["\u{98df}\u{3079}\u{7269}", "\u{97f3}\u{697d}", "\u{1f4b0}"]})
    client.close
  end

end
