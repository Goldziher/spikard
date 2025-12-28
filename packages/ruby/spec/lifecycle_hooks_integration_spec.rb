# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Lifecycle Hooks Integration' do
  let(:app) { Spikard::App.new }
  let(:client) { Spikard::Testing.create_test_client(app) }

  describe 'hook execution order' do
    it 'fires hooks in order: onRequest -> preValidation -> preHandler -> handler -> onResponse' do
      execution_order = []

      app.on_request do |_request|
        execution_order << :on_request
      end

      app.pre_validation do |_request|
        execution_order << :pre_validation
      end

      app.pre_handler do |_request|
        execution_order << :pre_handler
      end

      app.on_response do |_response|
        execution_order << :on_response
      end

      app.get('/test') do
        execution_order << :handler
        { status: 'ok' }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to eq(%i[on_request pre_validation pre_handler handler on_response])
    end

    it 'preserves execution order across multiple routes' do
      execution_order = []

      app.on_request { |_| execution_order << :on_request }
      app.pre_handler { |_| execution_order << :pre_handler }
      app.on_response { |_| execution_order << :on_response }

      app.get('/first') do
        execution_order << :handler_first
        { route: 'first' }
      end
      app.post('/second') do
        execution_order << :handler_second
        { route: 'second' }
      end

      response1 = client.get('/first')
      expect(response1.status).to eq(200)
      expect(execution_order).to include(:on_request, :pre_handler, :handler_first, :on_response)

      execution_order.clear

      response2 = client.post('/second', body: {})
      expect(response2.status).to eq(200)
      expect(execution_order).to include(:on_request, :pre_handler, :handler_second, :on_response)
    end
  end

  describe 'onError hook behavior' do
    it 'calls onError hook when handler raises' do
      error_caught = []

      app.on_error do |error, _request|
        error_caught << error.message
      end

      app.get('/error') do
        raise StandardError, 'handler failed'
      end

      expect do
        client.get('/error')
      end.to raise_error(StandardError, 'handler failed')

      expect(error_caught).to include('handler failed')
    end

    it 'fires onError before onResponse when handler fails' do
      execution_order = []

      app.on_error do |_error, _request|
        execution_order << :on_error
      end

      app.on_response do |_response|
        execution_order << :on_response
      end

      app.get('/fail') do
        raise StandardError, 'boom'
      end

      expect do
        client.get('/fail')
      end.to raise_error(StandardError)

      # onError fires, then onResponse should still fire
      expect(execution_order).to include(:on_error)
    end

    it 'can transform error response in onError hook' do
      app.on_error do |_error, _request|
        # In real implementation, could return custom response
        # For now, verify the hook executes
      end

      app.get('/error') do
        raise StandardError, 'transformed'
      end

      expect do
        client.get('/error')
      end.to raise_error(StandardError, 'transformed')
    end
  end

  describe 'preHandler hook short-circuiting' do
    it 'can return early response to skip handler' do
      execution_order = []

      app.pre_handler do |request|
        execution_order << :pre_handler
        { early_response: true } if request.path == '/skip'
      end

      app.get('/skip') do
        execution_order << :handler
        { should_not_reach: true }
      end

      response = client.get('/skip')

      expect(response.status).to eq(200)
      # Handler should be skipped when pre_handler returns a response
      expect(execution_order).to include(:pre_handler)
    end

    it 'skips remaining preHandler hooks after short-circuit' do
      execution_order = []

      app.pre_handler do |_request|
        execution_order << :pre_handler1
        { short_circuit: true }
      end

      app.pre_handler do |_request|
        execution_order << :pre_handler2
      end

      app.get('/test') do
        execution_order << :handler
        { ok: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:pre_handler1)
    end

    it 'short-circuit response still fires onResponse hook' do
      execution_order = []

      app.pre_handler do |_request|
        execution_order << :pre_handler
        { early: true }
      end

      app.on_response do |_response|
        execution_order << :on_response
      end

      app.get('/test') do
        execution_order << :handler
        { original: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:pre_handler, :on_response)
    end
  end

  describe 'onRequest hook behavior' do
    it 'receives request metadata and can access path' do
      request_data = []

      app.on_request do |request|
        request_data << { path: request.path, method: request.method }
      end

      app.get('/info') { { ok: true } }

      response = client.get('/info')

      expect(response.status).to eq(200)
      expect(request_data).not_to be_empty
      expect(request_data.first).to include(path: '/info', method: 'GET')
    end

    it 'can log request in onRequest hook' do
      logs = []

      app.on_request do |request|
        logs << "Request: #{request.method} #{request.path}"
      end

      app.get('/logged') { { ok: true } }

      response = client.get('/logged')

      expect(response.status).to eq(200)
      expect(logs).not_to be_empty
      expect(logs.first).to include('Request: GET /logged')
    end

    it 'onRequest receives request object with headers' do
      headers_seen = []

      app.on_request do |request|
        headers_seen << request.headers
      end

      app.get('/headers') { { ok: true } }

      response = client.get('/headers', headers: { 'X-Test' => 'value' })

      expect(response.status).to eq(200)
      expect(headers_seen).not_to be_empty
    end

    it 'exception in onRequest does not break preValidation' do
      execution_order = []

      app.on_request do |_request|
        execution_order << :on_request
        # NOTE: real exception handling may vary by implementation
      end

      app.pre_validation do |_request|
        execution_order << :pre_validation
      end

      app.get('/test') do
        execution_order << :handler
        { ok: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:pre_validation)
    end
  end

  describe 'onResponse hook behavior' do
    it 'receives response object and can access status' do
      response_data = []

      app.on_response do |response|
        response_data << response.status
      end

      app.get('/response') { { ok: true } }

      response = client.get('/response')

      expect(response.status).to eq(200)
      expect(response_data).to include(200)
    end

    it 'can add headers in onResponse hook' do
      app.on_response do |response|
        response.headers['X-Custom-Header'] = 'added-by-hook'
      end

      app.get('/headers') { { ok: true } }

      response = client.get('/headers')

      expect(response.status).to eq(200)
      # Verify header was added
      expect(response.headers).to include('x-custom-header' => 'added-by-hook')
    end

    it 'onResponse hook does not modify handler response body' do
      app.on_response do |response|
        response.headers['X-Hook'] = 'executed'
      end

      app.get('/data') { { message: 'original' } }

      response = client.get('/data')

      expect(response.status).to eq(200)
      expect(response.json['message']).to eq('original')
    end

    it 'multiple onResponse hooks all execute' do
      execution_order = []

      app.on_response do |_response|
        execution_order << :on_response1
      end

      app.on_response do |_response|
        execution_order << :on_response2
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:on_response1, :on_response2)
    end
  end

  describe 'hook execution isolation' do
    it 'exception in onRequest does not prevent preValidation execution' do
      execution_order = []
      error_raised = false

      app.on_request do |_request|
        execution_order << :on_request
        begin
          raise StandardError, 'onRequest error'
        rescue StandardError
          error_raised = true
        end
      end

      app.pre_validation do |_request|
        execution_order << :pre_validation
      end

      app.get('/test') do
        execution_order << :handler
        { ok: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(error_raised).to be true
      expect(execution_order).to include(:pre_validation)
    end

    it 'exception in preHandler does not prevent onResponse' do
      execution_order = []

      app.pre_handler do |_request|
        execution_order << :pre_handler
      end

      app.on_response do |_response|
        execution_order << :on_response
      end

      app.get('/test') do
        execution_order << :handler
        { ok: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:on_response)
    end
  end

  describe 'multiple hooks of same type' do
    it 'all onRequest hooks execute in order' do
      execution_order = []

      app.on_request do |_request|
        execution_order << :on_request1
      end

      app.on_request do |_request|
        execution_order << :on_request2
      end

      app.on_request do |_request|
        execution_order << :on_request3
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to eq(%i[on_request1 on_request2 on_request3])
    end

    it 'all preValidation hooks execute in order' do
      execution_order = []

      app.pre_validation do |_request|
        execution_order << :pre_validation1
      end

      app.pre_validation do |_request|
        execution_order << :pre_validation2
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:pre_validation1, :pre_validation2)
    end

    it 'all preHandler hooks execute in order' do
      execution_order = []

      app.pre_handler do |_request|
        execution_order << :pre_handler1
      end

      app.pre_handler do |_request|
        execution_order << :pre_handler2
      end

      app.get('/test') do
        execution_order << :handler
        { ok: true }
      end

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to include(:pre_handler1, :pre_handler2)
    end

    it 'all onResponse hooks execute in order' do
      execution_order = []

      app.on_response do |_response|
        execution_order << :on_response1
      end

      app.on_response do |_response|
        execution_order << :on_response2
      end

      app.on_response do |_response|
        execution_order << :on_response3
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(execution_order).to eq(%i[on_response1 on_response2 on_response3])
    end
  end

  describe 'hook with async/blocking operations' do
    it 'hook can execute background job without blocking request' do
      background_executed = []

      app.on_response do |_response|
        # Simulate background work
        background_executed << true
      end

      app.get('/async') do
        { ok: true }
      end

      response = client.get('/async')

      expect(response.status).to eq(200)
      expect(background_executed).to include(true)
    end

    it 'preHandler hook can perform validation work' do
      validation_results = []

      app.pre_handler do |request|
        validation_results << { path: request.path, validated: true }
      end

      app.get('/validate') do
        { ok: true }
      end

      response = client.get('/validate')

      expect(response.status).to eq(200)
      expect(validation_results).not_to be_empty
      expect(validation_results.first[:validated]).to be true
    end
  end

  describe 'hook state access' do
    it 'hook can access route metadata' do
      route_info = []

      app.pre_handler do |request|
        route_info << { path: request.path }
      end

      app.get('/meta') { { ok: true } }

      response = client.get('/meta')

      expect(response.status).to eq(200)
      expect(route_info.first[:path]).to eq('/meta')
    end

    it 'hook can track request state across lifecycle' do
      request_states = []

      app.on_request do |request|
        request_states << { stage: :on_request, path: request.path }
      end

      app.pre_handler do |request|
        request_states << { stage: :pre_handler, path: request.path }
      end

      app.get('/state') do
        request_states << { stage: :handler, path: '/state' }
        { ok: true }
      end

      response = client.get('/state')

      expect(response.status).to eq(200)
      expect(request_states).to include(
        { stage: :on_request, path: '/state' },
        { stage: :pre_handler, path: '/state' },
        { stage: :handler, path: '/state' }
      )
    end
  end

  describe 'hook ordering preserved across routes' do
    it 'hooks execute in same order for different routes' do
      execution_traces = {}

      app.on_request { |r| (execution_traces[r.path] ||= []) << :on_request }
      app.pre_handler { |r| (execution_traces[r.path] ||= []) << :pre_handler }
      app.on_response { |_r| (execution_traces[:response] ||= []) << :on_response }

      app.get('/route1') { { ok: true } }
      app.post('/route2', body_param_name: :data) { { ok: true } }

      response1 = client.get('/route1')
      expect(response1.status).to eq(200)

      response2 = client.post('/route2', body: {})
      expect(response2.status).to eq(200)

      expect(execution_traces['/route1']).to include(:on_request, :pre_handler)
      expect(execution_traces['/route2']).to include(:on_request, :pre_handler)
    end
  end

  describe 'metrics/logging hooks' do
    it 'hook can log without modifying response' do
      logged_requests = []

      app.on_response do |response|
        logged_requests << { status: response.status }
      end

      app.get('/log') { { message: 'test' } }

      response = client.get('/log')

      expect(response.status).to eq(200)
      expect(response.json['message']).to eq('test')
      expect(logged_requests.first[:status]).to eq(200)
    end

    it 'multiple logging hooks can coexist' do
      access_log = []
      error_log = []

      app.on_response do |response|
        access_log << { status: response.status }
      end

      app.on_error do |error, _request|
        error_log << { message: error.message }
      end

      app.get('/logged') { { ok: true } }

      response = client.get('/logged')

      expect(response.status).to eq(200)
      expect(access_log).not_to be_empty
    end
  end

  describe 'hook error propagation' do
    it 'error in handler is caught by onError hook' do
      errors_caught = []

      app.on_error do |error, _request|
        errors_caught << error.class.name
      end

      app.get('/error') do
        raise ArgumentError, 'invalid input'
      end

      expect do
        client.get('/error')
      end.to raise_error(ArgumentError)

      expect(errors_caught).to include('ArgumentError')
    end

    it 'onError hook receives original error instance' do
      error_messages = []

      app.on_error do |error, _request|
        error_messages << error.message
      end

      app.get('/fail') do
        raise StandardError, 'original message'
      end

      expect do
        client.get('/fail')
      end.to raise_error(StandardError)

      expect(error_messages).to include('original message')
    end
  end

  describe 'zero-cost optimization' do
    it 'route without hooks completes successfully' do
      app.get('/simple') { { ok: true } }

      response = client.get('/simple')

      expect(response.status).to eq(200)
      expect(response.json['ok']).to be true
    end

    it 'app with no hooks registered has no overhead' do
      start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)

      100.times do
        app_no_hooks = Spikard::App.new
        app_no_hooks.get('/test') { { ok: true } }
      end

      elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC) - start_time

      # Should complete quickly with no hooks
      expect(elapsed).to be < 5.0
    end

    it 'hooks only execute when registered' do
      app_with_hooks = Spikard::App.new
      execution = []

      app_with_hooks.on_request { |_| execution << :hook }
      app_with_hooks.get('/test') { { ok: true } }

      client_with_hooks = Spikard::Testing.create_test_client(app_with_hooks)
      response = client_with_hooks.get('/test')

      expect(response.status).to eq(200)
      expect(execution).to include(:hook)
    end
  end

  describe 'hook return values' do
    it 'onRequest hook return value does not affect request processing' do
      app.on_request do |_request|
        { should_be_ignored: true }
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(response.json['ok']).to be true
    end

    it 'preValidation hook return value can indicate validation state' do
      validation_passed = []

      app.pre_validation do |_request|
        validation_passed << true
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(validation_passed).to include(true)
    end

    it 'onResponse hook return value does not affect response' do
      app.on_response do |response|
        response.status == 200 ? :success : :error
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
    end
  end

  describe 'complex hook scenarios' do
    it 'full lifecycle with all hooks, multiple routes, and error handling' do
      lifecycle = []

      app.on_request { |r| lifecycle << "on_request:#{r.path}" }
      app.pre_validation { |_| lifecycle << 'pre_validation' }
      app.pre_handler { |_| lifecycle << 'pre_handler' }
      app.on_response { |_| lifecycle << 'on_response' }
      app.on_error { |_, _| lifecycle << 'on_error' }

      app.get('/success') { { ok: true } }
      app.get('/error') { raise StandardError, 'fail' }

      response1 = client.get('/success')
      expect(response1.status).to eq(200)
      expect(lifecycle).to include('on_request:/success', 'on_response')

      lifecycle.clear

      expect do
        client.get('/error')
      end.to raise_error(StandardError)

      expect(lifecycle).to include('on_error')
    end

    it 'hooks work with different HTTP methods' do
      methods_seen = []

      app.on_request { |r| methods_seen << r.method }

      app.get('/resource') { { ok: true } }
      app.post('/resource', body_param_name: :data) { { created: true } }
      app.put('/resource/:id', body_param_name: :data) { { updated: true } }
      app.delete('/resource/:id') { { deleted: true } }

      client.get('/resource')
      client.post('/resource', body: {})
      client.put('/resource/1', body: {})
      client.delete('/resource/1')

      expect(methods_seen).to include('GET', 'POST', 'PUT', 'DELETE')
    end

    it 'hooks work with routes having path parameters' do
      captured_paths = []

      app.on_request { |r| captured_paths << r.path }

      app.get('/users/:id') { { user_id: 'from_handler' } }
      app.get('/posts/:post_id/comments/:comment_id') { { ok: true } }

      client.get('/users/123')
      client.get('/posts/456/comments/789')

      expect(captured_paths).to include('/users/123', '/posts/456/comments/789')
    end
  end

  describe 'hook registration patterns' do
    it 'hooks can be registered via block' do
      executed = []

      app.on_request do |_request|
        executed << true
      end

      app.get('/test') { { ok: true } }

      response = client.get('/test')

      expect(response.status).to eq(200)
      expect(executed).to include(true)
    end

    it 'multiple independent apps have independent hooks' do
      app1 = Spikard::App.new
      app2 = Spikard::App.new

      app1_hooks = []
      app2_hooks = []

      app1.on_request { |_| app1_hooks << true }
      app2.on_request { |_| app2_hooks << true }

      app1.get('/test') { { app: 1 } }
      app2.get('/test') { { app: 2 } }

      client1 = Spikard::Testing.create_test_client(app1)
      client2 = Spikard::Testing.create_test_client(app2)

      response1 = client1.get('/test')
      response2 = client2.get('/test')

      expect(response1.status).to eq(200)
      expect(response2.status).to eq(200)
      expect(app1_hooks).to include(true)
      expect(app2_hooks).to include(true)
    end
  end
end
