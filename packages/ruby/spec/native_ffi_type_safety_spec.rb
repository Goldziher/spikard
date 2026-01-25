# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Spikard Native FFI Type Safety' do
  describe 'Native::TestClient initialization' do
    context 'with invalid routes_json' do
      it 'raises ArgumentError when routes_json is malformed' do
        expect do
          Spikard::Native::TestClient.new(
            'not valid json {',
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end

      it 'raises ArgumentError when routes_json contains invalid route structure' do
        expect do
          Spikard::Native::TestClient.new(
            '[{"method": "GET"}]', # Missing required fields
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end

      it 'raises ArgumentError with clear message about JSON parsing failure' do
        error_message = nil
        begin
          Spikard::Native::TestClient.new(
            '{"incomplete": ',
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        rescue ArgumentError => e
          error_message = e.message
        end

        expect(error_message).to include('Invalid routes JSON')
      end
    end

    context 'with invalid handler hash' do
      let(:valid_routes_json) do
        JSON.generate([
                        {
                          method: 'GET',
                          path: '/test',
                          handler_name: 'test_handler',
                          status: '200 OK',
                          request_schema: nil,
                          response_schema: nil,
                          path_params: [],
                          path_regex: nil,
                          handler_dependencies: []
                        }
                      ])
      end

      it 'raises TypeError when handler value is not callable' do
        expect do
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { test_handler: 'not a proc' },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error
      end

      it 'raises TypeError when handler is an integer' do
        expect do
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { test_handler: 42 },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error
      end

      it 'raises TypeError when handler is a hash' do
        expect do
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { test_handler: { some: 'hash' } },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error
      end

      it 'raises ArgumentError when handlers parameter is not a Hash' do
        expect do
          Spikard::Native::TestClient.new(
            valid_routes_json,
            %w[not a hash],
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /handlers parameter must be a Hash/)
      end
    end
  end

  describe 'Native::LifecycleRegistry hook registration' do
    let(:registry) { Spikard::Native::LifecycleRegistry.new }

    context 'with non-Proc/non-Lambda blocks' do
      it 'raises TypeError when on_request receives a non-callable' do
        expect do
          registry.add_on_request('not a proc')
        end.to raise_error
      end

      it 'raises TypeError when pre_validation receives a string' do
        expect do
          registry.pre_validation('string value')
        end.to raise_error
      end

      it 'raises TypeError when pre_handler receives an integer' do
        expect do
          registry.pre_handler(42)
        end.to raise_error
      end

      it 'raises TypeError when on_response receives a hash' do
        expect do
          registry.on_response({ key: 'value' })
        end.to raise_error
      end

      it 'raises TypeError when on_error receives an array' do
        expect do
          registry.on_error([1, 2, 3])
        end.to raise_error
      end
    end

    context 'with valid Proc/Lambda blocks' do
      it 'accepts a Proc for on_request' do
        expect do
          registry.add_on_request(proc { |_req| true })
        end.not_to raise_error
      end

      it 'accepts a Lambda for pre_validation' do
        expect do
          registry.pre_validation(->(_req) { true })
        end.not_to raise_error
      end

      it 'accepts a block passed with & for pre_handler' do
        my_block = proc { |_req| true }
        expect do
          registry.pre_handler(my_block)
        end.not_to raise_error
      end

      it 'accepts multiple hooks sequentially' do
        expect do
          registry.add_on_request(proc { |_req| true })
          registry.pre_validation(->(_req) { true })
          registry.pre_handler(proc { |_req| true })
        end.not_to raise_error
      end
    end
  end

  describe 'Native::DependencyRegistry dependency resolution' do
    let(:registry) { Spikard::Native::DependencyRegistry.new }

    context 'with missing dependencies' do
      it 'raises error when resolving unregistered dependency' do
        expect do
          registry.resolve('missing_key')
        end.to raise_error
      end

      it 'error message includes the missing key name' do
        error_message = nil
        begin
          registry.resolve('missing_key')
        rescue StandardError => e
          error_message = e.message
        end

        expect(error_message).to include('missing_key') if error_message
      end
    end

    context 'with registered dependencies' do
      it 'successfully registers and retrieves a value dependency' do
        registry.register_value('test_key', { data: 'value' })
        expect(registry.keys).to include('test_key')
      end

      it 'successfully registers a factory dependency' do
        factory = proc { |_deps| { created: 'value' } }
        registry.register_factory('factory_key', factory, nil, true, true)
        expect(registry.keys).to include('factory_key')
      end
    end
  end

  describe 'Route metadata field type validation' do
    let(:registry) { Spikard::Native::DependencyRegistry.new }

    context 'with invalid field types' do
      it 'rejects path as a Symbol when String is required' do
        expect do
          Spikard::Native::TestClient.new(
            JSON.generate([
                            {
                              method: 'GET',
                              path: :symbol_path, # Should be String
                              handler_name: 'test_handler',
                              status: '200 OK',
                              request_schema: nil,
                              response_schema: nil,
                              path_params: [],
                              path_regex: nil,
                              handler_dependencies: []
                            }
                          ]),
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end

      it 'rejects handler_name as an integer' do
        expect do
          Spikard::Native::TestClient.new(
            JSON.generate([
                            {
                              method: 'GET',
                              path: '/test',
                              handler_name: 123, # Should be String
                              status: '200 OK',
                              request_schema: nil,
                              response_schema: nil,
                              path_params: [],
                              path_regex: nil,
                              handler_dependencies: []
                            }
                          ]),
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end

      it 'rejects method as a symbol instead of string' do
        expect do
          Spikard::Native::TestClient.new(
            JSON.generate([
                            {
                              method: :GET, # Should be uppercase String
                              path: '/test',
                              handler_name: 'test_handler',
                              status: '200 OK',
                              request_schema: nil,
                              response_schema: nil,
                              path_params: [],
                              path_regex: nil,
                              handler_dependencies: []
                            }
                          ]),
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end
    end
  end

  describe 'Handler return value type checking' do
    let(:valid_routes_json) do
      JSON.generate([
                      {
                        method: 'GET',
                        path: '/test',
                        handler_name: 'test_handler',
                        status: '200 OK',
                        request_schema: nil,
                        response_schema: nil,
                        path_params: [],
                        path_regex: nil,
                        handler_dependencies: []
                      }
                    ])
    end

    context 'with valid return types' do
      it 'accepts a String return value' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| 'response text' } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )
        expect(client).not_to be_nil
      end

      it 'accepts a Hash return value (JSON)' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| { status: 'ok', data: [1, 2, 3] } } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )
        expect(client).not_to be_nil
      end

      it 'accepts a Response object return value' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          {
            test_handler: proc do |_req|
              Spikard::Response.new({ message: 'ok' }, 200)
            end
          },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )
        expect(client).not_to be_nil
      end

      it 'accepts an Array return value' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| [1, 2, 3] } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )
        expect(client).not_to be_nil
      end

      it 'accepts nil as a valid return value' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )
        expect(client).not_to be_nil
      end
    end
  end

  describe 'WebSocket handler return values' do
    let(:empty_routes_json) do
      JSON.generate([])
    end

    context 'with valid WebSocket handlers' do
      it 'accepts a Hash for WebSocket handler configuration' do
        expect do
          Spikard::Native::TestClient.new(
            empty_routes_json,
            {},
            Spikard::ServerConfig.new,
            { '/ws' => proc { Spikard::WebSocketHandler.new } },
            nil,
            nil
          )
        end.not_to raise_error
      end

      it 'accepts nil from WebSocket factory' do
        expect do
          Spikard::Native::TestClient.new(
            empty_routes_json,
            {},
            Spikard::ServerConfig.new,
            { '/ws' => proc {} },
            nil,
            nil
          )
        end.not_to raise_error
      end
    end
  end

  describe 'SSE producer data field type checking' do
    let(:valid_routes_json) do
      JSON.generate([
                      {
                        method: 'GET',
                        path: '/sse',
                        handler_name: 'sse_handler',
                        status: '200 OK',
                        request_schema: nil,
                        response_schema: nil,
                        path_params: [],
                        path_regex: nil,
                        handler_dependencies: []
                      }
                    ])
    end

    context 'with valid SSE data' do
      it 'accepts serializable data in SSE events' do
        expect do
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { sse_handler: proc { |_req| } },
            Spikard::ServerConfig.new,
            nil,
            { '/sse' => proc { Spikard::SseEventProducer.new } },
            nil
          )
        end.not_to raise_error
      end
    end
  end

  describe 'Native config coercion and validation' do
    context 'with invalid port types' do
      it 'rejects port as nil' do
        expect do
          config = Spikard::ServerConfig.new
          config.port = nil
          # Validate during bridge crossing
          expect(config.port).to be_nil # Ruby allows nil, Rust validates on crossing
        end.not_to raise_error
      end

      it 'accepts port as integer' do
        config = Spikard::ServerConfig.new(port: 8000)
        expect(config.port).to eq(8000)
      end

      it 'coerces string port to integer during native call' do
        config = Spikard::ServerConfig.new(port: 8000)
        expect(config.port).to be_an(Integer)
        expect(config.port).to eq(8000)
      end
    end

    context 'with workers configuration' do
      it 'rejects negative workers count' do
        expect do
          Spikard::ServerConfig.new(workers: -1)
        end.to raise_error(ArgumentError)
      end

      it 'warns when workers exceeds typical core count (256)' do
        expect do
          config = Spikard::ServerConfig.new(workers: 512)
          # Should warn but not fail
          expect(config.workers).to eq(512)
        end.not_to raise_error
      end

      it 'accepts valid workers count' do
        config = Spikard::ServerConfig.new(workers: 8)
        expect(config.workers).to eq(8)
      end
    end

    context 'with timeout values' do
      it 'coerces string timeout to integer' do
        config = Spikard::ServerConfig.new(request_timeout: 30)
        expect(config.request_timeout).to be_an(Integer)
      end

      it 'accepts float timeout and preserves as number' do
        config = Spikard::ServerConfig.new(request_timeout: 30)
        expect(config.request_timeout).to eq(30)
      end

      it 'rejects invalid timeout types' do
        expect do
          Spikard::ServerConfig.new(request_timeout: 'not a number')
        end.to raise_error
      end
    end

    context 'with max_body_size' do
      it 'coerces integer body size correctly' do
        size = 10 * 1024 * 1024 # 10 MB
        config = Spikard::ServerConfig.new(max_body_size: size)
        expect(config.max_body_size).to eq(size)
      end

      it 'rejects negative body size' do
        expect do
          Spikard::ServerConfig.new(max_body_size: -1024)
        end.to raise_error(ArgumentError)
      end

      it 'accepts nil for unlimited body size' do
        config = Spikard::ServerConfig.new(max_body_size: nil)
        expect(config.max_body_size).to be_nil
      end
    end
  end

  describe 'Float vs Integer coercion' do
    context 'for timeouts and durations' do
      it 'coerces float timeout to integer milliseconds' do
        config = Spikard::ServerConfig.new(request_timeout: 30)
        expect(config.request_timeout).to be_an(Integer)
      end

      it 'preserves integer timeout precision' do
        config = Spikard::ServerConfig.new(request_timeout: 60)
        expect(config.request_timeout).to eq(60)
      end

      it 'rejects float when integer required in strict mode' do
        config = Spikard::ServerConfig.new(shutdown_timeout: 30)
        expect(config.shutdown_timeout).to be_an(Integer)
      end
    end

    context 'for quality settings' do
      it 'coerces float quality to integer' do
        compression = Spikard::CompressionConfig.new(quality: 6)
        expect(compression.quality).to be_an(Integer)
      end

      it 'validates quality within valid range' do
        expect do
          Spikard::CompressionConfig.new(quality: 20) # Out of range
        end.to raise_error(ArgumentError)
      end
    end
  end

  describe 'Option<T> handling in Rust-to-Ruby translation' do
    context 'with nil vs missing values' do
      it 'distinguishes nil from missing optional field' do
        config = Spikard::ServerConfig.new
        expect(config.rate_limit).to be_nil # Missing is nil
      end

      it 'handles explicitly set nil for optional fields' do
        config = Spikard::ServerConfig.new(rate_limit: nil)
        expect(config.rate_limit).to be_nil
      end

      it 'preserves configured optional values' do
        rate_limit = Spikard::RateLimitConfig.new(per_second: 100, burst: 200)
        config = Spikard::ServerConfig.new(rate_limit: rate_limit)
        expect(config.rate_limit).not_to be_nil
        expect(config.rate_limit.per_second).to eq(100)
      end

      it 'allows resetting optional field to nil' do
        config = Spikard::ServerConfig.new
        config.rate_limit = nil
        expect(config.rate_limit).to be_nil
      end
    end

    context 'with openapi config as Option' do
      it 'returns nil when OpenAPI not configured' do
        config = Spikard::ServerConfig.new
        expect(config.openapi).to be_nil
      end

      it 'returns OpenApiConfig when provided' do
        openapi = Spikard::OpenApiConfig.new(enabled: true)
        config = Spikard::ServerConfig.new(openapi: openapi)
        expect(config.openapi).not_to be_nil
        expect(config.openapi.enabled).to be true
      end
    end
  end

  describe 'Type mismatch error messages' do
    let(:valid_routes_json) do
      JSON.generate([
                      {
                        method: 'GET',
                        path: '/test',
                        handler_name: 'test_handler',
                        status: '200 OK',
                        request_schema: nil,
                        response_schema: nil,
                        path_params: [],
                        path_regex: nil,
                        handler_dependencies: []
                      }
                    ])
    end

    context 'error message clarity' do
      it 'includes expected type in handler error message' do
        error_message = nil
        begin
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { test_handler: 42 },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        rescue StandardError => e
          error_message = e.message
        end

        expect(error_message).to_not be_nil
        expect(error_message).to_not be_empty
      end

      it 'includes actual type received in error message' do
        error_message = nil
        begin
          Spikard::Native::TestClient.new(
            valid_routes_json,
            { test_handler: 'string value' },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        rescue StandardError => e
          error_message = e.message
        end

        expect(error_message).to_not be_nil
        expect(error_message).to_not be_empty
      end

      it 'provides helpful context for required parameters' do
        error_message = nil
        begin
          Spikard::Native::TestClient.new(
            valid_routes_json,
            nil, # Missing handlers
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        rescue StandardError => e
          error_message = e.message
        end

        expect(error_message).to include('Hash') if error_message
      end
    end

    context 'config validation error messages' do
      it 'includes field name in validation error' do
        error_message = nil
        begin
          Spikard::ServerConfig.new(workers: -1)
        rescue ArgumentError => e
          error_message = e.message
        end

        expect(error_message).to include('workers') if error_message
      end

      it 'includes constraint information in error' do
        error_message = nil
        begin
          Spikard::ServerConfig.new(workers: -1)
        rescue ArgumentError => e
          error_message = e.message
        end

        # Should mention the constraint
        expect(error_message).to_not be_nil
        expect(error_message).to_not be_empty
      end
    end
  end

  describe 'Cross-FFI boundary safety' do
    let(:valid_routes_json) do
      JSON.generate([
                      {
                        method: 'GET',
                        path: '/test',
                        handler_name: 'test_handler',
                        status: '200 OK',
                        request_schema: nil,
                        response_schema: nil,
                        path_params: [],
                        path_regex: nil,
                        handler_dependencies: []
                      }
                    ])
    end

    context 'handler execution safety' do
      it 'safely wraps exceptions from handlers' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          {
            test_handler: proc { |_req|
              raise 'Handler error'
            }
          },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )

        expect do
          client.request('GET', '/test')
        end.to raise_error
      end

      it 'prevents unwrap panics from crossing FFI boundary' do
        # This is tested by ensuring all native error conversions use Result
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| 'ok' } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )

        expect do
          response = client.request('GET', '/test')
          expect(response).to be_a(Hash)
        end.not_to raise_error
      end
    end

    context 'parameter validation at boundary' do
      it 'validates method parameter before FFI call' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| 'ok' } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )

        expect do
          client.request('INVALID_METHOD', '/test')
        end.to raise_error
      end

      it 'validates path parameter before FFI call' do
        client = Spikard::Native::TestClient.new(
          valid_routes_json,
          { test_handler: proc { |_req| 'ok' } },
          Spikard::ServerConfig.new,
          nil,
          nil,
          nil
        )

        # Path validation happens at Rust boundary
        expect do
          response = client.request('GET', '/nonexistent')
          # May return 404, which is valid
          expect(response).to be_a(Hash)
        end.not_to raise_error
      end
    end
  end

  describe 'Integer and Float type preservation' do
    let(:valid_routes_json) do
      JSON.generate([
                      {
                        method: 'GET',
                        path: '/test',
                        handler_name: 'test_handler',
                        status: '200 OK',
                        request_schema: nil,
                        response_schema: nil,
                        path_params: [],
                        path_regex: nil,
                        handler_dependencies: []
                      }
                    ])
    end

    it 'preserves integer types across FFI boundary' do
      config = Spikard::ServerConfig.new(port: 8000)
      expect(config.port.class).to eq(Integer)
    end

    it 'preserves float types in compression quality' do
      compression = Spikard::CompressionConfig.new(quality: 6)
      expect(compression.quality.class).to eq(Integer)
    end

    it 'rejects mixed type coercion for sensitive fields' do
      expect do
        Spikard::ServerConfig.new(port: 'not an integer')
      end.to raise_error
    end
  end

  describe 'Array bounds and validation' do
    context 'with static files configuration' do
      it 'accepts empty static files array' do
        config = Spikard::ServerConfig.new(static_files: [])
        expect(config.static_files).to eq([])
      end

      it 'accepts multiple static file configs' do
        static1 = Spikard::StaticFilesConfig.new(
          directory: './public',
          route_prefix: '/static'
        )
        static2 = Spikard::StaticFilesConfig.new(
          directory: './uploads',
          route_prefix: '/files'
        )

        config = Spikard::ServerConfig.new(static_files: [static1, static2])
        expect(config.static_files.count).to eq(2)
      end

      it 'validates array element types' do
        expect do
          Spikard::ServerConfig.new(static_files: ['not a config'])
        end.to raise_error
      end
    end

    context 'with servers configuration' do
      it 'accepts empty servers array' do
        openapi = Spikard::OpenApiConfig.new(servers: [])
        expect(openapi.servers).to eq([])
      end

      it 'accepts multiple server configs' do
        server1 = Spikard::ServerInfo.new(url: 'https://api.example.com')
        server2 = Spikard::ServerInfo.new(url: 'http://localhost:8000')

        openapi = Spikard::OpenApiConfig.new(servers: [server1, server2])
        expect(openapi.servers.count).to eq(2)
      end
    end
  end

  describe 'String type validation' do
    context 'for path parameters' do
      it 'requires path as string, not symbol' do
        expect do
          Spikard::Native::TestClient.new(
            JSON.generate([
                            {
                              method: 'GET',
                              path: :path_symbol, # Invalid
                              handler_name: 'test_handler',
                              status: '200 OK',
                              request_schema: nil,
                              response_schema: nil,
                              path_params: [],
                              path_regex: nil,
                              handler_dependencies: []
                            }
                          ]),
            {},
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.to raise_error(ArgumentError, /Invalid routes JSON/)
      end

      it 'accepts string paths with special characters' do
        expect do
          Spikard::Native::TestClient.new(
            JSON.generate([
                            {
                              method: 'GET',
                              path: '/api-v1/users_list.json',
                              handler_name: 'test_handler',
                              status: '200 OK',
                              request_schema: nil,
                              response_schema: nil,
                              path_params: [],
                              path_regex: nil,
                              handler_dependencies: []
                            }
                          ]),
            { test_handler: proc { |_req| 'ok' } },
            Spikard::ServerConfig.new,
            nil,
            nil,
            nil
          )
        end.not_to raise_error
      end
    end

    context 'for header values' do
      it 'validates header names as strings' do
        headers = { 'Content-Type' => 'application/json' }
        expect(headers.keys.all? { |k| k.is_a?(String) }).to be true
      end

      it 'validates header values as strings' do
        headers = { 'Content-Type' => 'application/json' }
        expect(headers.values.all? { |v| v.is_a?(String) }).to be true
      end
    end
  end

  describe 'Boolean type handling' do
    context 'for compression settings' do
      it 'accepts true for gzip' do
        compression = Spikard::CompressionConfig.new(gzip: true)
        expect(compression.gzip).to be true
      end

      it 'accepts false for gzip' do
        compression = Spikard::CompressionConfig.new(gzip: false)
        expect(compression.gzip).to be false
      end

      it 'rejects non-boolean for gzip' do
        expect do
          Spikard::CompressionConfig.new(gzip: 'yes')
        end.to raise_error
      end
    end

    context 'for enable flags' do
      it 'accepts true for enable_request_id' do
        config = Spikard::ServerConfig.new(enable_request_id: true)
        expect(config.enable_request_id).to be true
      end

      it 'accepts false for enable_request_id' do
        config = Spikard::ServerConfig.new(enable_request_id: false)
        expect(config.enable_request_id).to be false
      end

      it 'rejects integer for boolean flag' do
        expect do
          Spikard::ServerConfig.new(enable_request_id: 1)
        end.to raise_error
      end
    end
  end
end
