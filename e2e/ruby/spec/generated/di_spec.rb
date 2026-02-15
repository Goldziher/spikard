# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "di" do
  it "Async factory dependency - success" do
    app = E2ERubyApp.create_app_di_1_async_factory_dependency_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/db-status")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"max_size" => 10, "pool_status" => "connected"})
    client.close
  end

  it "Circular dependency detection - error" do
    app = E2ERubyApp.create_app_di_2_circular_dependency_detection_error
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/circular")
    expect(response.status_code).to eq(500)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("Circular dependency detected")
    expect(body['status']).to eq(500)
    expect(body['errors'].first['type']).to eq("circular_dependency")
    client.close
  end

  it "Dependency injection in lifecycle hooks - success" do
    app = E2ERubyApp.create_app_di_3_dependency_injection_in_lifecycle_hooks_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/hook-di-test", headers: {"authorization" => "Bearer valid_token"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"authenticated" => true, "logged" => true})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-auth-mode"]).to eq("strict")
    expect(response_headers["x-log-level"]).to eq("debug")
    client.close
  end

  it "Factory dependency - success" do
    app = E2ERubyApp.create_app_di_4_factory_dependency_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/timestamp")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"timestamp" => "<<present>>"})
    client.close
  end

  it "Missing dependency - error" do
    app = E2ERubyApp.create_app_di_5_missing_dependency_error
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/missing-dep")
    expect(response.status_code).to eq(500)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("Required dependency not found")
    expect(body['status']).to eq(500)
    expect(body['errors'].first['type']).to eq("missing_dependency")
    client.close
  end

  it "Mixed singleton and per-request caching - success" do
    app = E2ERubyApp.create_app_di_6_mixed_singleton_and_per_request_caching_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/mixed-caching")

    # Second request to verify singleton caching
    response2 = client.get("/api/mixed-caching")
    expect(response.status_code).to eq(200)
    expect(response2.status_code).to eq(200)
    data1 = response.json
    data2 = response2.json

    # pool_id is singleton; context_id is per-request
    expect(data1).to have_key('pool_id')
    expect(data2).to have_key('pool_id')
    expect(data1['pool_id']).to eq(data2['pool_id'])
    expect(data1).to have_key('context_id')
    expect(data2).to have_key('context_id')
    expect(data1['context_id']).not_to eq(data2['context_id'])
    client.close
  end

  it "Multiple dependencies with cleanup - success" do
    app = E2ERubyApp.create_app_di_7_multiple_dependencies_with_cleanup_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/multi-cleanup-test")
    expect(response.status_code).to eq(200)

    # Allow async cleanup to complete
    sleep 0.1

    # Verify cleanup was called
    cleanup_response = client.get('/api/cleanup-state')
    expect(cleanup_response.status_code).to eq(200)
    cleanup_state = cleanup_response.json
    expect(cleanup_state).to have_key('cleanup_events')
    events = cleanup_state['cleanup_events']
    expect(events).to include('session_opened')
    expect(events).to include('session_closed')
    client.close
  end

  it "Nested dependencies (3 levels) - success" do
    app = E2ERubyApp.create_app_di_8_nested_dependencies_3_levels_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/auth-status")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"auth_enabled" => true, "has_cache" => true, "has_db" => true})
    client.close
  end

  it "Node.js object destructuring injection - success" do
    app = E2ERubyApp.create_app_di_9_node_js_object_destructuring_injection_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/node-destructure")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"db_name" => "PostgreSQL", "log_level" => "info"})
    client.close
  end

  it "Per-request dependency caching - success" do
    app = E2ERubyApp.create_app_di_10_per_request_dependency_caching_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/request-id")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"first_id" => "<<uuid>>", "second_id" => "<<same_as:first_id>>"})
    client.close
  end

  it "Python parameter name-based injection - success" do
    app = E2ERubyApp.create_app_di_11_python_parameter_name_based_injection_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/python-name-inject")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"cache_status" => "ready", "db_status" => "connected"})
    client.close
  end

  it "Python type annotation-based injection - success" do
    app = E2ERubyApp.create_app_di_12_python_type_annotation_based_injection_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/python-type-inject")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"cache_type" => "Redis", "pool_type" => "PostgreSQL"})
    client.close
  end

  it "Resource cleanup after request - success" do
    app = E2ERubyApp.create_app_di_13_resource_cleanup_after_request_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/cleanup-test")
    expect(response.status_code).to eq(200)

    # Allow async cleanup to complete
    sleep 0.1

    # Verify cleanup was called
    cleanup_response = client.get('/api/cleanup-state')
    expect(cleanup_response.status_code).to eq(200)
    cleanup_state = cleanup_response.json
    expect(cleanup_state).to have_key('cleanup_events')
    events = cleanup_state['cleanup_events']
    expect(events).to include('session_opened')
    expect(events).to include('session_closed')
    client.close
  end

  it "Route-level dependency override - success" do
    app = E2ERubyApp.create_app_di_14_route_level_dependency_override_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/override-test")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"mode" => "test", "strict" => false})
    client.close
  end

  it "Ruby keyword argument injection - success" do
    app = E2ERubyApp.create_app_di_15_ruby_keyword_argument_injection_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/ruby-kwargs")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"adapter" => "postgresql", "user_id" => 42})
    client.close
  end

  it "Singleton dependency caching - success" do
    app = E2ERubyApp.create_app_di_16_singleton_dependency_caching_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/app-counter")

    # Second request to verify singleton caching
    response2 = client.get("/api/app-counter")
    expect(response.status_code).to eq(200)
    expect(response2.status_code).to eq(200)
    data1 = response.json
    data2 = response2.json

    # Singleton counter should have stable counter_id and incremented count
    expect(data1).to have_key('counter_id')
    expect(data2).to have_key('counter_id')
    expect(data1['counter_id']).to eq(data2['counter_id'])
    expect(data2['count']).to be > data1['count']
    client.close
  end

  it "Type mismatch in dependency resolution - error" do
    app = E2ERubyApp.create_app_di_17_type_mismatch_in_dependency_resolution_error
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/type-mismatch")
    expect(response.status_code).to eq(500)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("Dependency type mismatch")
    expect(body['status']).to eq(500)
    expect(body['errors'].first['type']).to eq("type_mismatch")
    client.close
  end

  it "Value dependency injection - success" do
    app = E2ERubyApp.create_app_di_18_value_dependency_injection_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/config")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"app_name" => "SpikardApp", "max_connections" => 100, "version" => "1.0.0"})
    client.close
  end

end
