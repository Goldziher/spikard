# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "path_params" do
  it "20_uuid_v3_path_param_success" do
    app = E2ERubyApp.create_app_path_params_1_20_uuid_v3_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/e8b5a51d-11c8-3310-a6ab-367563f20686")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => "e8b5a51d-11c8-3310-a6ab-367563f20686"})
    client.close
  end

  it "21_uuid_v5_path_param_success" do
    app = E2ERubyApp.create_app_path_params_2_21_uuid_v5_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/630eb68f-e0fa-5ecc-887a-7c7a62614681")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => "630eb68f-e0fa-5ecc-887a-7c7a62614681"})
    client.close
  end

  it "24_date_format_path_param_success" do
    app = E2ERubyApp.create_app_path_params_3_24_date_format_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/events/2025-10-30")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"date" => "2025-10-30"})
    client.close
  end

  it "25_date_format_invalid_failure" do
    app = E2ERubyApp.create_app_path_params_4_25_date_format_invalid_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/events/2025-13-45")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "date"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "27_datetime_format_path_param_success" do
    app = E2ERubyApp.create_app_path_params_5_27_datetime_format_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/bookings/2025-10-30T14:30:00Z")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"timestamp" => "2025-10-30T14:30:00Z"})
    client.close
  end

  it "28_duration_format_path_param_success" do
    app = E2ERubyApp.create_app_path_params_6_28_duration_format_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/delays/P1DT2H30M")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"duration" => "P1DT2H30M"})
    client.close
  end

  it "29_decimal_path_param_success" do
    app = E2ERubyApp.create_app_path_params_7_29_decimal_path_param_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/prices/19.99")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"amount" => "19.99"})
    client.close
  end

  it "30_string_minlength_path_success" do
    app = E2ERubyApp.create_app_path_params_8_30_string_minlength_path_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/users/alice")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "alice"})
    client.close
  end

  it "31_string_minlength_path_failure" do
    app = E2ERubyApp.create_app_path_params_9_31_string_minlength_path_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/users/ab")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "username"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "32_string_maxlength_path_failure" do
    app = E2ERubyApp.create_app_path_params_10_32_string_maxlength_path_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/users/this_username_is_way_too_long_to_be_valid")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "username"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "33_string_pattern_path_success" do
    app = E2ERubyApp.create_app_path_params_11_33_string_pattern_path_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/repos/spikard-labs/spikard-http")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"owner" => "spikard-labs", "repo" => "spikard-http"})
    client.close
  end

  it "34_string_pattern_path_failure" do
    app = E2ERubyApp.create_app_path_params_12_34_string_pattern_path_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/repos/invalid@owner")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "owner"])
    expect(body['errors'].first['type']).to eq("validation_error")
    client.close
  end

  it "35_negative_integer_path_param" do
    app = E2ERubyApp.create_app_path_params_13_35_negative_integer_path_param
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/offset/-100")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"value" => -100})
    client.close
  end

  it "Boolean path parameter - True" do
    app = E2ERubyApp.create_app_path_params_14_boolean_path_parameter_true
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/bool/True")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => true})
    client.close
  end

  it "Boolean path parameter - numeric 1" do
    app = E2ERubyApp.create_app_path_params_15_boolean_path_parameter_numeric_1
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/bool/1")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => true})
    client.close
  end

  it "Date path parameter - success" do
    app = E2ERubyApp.create_app_path_params_16_date_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/date/2023-07-15")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"date_param" => "2023-07-15"})
    client.close
  end

  it "Enum path parameter - invalid value" do
    app = E2ERubyApp.create_app_path_params_17_enum_path_parameter_invalid_value
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/models/foo")
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

  it "Enum path parameter - success" do
    app = E2ERubyApp.create_app_path_params_18_enum_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/models/alexnet")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"model_name" => "alexnet"})
    client.close
  end

  it "Float path parameter - success" do
    app = E2ERubyApp.create_app_path_params_19_float_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/float/42.5")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 42.5})
    client.close
  end

  it "Integer path parameter - invalid string" do
    app = E2ERubyApp.create_app_path_params_20_integer_path_parameter_invalid_string
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/int/foobar")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "item_id"])
    expect(body['errors'].first['type']).to eq("int_parsing")
    client.close
  end

  it "Integer path parameter - success" do
    app = E2ERubyApp.create_app_path_params_21_integer_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/int/42")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 42})
    client.close
  end

  it "Integer path parameter with combined lt and gt constraints - success" do
    app = E2ERubyApp.create_app_path_params_22_integer_path_parameter_with_combined_lt_and_gt_constraints_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-lt-gt/2")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 2})
    client.close
  end

  it "Integer path parameter with ge constraint - success" do
    app = E2ERubyApp.create_app_path_params_23_integer_path_parameter_with_ge_constraint_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-ge/3")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 3})
    client.close
  end

  it "Integer path parameter with gt constraint - failure" do
    app = E2ERubyApp.create_app_path_params_24_integer_path_parameter_with_gt_constraint_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-gt/2")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "item_id"])
    expect(body['errors'].first['type']).to eq("greater_than")
    client.close
  end

  it "Integer path parameter with gt constraint - success" do
    app = E2ERubyApp.create_app_path_params_25_integer_path_parameter_with_gt_constraint_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-gt/42")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 42})
    client.close
  end

  it "Integer path parameter with le constraint - success" do
    app = E2ERubyApp.create_app_path_params_26_integer_path_parameter_with_le_constraint_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-le/3")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 3})
    client.close
  end

  it "Integer path parameter with lt constraint - success" do
    app = E2ERubyApp.create_app_path_params_27_integer_path_parameter_with_lt_constraint_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-lt/2")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => 2})
    client.close
  end

  it "Multiple path parameters - success" do
    app = E2ERubyApp.create_app_path_params_28_multiple_path_parameters_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"order_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716", "service_id" => 1, "user_id" => "abc", "version" => 1.0})
    client.close
  end

  it "Path parameter type syntax - invalid UUID" do
    app = E2ERubyApp.create_app_path_params_29_path_parameter_type_syntax_invalid_uuid
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/type-syntax/items/not-a-uuid")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "id"])
    expect(body['errors'].first['type']).to eq("uuid_parsing")
    client.close
  end

  it "Path parameter type syntax with override" do
    app = E2ERubyApp.create_app_path_params_30_path_parameter_type_syntax_with_override
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/type-syntax/items-count/50")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"count" => "50"})
    client.close
  end

  it "Path parameter with type syntax - UUID" do
    app = E2ERubyApp.create_app_path_params_31_path_parameter_with_type_syntax_uuid
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/type-syntax/items/550e8400-e29b-41d4-a716-446655440000")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => "550e8400-e29b-41d4-a716-446655440000"})
    client.close
  end

  it "Path parameter with type syntax - integer" do
    app = E2ERubyApp.create_app_path_params_32_path_parameter_with_type_syntax_integer
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/type-syntax/users/42")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"user_id" => "42"})
    client.close
  end

  it "Path type parameter - file path" do
    app = E2ERubyApp.create_app_path_params_33_path_type_parameter_file_path
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/files/home/johndoe/myfile.txt")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"file_path" => "home/johndoe/myfile.txt"})
    client.close
  end

  it "String path parameter - success" do
    app = E2ERubyApp.create_app_path_params_34_string_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/str/foobar")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => "foobar"})
    client.close
  end

  it "String path parameter with max_length - failure" do
    app = E2ERubyApp.create_app_path_params_35_string_path_parameter_with_max_length_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-maxlength/foobar")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "item_id"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "String path parameter with min_length - failure" do
    app = E2ERubyApp.create_app_path_params_36_string_path_parameter_with_min_length_failure
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/path/param-minlength/fo")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["path", "item_id"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "UUID path parameter - success" do
    app = E2ERubyApp.create_app_path_params_37_uuid_path_parameter_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"item_id" => "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"})
    client.close
  end

end
