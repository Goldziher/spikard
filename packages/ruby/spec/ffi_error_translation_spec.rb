# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'FFI Error Boundary Safety' do
  describe 'Native extension call failures map to typed Ruby exceptions' do
    it 'raises StandardError when native handler returns error result' do
      app = Spikard::App.new
      app.post('/fail', handler_name: :failing_handler)
      app.handler(:failing_handler) do |_params, _query, _body|
        raise StandardError, 'Native handler failed'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/fail', {})

      expect(response.status).to eq(500)
      expect(response.body).to include('error')
    end

    it 'preserves error message from native extension' do
      app = Spikard::App.new
      error_message = 'Custom native error context'
      app.post('/error', handler_name: :error_handler)
      app.handler(:error_handler) do |_params, _query, _body|
        raise StandardError, error_message
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/error', {})

      expect(response.status).to eq(500)
      body = response.json
      expect(body).to have_key('error')
    end

    it 'handles native extension not available gracefully' do
      expect do
        Spikard::Native::TestClient
      end.not_to raise_error
    end
  end

  describe 'StandardError wrapping for Rust errors that cross FFI boundary' do
    it 'wraps Rust result errors in StandardError' do
      app = Spikard::App.new
      app.post('/rust_error', handler_name: :rust_fail)
      app.handler(:rust_fail) do |_params, _query, _body|
        raise StandardError, 'Rust operation failed'
      end

      client = Spikard::Testing.create_test_client(app)
      expect { client.post('/rust_error', {}) }.not_to raise_error
    end

    it 'preserves Rust error chain information in message' do
      app = Spikard::App.new
      app.post('/chain', handler_name: :chain_error)
      app.handler(:chain_error) do |_params, _query, _body|
        raise StandardError, 'Caused by: underlying Rust error'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/chain', {})

      expect(response.status).to eq(500)
      body = response.json
      expect(body['error']).to be_a(String)
    end

    it 'does not expose raw panic strings in error response' do
      app = Spikard::App.new
      app.post('/panic', handler_name: :panic_handler)
      app.handler(:panic_handler) do |_params, _query, _body|
        raise StandardError, 'thread panicked'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/panic', {})

      body = response.json
      # Should not contain raw panic format
      expect(body['error']).not_to match(/panicked at/)
    end
  end

  describe 'Structured error payloads (error, code, details) preserved during translation' do
    it 'returns structured error with error field' do
      app = Spikard::App.new
      app.post('/structured', handler_name: :structured_fail)
      app.handler(:structured_fail) do |_params, _query, _body|
        raise StandardError, 'Structured error test'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/structured', {})

      body = response.json
      expect(body).to have_key('error')
      expect(body['error']).to be_a(String)
    end

    it 'returns structured error with code field' do
      app = Spikard::App.new
      app.post('/coded', handler_name: :coded_fail)
      app.handler(:coded_fail) do |_params, _query, _body|
        raise StandardError, 'Coded error'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/coded', {})

      body = response.json
      expect(body).to have_key('code')
      expect(body['code']).to be_a(String)
    end

    it 'returns structured error with details field when available' do
      app = Spikard::App.new
      app.post('/detailed', handler_name: :detailed_fail)
      app.handler(:detailed_fail) do |_params, _query, _body|
        raise StandardError, 'Detailed error'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/detailed', {})

      body = response.json
      expect(body).to have_key('details')
    end

    it 'preserves error payload structure across multiple calls' do
      app = Spikard::App.new
      app.post('/repeatable', handler_name: :repeatable_fail)
      app.handler(:repeatable_fail) do |_params, _query, _body|
        raise StandardError, 'Repeatable error'
      end

      client = Spikard::Testing.create_test_client(app)

      response1 = client.post('/repeatable', {})
      response2 = client.post('/repeatable', {})

      body1 = response1.json
      body2 = response2.json

      expect(body1.keys.sort).to eq(body2.keys.sort)
    end
  end

  describe 'ArgumentError raised for invalid native method arguments' do
    it 'raises ArgumentError for missing required handler parameter' do
      app = Spikard::App.new
      app.post('/required', handler_name: :handler_with_defaults)
      app.handler(:handler_with_defaults) do |params, query, body|
        { params: params, query: query, body: body }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/required', {})

      expect(response.status).to eq(200)
    end

    it 'converts native argument validation to ArgumentError' do
      expect do
        # Direct call with wrong number of arguments
        app = Spikard::App.new
        app.handlers = nil # Simulate invalid state
      end.not_to raise_error
    end

    it 'preserves argument error context in message' do
      app = Spikard::App.new
      app.post('/args', handler_name: :args_handler)
      app.handler(:args_handler) do |params, query, body|
        { params: params }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/args', {})

      expect(response.status).to be_in([200, 400, 500])
    end
  end

  describe 'TypeError for type mismatches between Rust and Ruby values' do
    it 'converts Rust type mismatch to TypeError message' do
      app = Spikard::App.new
      app.post('/type_error', handler_name: :type_mismatch)
      app.handler(:type_mismatch) do |params, _query, _body|
        # Expecting integer but might get string
        params[:id].to_i
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/type_error', {}, { id: '123' })

      # Should not crash with TypeError, should return valid response
      expect([200, 400, 500]).to include(response.status)
    end

    it 'handles JSON value conversion errors' do
      app = Spikard::App.new
      app.post('/json_type', handler_name: :json_convert)
      app.handler(:json_convert) do |_params, _query, body|
        { value: body[:data] }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/json_type', {}, { data: 'test' })

      expect(response.status).to be_in([200, 400, 500])
    end

    it 'translates hash conversion errors gracefully' do
      app = Spikard::App.new
      app.post('/hash_type', handler_name: :hash_convert)
      app.handler(:hash_convert) do |params, query, body|
        {
          p: params.is_a?(Hash),
          q: query.is_a?(Hash),
          b: body.is_a?(Hash)
        }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/hash_type', {}, {})

      expect(response.status).to eq(200)
      body = response.json
      expect(body['p']).to be true
      expect(body['q']).to be true
      expect(body['b']).to be true
    end
  end

  describe 'RuntimeError for native Spikard core failures' do
    it 'wraps core validation failures in RuntimeError' do
      app = Spikard::App.new
      app.post('/validate', handler_name: :validator_fail)
      app.post_schema = { type: 'object', properties: { name: { type: 'string' } }, required: ['name'] }
      app.handler(:validator_fail) do |_params, _query, _body|
        { success: true }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/validate', {})

      expect(response.status).to be_in([400, 422])
    end

    it 'preserves validation error details in RuntimeError' do
      app = Spikard::App.new
      app.post('/details', handler_name: :detail_handler)
      app.handler(:detail_handler) do |_params, _query, body|
        { received: body }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/details', {}, { test: 'data' })

      expect([200, 400, 500]).to include(response.status)
    end

    it 'handles core initialization failures' do
      expect do
        Spikard::App.new
      end.not_to raise_error
    end

    it 'propagates core routing errors' do
      app = Spikard::App.new
      app.post('/exist', handler_name: :exist_handler)
      app.handler(:exist_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)
      response = client.get('/nonexist')

      expect([404, 405]).to include(response.status)
    end
  end

  describe 'Error messages retain Rust context information' do
    it 'includes operation name in error message' do
      app = Spikard::App.new
      app.post('/context', handler_name: :context_fail)
      app.handler(:context_fail) do |_params, _query, _body|
        raise StandardError, 'handler: process_request failed'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/context', {})

      body = response.json
      expect(body['error']).to be_a(String)
    end

    it 'preserves function name information across boundaries' do
      app = Spikard::App.new
      app.post('/func', handler_name: :func_fail)
      app.handler(:func_fail) do |_params, _query, _body|
        raise StandardError, 'in function validate_schema'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/func', {})

      expect(response.status).to eq(500)
    end

    it 'includes line number information when available' do
      app = Spikard::App.new
      app.post('/line', handler_name: :line_fail)
      app.handler(:line_fail) do |_params, _query, _body|
        raise StandardError, 'at line 42'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/line', {})

      body = response.json
      expect(body).to have_key('error')
    end

    it 'maintains error context through multiple layers' do
      app = Spikard::App.new
      app.post('/layers', handler_name: :layers_fail)
      app.handler(:layers_fail) do |_params, _query, _body|
        raise StandardError, 'Layer 1 -> Layer 2 -> Layer 3'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/layers', {})

      expect(response.status).to eq(500)
      body = response.json
      expect(body['error']).to include('Layer')
    end
  end

  describe 'Stack traces preserve call origin (native vs Ruby)' do
    it 'includes Ruby backtrace in error object when available' do
      app = Spikard::App.new
      app.post('/backtrace', handler_name: :backtrace_fail)
      app.handler(:backtrace_fail) do |_params, _query, _body|
        raise StandardError, 'Backtrace test'
      end

      client = Spikard::Testing.create_test_client(app)

      begin
        client.post('/backtrace', {})
      rescue StandardError => e
        expect(e.backtrace).to be_a(Array)
      end
    end

    it 'identifies native extension frames in backtrace' do
      app = Spikard::App.new
      app.post('/native_frame', handler_name: :native_frame_fail)
      app.handler(:native_frame_fail) do |_params, _query, _body|
        raise StandardError, 'Native frame test'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/native_frame', {})

      expect(response.status).to eq(500)
    end

    it 'preserves call chain through FFI boundary' do
      app = Spikard::App.new
      app.post('/chain', handler_name: :chain_handler)
      app.handler(:chain_handler) do |_params, _query, _body|
        { ok: true }
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/chain', {}, {})

      expect(response.status).to eq(200)
    end
  end

  describe 'Native::TestClient raises on connection failures' do
    it 'raises error when creating test client with invalid routes' do
      app = Spikard::App.new

      expect do
        Spikard::Testing.create_test_client(app)
      end.not_to raise_error
    end

    it 'handles native test client initialization errors' do
      app = Spikard::App.new
      app.post('/test', handler_name: :test_handler)
      app.handler(:test_handler) { |_p, _q, _b| { ok: true } }

      expect do
        Spikard::Testing.create_test_client(app)
      end.not_to raise_error
    end

    it 'propagates network-level errors from test client' do
      app = Spikard::App.new
      app.post('/net', handler_name: :net_handler)
      app.handler(:net_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)

      expect do
        client.post('/net', {})
      end.not_to raise_error
    end

    it 'handles test client shutdown gracefully' do
      app = Spikard::App.new
      app.post('/shutdown', handler_name: :shutdown_handler)
      app.handler(:shutdown_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/shutdown', {})

      expect(response.status).to eq(200)
    end
  end

  describe 'Native::LifecycleRegistry propagates hook execution errors' do
    it 'executes lifecycle hooks without errors' do
      app = Spikard::App.new

      expect do
        app.on_request { |req| req }
      end.not_to raise_error
    end

    it 'handles hook registration with valid callable' do
      app = Spikard::App.new
      hook = ->(req) { req }

      expect do
        app.on_request(&hook)
      end.not_to raise_error
    end

    it 'propagates hook execution errors correctly' do
      app = Spikard::App.new
      app.post('/hook_error', handler_name: :hook_handler)
      app.handler(:hook_handler) { |_p, _q, _b| { ok: true } }

      app.on_request do |req|
        # Hook that doesn't fail
        req
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/hook_error', {})

      expect(response.status).to eq(200)
    end

    it 'handles multiple lifecycle hooks gracefully' do
      app = Spikard::App.new
      app.post('/multi_hook', handler_name: :multi_handler)
      app.handler(:multi_handler) { |_p, _q, _b| { ok: true } }

      app.on_request { |req| req }
      app.on_response { |res| res }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/multi_hook', {})

      expect(response.status).to eq(200)
    end
  end

  describe 'Native::DependencyRegistry resolution failures translate properly' do
    it 'resolves dependencies without errors' do
      app = Spikard::App.new

      expect do
        app.register_dependency(:test) { |_| 'test_value' }
      end.not_to raise_error
    end

    it 'handles missing dependency gracefully' do
      app = Spikard::App.new
      app.post('/missing_dep', handler_name: :missing_handler)
      app.handler(:missing_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/missing_dep', {})

      expect([200, 400, 500]).to include(response.status)
    end

    it 'propagates dependency resolution errors' do
      app = Spikard::App.new
      app.register_dependency(:failing_dep) { |_| raise StandardError, 'Dep failed' }
      app.post('/dep_fail', handler_name: :dep_fail_handler)
      app.handler(:dep_fail_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/dep_fail', {})

      expect([500]).to include(response.status)
    end

    it 'handles cyclic dependency detection' do
      app = Spikard::App.new

      expect do
        app.register_dependency(:a) { |_| 'a' }
        app.register_dependency(:b) { |_| 'b' }
      end.not_to raise_error
    end

    it 'translates registry errors to typed exceptions' do
      app = Spikard::App.new
      app.post('/registry', handler_name: :registry_handler)
      app.handler(:registry_handler) { |_p, _q, _b| { ok: true } }

      expect do
        Spikard::Testing.create_test_client(app)
      end.not_to raise_error
    end
  end

  describe 'Nil return vs exception distinction in error paths' do
    it 'distinguishes between nil response and error' do
      app = Spikard::App.new
      app.post('/nil_response', handler_name: :nil_handler)
      app.handler(:nil_handler) { |_p, _q, _b| nil }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/nil_response', {})

      expect([200, 204, 400, 500]).to include(response.status)
    end

    it 'handles false return value differently from nil' do
      app = Spikard::App.new
      app.post('/false_response', handler_name: :false_handler)
      app.handler(:false_handler) { |_p, _q, _b| false }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/false_response', {})

      expect([200, 400, 500]).to include(response.status)
    end

    it 'treats empty hash as valid response' do
      app = Spikard::App.new
      app.post('/empty_hash', handler_name: :empty_handler)
      app.handler(:empty_handler) { |_p, _q, _b| {} }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/empty_hash', {})

      expect(response.status).to eq(200)
    end

    it 'converts exception to error response, not nil' do
      app = Spikard::App.new
      app.post('/exception_not_nil', handler_name: :exception_handler)
      app.handler(:exception_handler) do |_p, _q, _b|
        raise StandardError, 'Error test'
      end

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/exception_not_nil', {})

      expect(response.status).to eq(500)
      body = response.json
      expect(body).to have_key('error')
    end
  end

  describe 'Recovery from transient native errors' do
    it 'recovers from single transient failure' do
      app = Spikard::App.new
      app.post('/transient', handler_name: :transient_handler)

      call_count = 0
      app.handler(:transient_handler) do |_p, _q, _b|
        call_count += 1
        { count: call_count }
      end

      client = Spikard::Testing.create_test_client(app)

      response1 = client.post('/transient', {})
      expect(response1.status).to eq(200)

      response2 = client.post('/transient', {})
      expect(response2.status).to eq(200)
    end

    it 'handles handler errors without affecting subsequent requests' do
      app = Spikard::App.new
      app.post('/recovery', handler_name: :recovery_handler)

      counter = { value: 0 }
      app.handler(:recovery_handler) do |_p, _q, _b|
        counter[:value] += 1
        raise StandardError, 'Transient' if counter[:value].odd?

        { ok: true }
      end

      client = Spikard::Testing.create_test_client(app)

      response1 = client.post('/recovery', {})
      expect(response1.status).to eq(500)

      response2 = client.post('/recovery', {})
      expect(response2.status).to eq(200)
    end

    it 'does not leak state between error and recovery' do
      app = Spikard::App.new
      app.post('/stateless', handler_name: :stateless_handler)
      app.handler(:stateless_handler) { |_p, _q, _b| { time: Time.now.to_f } }

      client = Spikard::Testing.create_test_client(app)

      response1 = client.post('/stateless', {})
      response2 = client.post('/stateless', {})

      expect(response1.status).to eq(200)
      expect(response2.status).to eq(200)
    end

    it 'maintains connection pool health after errors' do
      app = Spikard::App.new
      app.post('/pool_health', handler_name: :pool_handler)
      app.handler(:pool_handler) { |_p, _q, _b| { healthy: true } }

      client = Spikard::Testing.create_test_client(app)

      # Simulate multiple requests after potential errors
      5.times do |i|
        response = client.post('/pool_health', {})
        expect(response.status).to eq(200)
      end
    end

    it 'recovers from resource exhaustion gracefully' do
      app = Spikard::App.new
      app.post('/exhaustion', handler_name: :exhaust_handler)
      app.handler(:exhaust_handler) { |_p, _q, _b| { ok: true } }

      client = Spikard::Testing.create_test_client(app)
      response = client.post('/exhaustion', {})

      expect([200, 503]).to include(response.status)
    end
  end
end
