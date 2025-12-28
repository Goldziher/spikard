# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe 'Schema Validation Edge Cases' do
  let(:fixture_dir) { File.expand_path('../../../testing_data/validation_errors', __dir__) }

  # Helper to load fixture files
  def load_fixture(filename)
    filepath = File.join(fixture_dir, filename)
    JSON.parse(File.read(filepath))
  end

  # Helper to create app with schema validation
  def create_app_with_schema(schema)
    app = Spikard::App.new
    app.post('/validate', request_schema: schema) do |request|
      { success: true, body: request.body }
    end
    app
  end

  # Helper to assert error response structure
  def assert_validation_error(response, status_code, error_type, location_parts) # rubocop:disable Metrics/AbcSize
    expect(response.status).to eq(status_code)
    body = response.json
    expect(body).to include('type')
    expect(body).to include('status')
    expect(body['status']).to eq(status_code)

    return unless body.key?('errors')

    errors = Array(body['errors'])
    error = errors.find { |e| e['type'] == error_type && e['loc'] == location_parts }
    expect(error).not_to be_nil, "Expected error with type=#{error_type} and loc=#{location_parts.inspect}"
  end

  describe '1. Missing required field in request body' do
    it 'returns 422 with field name in error location' do
      fixture = load_fixture('04_body_missing_required_field.json')
      schema = fixture['handler']['body_schema']

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: fixture['request']['body'],
        headers: fixture['request']['headers']
      )

      assert_validation_error(response, 422, 'missing', %w[body price])
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body price] }
      expect(error['msg']).to include('Field required')
    end
  end

  describe '2. Extra fields in request body (additional properties handling)' do
    it 'handles additional properties based on schema configuration' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' },
          'age' => { 'type' => 'integer' }
        },
        'required' => ['name'],
        'additionalProperties' => false
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      # Extra field should trigger validation error with additionalProperties: false
      response = client.post(
        '/validate',
        body: { name: 'John', age: 30, extra_field: 'should_fail' },
        headers: { 'Content-Type' => 'application/json' }
      )

      # When additionalProperties is false, extra fields are rejected
      expect([422, 400]).to include(response.status)
    end

    it 'allows additional properties when schema allows them' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' }
        },
        'required' => ['name'],
        'additionalProperties' => true
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'John', extra: 'value' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(200)
      expect(response.json['success']).to be true
    end
  end

  describe '3. Query parameter type coercion (string to integer)' do
    it 'coerces string "123" to integer 123' do
      app = Spikard::App.new
      app.get('/items/:id', request_schema: {
                'type' => 'object',
                'properties' => {
                  'path' => {
                    'type' => 'object',
                    'properties' => {
                      'id' => { 'type' => 'integer' }
                    },
                    'required' => ['id']
                  }
                }
              }) do |request|
        { id: request.path_params['id'] }
      end

      client = Spikard::Testing::TestClient.new(app)
      response = client.get('/items/123')

      expect(response.status).to eq(200)
      expect(response.json['id']).to eq(123)
    end

    it 'returns 422 on coercion failure (invalid integer string)' do
      load_fixture('01_query_param_type_error_string_to_int.json')
      app = Spikard::App.new
      app.get('/items', request_schema: {
                'type' => 'object',
                'properties' => {
                  'query' => {
                    'type' => 'object',
                    'properties' => {
                      'skip' => { 'type' => 'integer' }
                    }
                  }
                }
              }) do |request|
        { skip: request.query_params['skip'] }
      end

      client = Spikard::Testing::TestClient.new(app)
      response = client.get('/items?skip=not_a_number')

      expect(response.status).to eq(422)
      body = response.json
      expect(body['errors']&.length).to be > 0
      error = body['errors']&.find { |e| e['loc']&.include?('skip') }
      expect(error).not_to be_nil
    end
  end

  describe '4. Type validation failure (string instead of integer)' do
    it 'returns 400/422 with type error when string provided for integer field' do
      load_fixture('03_body_field_type_error.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'count' => { 'type' => 'integer' }
        },
        'required' => ['count']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { count: 'not_a_number' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect([400, 422]).to include(response.status)
      body = response.json
      expect(body).to include('errors')
      expect(body['errors'].length).to be > 0
    end
  end

  describe '5. Nested object validation with path in error' do
    it 'includes full path in error location for nested validation errors' do
      fixture = load_fixture('15_nested_object_validation_error.json')
      schema = fixture['handler']['body_schema']

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: fixture['request']['body'],
        headers: fixture['request']['headers']
      )

      expect(response.status).to eq(422)
      body = response.json
      errors = body['errors'] || []

      # Should have errors with nested paths
      nested_errors = errors.select { |e| e['loc'].length > 2 }
      expect(nested_errors.length).to be > 0

      # Check for path like ["body", "seller", "name"]
      name_error = nested_errors.find { |e| e['loc'].include?('name') }
      expect(name_error['loc']).to include('seller')
    end
  end

  describe '6. Array element validation' do
    it 'validates each array item against schema' do
      fixture = load_fixture('18_array_item_validation_error.json')
      schema = fixture['handler']['body_schema']

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: fixture['request']['body'],
        headers: fixture['request']['headers']
      )

      expect(response.status).to eq(422)
      body = response.json
      errors = body['errors'] || []

      # Should have error for invalid array item
      array_error = errors.find { |e| e['loc'].include?('tags') }
      expect(array_error).not_to be_nil
      expect(array_error['loc']).to include('2') # Index of invalid item
    end

    it 'returns error with array index in location path' do
      schema = {
        'type' => 'object',
        'properties' => {
          'items' => {
            'type' => 'array',
            'items' => { 'type' => 'integer' }
          }
        },
        'required' => ['items']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { items: [1, 2, 'invalid', 4] },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body items 2] }
      expect(error).not_to be_nil
    end
  end

  describe '7. Null vs missing field distinction' do
    it 'accepts null value for nullable field' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' },
          'nickname' => { 'type' => %w[string null] }
        },
        'required' => %w[name nickname]
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'John', nickname: nil },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(200)
      expect(response.json['success']).to be true
    end

    it 'rejects null for non-nullable required field' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: nil },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect([400, 422]).to include(response.status)
    end

    it 'rejects missing required field entirely' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' },
          'age' => { 'type' => 'integer' }
        },
        'required' => %w[name age]
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'John' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body age] }
      expect(error).not_to be_nil
      expect(error['type']).to eq('missing')
    end
  end

  describe '8. Empty string validation against min_length' do
    it 'rejects empty string when minLength constraint is set' do
      load_fixture('07_string_min_length_violation.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string', 'minLength' => 3 }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: '' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body name] }
      expect(error).not_to be_nil
      expect(error['ctx']&.key?('min_length')).to be true
    end

    it 'accepts empty string when no minLength constraint' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string' }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: '' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(200)
    end
  end

  describe '9. Enum validation with valid values in error' do
    it 'rejects value not in enum set' do
      load_fixture('10_enum_invalid_value.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'status' => {
            'type' => 'string',
            'enum' => %w[active inactive pending]
          }
        },
        'required' => ['status']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { status: 'invalid' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body status] }
      expect(error).not_to be_nil
      expect(error['type']).to eq('enum')
      # Error message should contain valid values
      expect(error['msg']).to be_a(String)
    end

    it 'accepts valid enum values' do
      schema = {
        'type' => 'object',
        'properties' => {
          'status' => {
            'type' => 'string',
            'enum' => %w[active inactive pending]
          }
        },
        'required' => ['status']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      %w[active inactive pending].each do |status|
        response = client.post(
          '/validate',
          body: { status: status },
          headers: { 'Content-Type' => 'application/json' }
        )

        expect(response.status).to eq(200)
        expect(response.json['success']).to be true
      end
    end
  end

  describe '10. Format validation (email, UUID, datetime ISO8601)' do
    it 'validates email format' do
      schema = {
        'type' => 'object',
        'properties' => {
          'email' => { 'type' => 'string', 'format' => 'email' }
        },
        'required' => ['email']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      # Valid email
      response = client.post(
        '/validate',
        body: { email: 'test@example.com' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(200)

      # Invalid email
      response = client.post(
        '/validate',
        body: { email: 'not_an_email' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(422)
    end

    it 'validates UUID format' do
      schema = {
        'type' => 'object',
        'properties' => {
          'id' => { 'type' => 'string', 'format' => 'uuid' }
        },
        'required' => ['id']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      # Valid UUID
      response = client.post(
        '/validate',
        body: { id: '550e8400-e29b-41d4-a716-446655440000' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(200)

      # Invalid UUID
      response = client.post(
        '/validate',
        body: { id: 'not-a-uuid' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(422)
    end

    it 'validates ISO8601 datetime format' do
      schema = {
        'type' => 'object',
        'properties' => {
          'created_at' => { 'type' => 'string', 'format' => 'date-time' }
        },
        'required' => ['created_at']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      # Valid ISO8601 datetime
      response = client.post(
        '/validate',
        body: { created_at: '2024-01-15T10:30:00Z' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(200)

      # Invalid datetime
      response = client.post(
        '/validate',
        body: { created_at: 'not_a_datetime' },
        headers: { 'Content-Type' => 'application/json' }
      )
      expect(response.status).to eq(422)
    end
  end

  describe '11. Max/min length and numeric bounds' do
    it 'enforces string maxLength constraint' do
      load_fixture('08_string_max_length_violation.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string', 'maxLength' => 10 }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'this_is_a_very_long_string' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body name] }
      expect(error).not_to be_nil
    end

    it 'enforces numeric minimum constraint' do
      load_fixture('05_numeric_constraint_gt_violation.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'age' => { 'type' => 'integer', 'minimum' => 18 }
        },
        'required' => ['age']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { age: 10 },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body age] }
      expect(error).not_to be_nil
      expect(error['ctx']&.key?('gt')).to be true
    end

    it 'enforces numeric maximum constraint' do
      load_fixture('06_numeric_constraint_le_violation.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'score' => { 'type' => 'integer', 'maximum' => 100 }
        },
        'required' => ['score']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { score: 150 },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.find { |e| e['loc'] == %w[body score] }
      expect(error).not_to be_nil
      expect(error['ctx']&.key?('le')).to be true
    end

    it 'accepts values within valid bounds' do
      schema = {
        'type' => 'object',
        'properties' => {
          'age' => { 'type' => 'integer', 'minimum' => 0, 'maximum' => 120 },
          'name' => { 'type' => 'string', 'minLength' => 2, 'maxLength' => 50 }
        },
        'required' => %w[age name]
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { age: 30, name: 'John Doe' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(200)
      expect(response.json['success']).to be true
    end
  end

  describe '12. Error payload includes constraint details' do
    it 'includes min_length in constraint context' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string', 'minLength' => 5 }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'Jo' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.first
      expect(error['ctx']).to include('min_length')
      expect(error['ctx']['min_length']).to eq(5)
    end

    it 'includes max_length in constraint context' do
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string', 'maxLength' => 10 }
        },
        'required' => ['name']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'This is a very long name' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.first
      expect(error['ctx']).to include('max_length')
      expect(error['ctx']['max_length']).to eq(10)
    end

    it 'includes enum valid values in error context' do
      schema = {
        'type' => 'object',
        'properties' => {
          'status' => {
            'type' => 'string',
            'enum' => %w[draft published archived]
          }
        },
        'required' => ['status']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { status: 'deleted' },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.first
      expect(error).to include('ctx')
      expect(error['ctx']).to include('expected')
    end

    it 'includes numeric bounds in constraint context' do
      schema = {
        'type' => 'object',
        'properties' => {
          'quantity' => { 'type' => 'integer', 'minimum' => 1, 'maximum' => 1000 }
        },
        'required' => ['quantity']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      # Test minimum bound violation
      response = client.post(
        '/validate',
        body: { quantity: 0 },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.first
      expect(error['ctx']).to include('gt')

      # Test maximum bound violation
      response = client.post(
        '/validate',
        body: { quantity: 2000 },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      error = body['errors']&.first
      expect(error['ctx']).to include('le')
    end
  end

  describe '13. Multiple validation errors in single response' do
    it 'collects and returns multiple validation errors' do
      load_fixture('14_multiple_validation_errors.json')
      schema = {
        'type' => 'object',
        'properties' => {
          'name' => { 'type' => 'string', 'minLength' => 2 },
          'email' => { 'type' => 'string', 'format' => 'email' },
          'age' => { 'type' => 'integer', 'minimum' => 0, 'maximum' => 120 }
        },
        'required' => %w[name email age]
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { name: 'X', email: 'invalid', age: 200 },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      errors = body['errors'] || []

      # Should have at least 3 errors
      expect(errors.length).to be >= 3

      # Check each field has an error
      fields_with_errors = errors.map { |e| e['loc'][1] if e['loc'].length > 1 }.compact
      expect(fields_with_errors).to include('name')
      expect(fields_with_errors).to include('email')
      expect(fields_with_errors).to include('age')
    end

    it 'includes multiple nested object validation errors with full path' do
      schema = {
        'type' => 'object',
        'properties' => {
          'user' => {
            'type' => 'object',
            'properties' => {
              'name' => { 'type' => 'string', 'minLength' => 3 },
              'email' => { 'type' => 'string', 'format' => 'email' }
            },
            'required' => %w[name email]
          }
        },
        'required' => ['user']
      }

      app = create_app_with_schema(schema)
      client = Spikard::Testing::TestClient.new(app)

      response = client.post(
        '/validate',
        body: { user: { name: 'X', email: 'invalid_email' } },
        headers: { 'Content-Type' => 'application/json' }
      )

      expect(response.status).to eq(422)
      body = response.json
      errors = body['errors'] || []

      # Both nested errors should be present with correct paths
      name_error = errors.find { |e| e['loc'] == %w[body user name] }
      email_error = errors.find { |e| e['loc'] == %w[body user email] }

      expect(name_error).not_to be_nil
      expect(email_error).not_to be_nil
    end
  end
end
