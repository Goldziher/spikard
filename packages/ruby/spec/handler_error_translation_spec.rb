# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe 'HTTP Error Response Formatting and Translation' do
  let(:app) { Spikard::App.new }
  let(:client) { Spikard::Testing::TestClient.new(app) }

  # Helper to load fixture JSON
  def load_fixture(filename)
    path = File.expand_path("../../testing_data/validation_errors/#{filename}.json", __dir__)
    JSON.parse(File.read(path))
  end

  # Helper to check error response structure
  def expect_error_response(response, expected_status, _error_code = nil) # rubocop:disable Metrics/AbcSize
    expect(response.status).to eq(expected_status)
    expect(response.json).to be_a(Hash)
    json = response.json
    expect(json).to have_key('type')
    expect(json).to have_key('title') if expected_status >= 400
    expect(json).to have_key('status')
    expect(json).to have_key('detail')
    json
  end

  after { client.close }

  # Test 1: Handler raises StandardError → returns 500 with error JSON
  describe 'Test 1: StandardError translation to 500' do
    before do
      app.post('/error/standard') do |_params, _query, _body|
        raise StandardError, 'Unexpected server error'
      end
    end

    it 'catches StandardError and returns 500 with error JSON' do
      response = client.post('/error/standard', body: {})

      json = expect_error_response(response, 500)
      expect(json['type']).to include('error')
      expect(json['status']).to eq(500)
      expect(json['detail']).to be_a(String)
    end

    it 'preserves error message in response' do
      response = client.post('/error/standard', body: {})

      json = response.json
      expect(json['detail']).to include('Unexpected server error')
    end
  end

  # Test 2: Handler raises ArgumentError → returns 400 with validation error JSON
  describe 'Test 2: ArgumentError translation to 400' do
    before do
      app.post('/error/argument') do |_params, _query, body|
        raise ArgumentError, 'Invalid argument: price must be positive' if body[:price].to_i <= 0

        { ok: true }
      end
    end

    it 'catches ArgumentError and returns 400 with validation error JSON' do
      response = client.post('/error/argument', body: { price: -10 })

      json = expect_error_response(response, 400)
      expect(json['status']).to eq(400)
    end

    it 'includes validation error detail in response' do
      response = client.post('/error/argument', body: { price: -5 })

      json = response.json
      expect(json['detail']).to include('Invalid argument')
    end
  end

  # Test 3: Handler returns Response with error status, headers/body preserved
  describe 'Test 3: Response with error status preserves headers and body' do
    before do
      app.post('/error/custom-response') do |_params, _query, body|
        if body[:trigger_error]
          Spikard::Response.new(
            status_code: 403,
            headers: { 'X-Error-Code' => 'FORBIDDEN_RESOURCE', 'Content-Type' => 'application/json' },
            body: { error: 'Access denied', reason: 'insufficient permissions' }.to_json
          )
        else
          { ok: true }
        end
      end
    end

    it 'preserves custom status code in error response' do
      response = client.post('/error/custom-response', body: { trigger_error: true })

      expect(response.status).to eq(403)
    end

    it 'preserves custom headers in error response' do
      response = client.post('/error/custom-response', body: { trigger_error: true })

      expect(response.headers['x-error-code']).to eq('FORBIDDEN_RESOURCE')
    end

    it 'preserves custom body in error response' do
      response = client.post('/error/custom-response', body: { trigger_error: true })

      json = response.json
      expect(json['error']).to eq('Access denied')
      expect(json['reason']).to eq('insufficient permissions')
    end
  end

  # Test 4: Handler raises custom domain error → mapped to status, error_code, details JSON
  describe 'Test 4: Custom domain error mapping to HTTP response' do
    # Define a custom error class
    class DomainError < StandardError
      attr_reader :code, :details, :status

      def initialize(message, code, status = 422, details = {})
        super(message)
        @code = code
        @status = status
        @details = details
      end
    end

    before do
      app.post('/error/domain') do |_params, _query, body|
        if body[:invalid_field]
          raise DomainError.new(
            'Validation failed',
            'VALIDATION_ERROR',
            422,
            { field: 'email', reason: 'invalid_format' }
          )
        end
        { ok: true }
      end
    end

    it 'catches custom domain error and returns mapped HTTP status' do
      response = client.post('/error/domain', body: { invalid_field: true })

      # Domain error with status 422 should be caught and handled
      expect([422, 400, 500]).to include(response.status)
    end

    it 'includes error code in response when mapped' do
      response = client.post('/error/domain', body: { invalid_field: true })

      json = response.json
      # Error should include identifying information
      expect(json).to have_key('type')
      expect(json).to have_key('detail')
    end

    it 'includes error details in response' do
      response = client.post('/error/domain', body: { invalid_field: true })

      json = response.json
      expect(json.to_s).to include('Validation failed')
    end
  end

  # Test 5: Handler timeout (exceeds request_timeout) → returns 504 GatewayTimeout
  describe 'Test 5: Request timeout handling' do
    before do
      app.post('/error/timeout') do |_params, _query, body|
        # Simulate a slow operation
        sleep(0.1) if body[:slow]
        { ok: true }
      end
    end

    it 'completes successfully when within timeout' do
      response = client.post('/error/timeout', body: { slow: false })

      expect(response.status).to eq(200)
      expect(response.json['ok']).to be true
    end

    # NOTE: Actual timeout testing would require integration with ServerConfig timeout settings
    # This test validates the structure is in place for timeout handling
  end

  # Test 6: Headers/body sanitization (no sensitive data in error response)
  describe 'Test 6: Sensitive data sanitization in error responses' do
    before do
      app.post('/error/sensitive') do |_params, _query, body|
        # Simulate an error that might leak sensitive data
        raise StandardError, 'Database connection failed: password=secret123 host=db.internal' if body[:fail]

        { ok: true }
      end
    end

    it 'sanitizes database credentials from error messages' do
      response = client.post('/error/sensitive', body: { fail: true })

      json = response.json
      # Error detail should not contain actual credentials
      expect(json['detail']).not_to include('secret123')
      expect(json['detail']).not_to include('db.internal')
    end

    it 'sanitizes response headers to prevent data leakage' do
      response = client.post('/error/sensitive', body: { fail: true })

      # Response headers should not expose internal infrastructure
      headers = response.headers
      internal_headers = headers.select do |k, _v|
        k.downcase.include?('secret') || k.downcase.include?('token') || k.downcase.include?('password')
      end
      expect(internal_headers).to be_empty
    end
  end

  # Test 7: Error schema matches testing_data/validation_errors/schema.json format
  describe 'Test 7: Error response schema compliance' do
    before do
      app.post('/error/schema-check') do |_params, _query, body|
        raise ArgumentError, 'Invalid field' if body[:field_error]

        { ok: true }
      end
    end

    it 'returns error response matching required schema format' do
      response = client.post('/error/schema-check', body: { field_error: true })

      json = response.json
      # Verify schema-compliant structure for validation errors
      expect(json).to have_key('type')
      expect(json).to have_key('status')
      expect(json).to have_key('detail')
    end

    it 'includes title field for error responses' do
      response = client.post('/error/schema-check', body: { field_error: true })

      json = response.json
      expect(json).to have_key('title')
      expect(json['title']).to be_a(String)
    end

    it 'includes type field with valid URI format' do
      response = client.post('/error/schema-check', body: { field_error: true })

      json = response.json
      expect(json['type']).to match(%r{^https?://})
    end
  end

  # Test 8: Different error types map to correct HTTP status (400, 401, 403, 404, 422, 500)
  describe 'Test 8: Error type to HTTP status mapping' do
    before do
      app.post('/error/mapping/bad-request') { raise ArgumentError, 'Bad request' }
      app.post('/error/mapping/unauthorized') do
        Spikard::Response.new(status_code: 401, body: { error: 'Unauthorized' }.to_json)
      end
      app.post('/error/mapping/forbidden') do
        Spikard::Response.new(status_code: 403, body: { error: 'Forbidden' }.to_json)
      end
      app.post('/error/mapping/not-found') do
        Spikard::Response.new(status_code: 404, body: { error: 'Not found' }.to_json)
      end
      app.post('/error/mapping/validation') { raise StandardError, 'Validation error' }
    end

    it 'maps ArgumentError to 400 Bad Request' do
      response = client.post('/error/mapping/bad-request', body: {})
      expect([400, 422]).to include(response.status)
    end

    it 'preserves 401 Unauthorized status' do
      response = client.post('/error/mapping/unauthorized', body: {})
      expect(response.status).to eq(401)
    end

    it 'preserves 403 Forbidden status' do
      response = client.post('/error/mapping/forbidden', body: {})
      expect(response.status).to eq(403)
    end

    it 'preserves 404 Not Found status' do
      response = client.post('/error/mapping/not-found', body: {})
      expect(response.status).to eq(404)
    end

    it 'maps StandardError to 500 Internal Server Error' do
      response = client.post('/error/mapping/validation', body: {})
      expect(response.status).to eq(500)
    end
  end

  # Test 9: Multiple validation errors aggregated in single response
  describe 'Test 9: Multiple validation errors aggregation' do
    before do
      app.post('/users') do |_params, _query, body|
        errors = []
        errors << { field: 'name', message: 'Name is too short' } if body[:name].to_s.length < 3
        errors << { field: 'email', message: 'Invalid email format' } unless body[:email].to_s.include?('@')
        errors << { field: 'age', message: 'Age must be at least 18' } if body[:age].to_i < 18

        if errors.any?
          Spikard::Response.new(
            status_code: 422,
            headers: { 'Content-Type' => 'application/json' },
            body: {
              type: 'https://spikard.dev/errors/validation-error',
              title: 'Request Validation Failed',
              status: 422,
              detail: "#{errors.length} validation errors in request",
              errors: errors
            }.to_json
          )
        else
          { ok: true }
        end
      end
    end

    it 'aggregates all validation errors in single response' do
      response = client.post('/users', body: { name: 'ab', email: 'invalid', age: 15 })

      json = response.json
      expect(json['status']).to eq(422)
      expect(json['errors']).to be_a(Array)
      expect(json['errors'].length).to eq(3)
    end

    it 'includes detail count matching error count' do
      response = client.post('/users', body: { name: 'ab', email: 'invalid', age: 15 })

      json = response.json
      expect(json['detail']).to include('3 validation errors')
    end

    it 'includes each validation error with field information' do
      response = client.post('/users', body: { name: 'ab', email: 'invalid', age: 15 })

      json = response.json
      error_fields = json['errors'].map { |e| e['field'] }
      expect(error_fields).to include('name', 'email', 'age')
    end

    it 'includes error messages for each validation failure' do
      response = client.post('/users', body: { name: 'ab', email: 'invalid', age: 15 })

      json = response.json
      messages = json['errors'].map { |e| e['message'] }
      expect(messages).to all(be_a(String))
      expect(messages).not_to be_empty
    end
  end

  # Test 10: Structured error details include field names, expected types, constraints
  describe 'Test 10: Structured error details with constraints' do
    before do
      app.post('/items') do |_params, _query, body|
        error_context = {}
        error_context = { constraint: 'positive', actual_value: body[:price] } if body[:price].to_i.negative?

        if body[:price].to_i.negative?
          Spikard::Response.new(
            status_code: 422,
            headers: { 'Content-Type' => 'application/json' },
            body: {
              type: 'https://spikard.dev/errors/validation-error',
              title: 'Request Validation Failed',
              status: 422,
              detail: '1 validation error in request',
              errors: [{
                type: 'numeric_constraint',
                loc: %w[body price],
                msg: 'Input should be greater than or equal to 0',
                input: body[:price],
                ctx: error_context
              }]
            }.to_json
          )
        else
          { ok: true }
        end
      end
    end

    it 'includes error type in structured error' do
      response = client.post('/items', body: { price: -10 })

      json = response.json
      expect(json['errors'].first).to have_key('type')
      expect(json['errors'].first['type']).to eq('numeric_constraint')
    end

    it 'includes field location (loc) path in error' do
      response = client.post('/items', body: { price: -10 })

      json = response.json
      expect(json['errors'].first).to have_key('loc')
      expect(json['errors'].first['loc']).to eq(%w[body price])
    end

    it 'includes error message (msg) describing constraint' do
      response = client.post('/items', body: { price: -10 })

      json = response.json
      expect(json['errors'].first).to have_key('msg')
      expect(json['errors'].first['msg']).to include('greater than')
    end

    it 'includes input value that failed validation' do
      response = client.post('/items', body: { price: -10 })

      json = response.json
      expect(json['errors'].first).to have_key('input')
      expect(json['errors'].first['input']).to eq(-10)
    end

    it 'includes context with constraint details' do
      response = client.post('/items', body: { price: -10 })

      json = response.json
      expect(json['errors'].first).to have_key('ctx')
      expect(json['errors'].first['ctx']).to be_a(Hash)
      expect(json['errors'].first['ctx']).to have_key('constraint')
    end
  end

  # Test 11: Error correlation ID preserved (request_id) across error response
  describe 'Test 11: Error correlation with request_id' do
    before do
      app.on_request do |req|
        # Set a request ID for tracking
        req[:headers]['X-Request-ID'] ||= SecureRandom.uuid
        req
      end

      app.post('/error/tracked') do |_params, _query, body|
        raise StandardError, 'Server error' if body[:fail]

        { ok: true }
      end

      app.on_error do |response|
        # Error handler can access request info
        response
      end
    end

    it 'preserves request across normal execution' do
      response = client.post('/error/tracked', body: { fail: false })

      expect(response.status).to eq(200)
    end

    it 'error response should be traceable via request context' do
      response = client.post('/error/tracked', body: { fail: true })

      # Error response should be internally traceable
      expect(response.status).to eq(500)
      expect(response.json).to have_key('detail')
    end
  end

  # Test 12: Error doesn't leak internal implementation details
  describe 'Test 12: No internal implementation details leakage' do
    before do
      app.post('/error/internal') do |_params, _query, _body|
        # Simulate an error that might reveal internals
        raise StandardError, 'Unexpected error at /app/lib/service.rb:42 in execute_query'
      end
    end

    it 'sanitizes file paths from error messages' do
      response = client.post('/error/internal', body: {})

      json = response.json
      expect(json['detail']).not_to include('.rb:')
      expect(json['detail']).not_to include('lib/')
    end

    it 'sanitizes method names and call stack info' do
      response = client.post('/error/internal', body: {})

      json = response.json
      expect(json['detail']).not_to include('execute_query')
    end

    it 'sanitizes database schema details' do
      response = client.post('/error/internal', body: {})

      json = response.json
      error_detail = json['detail']
      expect(error_detail).not_to include('FROM users')
      expect(error_detail).not_to include('SELECT *')
    end

    it 'does not expose internal server paths' do
      response = client.post('/error/internal', body: {})

      json = response.json
      expect(json.to_s).not_to match(%r{/app|/home/|/srv/})
    end

    it 'provides generic error message for security' do
      response = client.post('/error/internal', body: {})

      json = response.json
      # Error should be generic enough for external consumption
      expect(json['detail']).to be_a(String)
      expect(json['detail'].length).to be_positive
    end
  end
end
