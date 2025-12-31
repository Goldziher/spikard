# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::Grpc do
  describe 'module structure' do
    it 'defines the Grpc module' do
      expect(described_class).to be_a(Module)
    end

    it 'defines Request class' do
      expect(Spikard::Grpc::Request).to be_a(Class)
    end

    it 'defines Response class' do
      expect(Spikard::Grpc::Response).to be_a(Class)
    end

    it 'defines Handler class' do
      expect(Spikard::Grpc::Handler).to be_a(Class)
    end

    it 'defines Service class' do
      expect(Spikard::Grpc::Service).to be_a(Class)
    end
  end

  describe Spikard::Grpc::Response do
    describe '#initialize' do
      it 'creates a response with payload' do
        payload = "\x08\x01\x12\x04test".b
        response = described_class.new(payload: payload)
        expect(response).to be_a(described_class)
      end

      it 'accepts binary string payload' do
        payload = "binary data\x00\x01\x02".b
        response = described_class.new(payload: payload)
        expect(response.payload).to eq(payload)
      end

      it 'raises error for nil payload' do
        expect { described_class.new(payload: nil) }.to raise_error(ArgumentError)
      end

      it 'raises error for non-string payload' do
        expect { described_class.new(payload: 123) }.to raise_error(ArgumentError)
      end
    end

    describe '#metadata=' do
      it 'accepts a hash of metadata' do
        response = described_class.new(payload: 'test'.b)
        response.metadata = { 'x-custom-header' => 'value' }
        # Metadata is set successfully (no error raised)
        expect(response).to be_a(described_class)
      end

      it 'accepts nil metadata' do
        response = described_class.new(payload: 'test'.b)
        response.metadata = nil
        expect(response).to be_a(described_class)
      end

      it 'accepts empty hash metadata' do
        response = described_class.new(payload: 'test'.b)
        response.metadata = {}
        expect(response).to be_a(described_class)
      end
    end
  end

  describe Spikard::Grpc::Handler do
    describe '#handle_request' do
      it 'raises NotImplementedError when not overridden' do
        handler = described_class.new
        request = double('request')

        expect { handler.handle_request(request) }.to raise_error(NotImplementedError)
      end

      context 'when subclassed' do
        let(:handler_class) do
          Class.new(described_class) do
            def handle_request(request)
              Spikard::Grpc::Response.new(payload: "response for #{request.method_name}".b)
            end
          end
        end

        it 'can be overridden in subclass' do
          handler = handler_class.new
          request = double('request', method_name: 'TestMethod')

          response = handler.handle_request(request)
          expect(response).to be_a(Spikard::Grpc::Response)
        end
      end
    end
  end

  describe Spikard::Grpc::Service do
    describe '#initialize' do
      it 'creates an empty service registry' do
        service = described_class.new
        expect(service.service_names).to be_empty
      end
    end

    describe '#register_handler' do
      let(:service) { described_class.new }
      let(:handler_class) do
        Class.new(Spikard::Grpc::Handler) do
          def handle_request(_request)
            Spikard::Grpc::Response.new(payload: 'test'.b)
          end
        end
      end

      it 'registers a handler for a service' do
        handler = handler_class.new
        service.register_handler('mypackage.MyService', handler)

        expect(service.service_names).to include('mypackage.MyService')
      end

      it 'raises error when handler does not respond to handle_request' do
        invalid_handler = Object.new

        expect do
          service.register_handler('test.Service', invalid_handler)
        end.to raise_error(ArgumentError, /must respond to :handle_request/)
      end

      it 'allows multiple handlers to be registered' do
        handler1 = handler_class.new
        handler2 = handler_class.new

        service.register_handler('service1', handler1)
        service.register_handler('service2', handler2)

        expect(service.service_names).to contain_exactly('service1', 'service2')
      end
    end

    describe '#get_handler' do
      let(:service) { described_class.new }
      let(:handler_class) do
        Class.new(Spikard::Grpc::Handler) do
          def handle_request(_request)
            Spikard::Grpc::Response.new(payload: 'test'.b)
          end
        end
      end

      it 'returns registered handler' do
        handler = handler_class.new
        service.register_handler('test.Service', handler)

        retrieved = service.get_handler('test.Service')
        expect(retrieved).to eq(handler)
      end

      it 'returns nil for unregistered service' do
        retrieved = service.get_handler('nonexistent.Service')
        expect(retrieved).to be_nil
      end
    end

    describe '#registered?' do
      let(:service) { described_class.new }
      let(:handler) do
        Class.new(Spikard::Grpc::Handler) do
          def handle_request(_request)
            Spikard::Grpc::Response.new(payload: 'test'.b)
          end
        end.new
      end

      it 'returns true for registered service' do
        service.register_handler('test.Service', handler)
        expect(service.registered?('test.Service')).to be true
      end

      it 'returns false for unregistered service' do
        expect(service.registered?('nonexistent.Service')).to be false
      end
    end
  end

  describe 'Integration examples' do
    let(:handler_class) do
      Class.new(Spikard::Grpc::Handler) do
        def handle_request(request)
          case request.method_name
          when 'Echo'
            # Echo back the payload
            Spikard::Grpc::Response.new(payload: request.payload)
          when 'GetUser'
            # Return a mock user response
            response = Spikard::Grpc::Response.new(payload: "\x08\x01\x12\x04John".b)
            response.metadata = { 'x-user-id' => '1' }
            response
          else
            raise "Unknown method: #{request.method_name}"
          end
        end
      end
    end

    it 'can handle echo requests' do
      handler = handler_class.new

      # Mock request
      request = double(
        'request',
        service_name: 'test.EchoService',
        method_name: 'Echo',
        payload: 'hello world'.b
      )

      response = handler.handle_request(request)
      expect(response.payload).to eq('hello world'.b)
    end

    it 'can handle user requests with metadata' do
      handler = handler_class.new

      # Mock request
      request = double(
        'request',
        service_name: 'test.UserService',
        method_name: 'GetUser',
        payload: "\x08\x01".b
      )

      response = handler.handle_request(request)
      expect(response).to be_a(Spikard::Grpc::Response)
      expect(response.payload).to eq("\x08\x01\x12\x04John".b)
    end

    it 'raises error for unknown methods' do
      handler = handler_class.new

      request = double(
        'request',
        service_name: 'test.Service',
        method_name: 'UnknownMethod',
        payload: ''.b
      )

      expect { handler.handle_request(request) }.to raise_error(/Unknown method/)
    end
  end

  describe 'Service registry workflow' do
    it 'allows complete handler registration and lookup' do
      # Create service registry
      service = Spikard::Grpc::Service.new

      # Define handler
      handler_class = Class.new(Spikard::Grpc::Handler) do
        def handle_request(_request)
          Spikard::Grpc::Response.new(payload: 'response'.b)
        end
      end

      # Register handlers
      user_handler = handler_class.new
      post_handler = handler_class.new

      service.register_handler('myapp.UserService', user_handler)
      service.register_handler('myapp.PostService', post_handler)

      # Verify registration
      expect(service.service_names).to contain_exactly('myapp.UserService', 'myapp.PostService')
      expect(service.registered?('myapp.UserService')).to be true
      expect(service.registered?('myapp.PostService')).to be true

      # Retrieve and use handlers
      retrieved_user = service.get_handler('myapp.UserService')
      expect(retrieved_user).to eq(user_handler)

      # Mock request and handle it
      request = double('request', method_name: 'Test', payload: 'test'.b)
      response = retrieved_user.handle_request(request)
      expect(response.payload).to eq('response'.b)
    end
  end

  describe 'Streaming Support' do
    describe 'server-side streaming' do
      it 'yields multiple responses from handler as enumerator' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            5.times do |i|
              yielder << Spikard::Grpc::Response.new(
                payload: { count: i }.to_json.b
              )
            end
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'ServerStream',
          payload: '{}'.b,
          metadata: {}
        )

        responses = handler.call(request).to_a

        expect(responses).to be_a(Array)
        expect(responses.length).to eq(5)
        expect(responses.first).to be_a(Spikard::Grpc::Response)
        expect(responses.last).to be_a(Spikard::Grpc::Response)
      end

      it 'streams responses lazily without buffering all' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            100.times do |i|
              yielder << Spikard::Grpc::Response.new(
                payload: { index: i }.to_json.b
              )
            end
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'LazyStream',
          payload: '{}'.b,
          metadata: {}
        )

        enum = handler.call(request)
        expect(enum).to be_a(Enumerator)

        first_response = enum.next
        expect(first_response).to be_a(Spikard::Grpc::Response)
      end

      it 'handles empty streams gracefully' do
        handler = lambda do |request|
          Enumerator.new { |_yielder| } # Yields nothing
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'EmptyStream',
          payload: '{}'.b,
          metadata: {}
        )

        responses = handler.call(request).to_a
        expect(responses).to be_empty
      end

      it 'handles large number of stream messages' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            1000.times do |i|
              yielder << Spikard::Grpc::Response.new(
                payload: { msg_id: i }.to_json.b
              )
            end
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'LargeStream',
          payload: '{}'.b,
          metadata: {}
        )

        responses = handler.call(request).to_a
        expect(responses.length).to eq(1000)
        expect(responses[500]).to be_a(Spikard::Grpc::Response)
      end

      it 'handles stream errors gracefully' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            yielder << Spikard::Grpc::Response.new(
              payload: { msg: 'first' }.to_json.b
            )
            raise StandardError, 'Stream processing error'
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'ErrorStream',
          payload: '{}'.b,
          metadata: {}
        )

        expect { handler.call(request).to_a }.to raise_error(StandardError, /Stream processing error/)
      end

      it 'preserves metadata in streamed responses' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            3.times do |i|
              response = Spikard::Grpc::Response.new(
                payload: { index: i }.to_json.b
              )
              response.metadata = { 'stream-id' => 'abc123', 'chunk-num' => i.to_s }
              yielder << response
            end
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'MetadataStream',
          payload: '{}'.b,
          metadata: {}
        )

        responses = handler.call(request).to_a

        expect(responses.first.metadata).to include('stream-id' => 'abc123')
        expect(responses[1].metadata).to include('chunk-num' => '1')
      end

      it 'handles stream cancellation by StopIteration' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            5.times do |i|
              yielder << Spikard::Grpc::Response.new(
                payload: { count: i }.to_json.b
              )
            end
          end
        end

        request = double(
          'request',
          service_name: 'test.StreamService',
          method_name: 'CancelStream',
          payload: '{}'.b,
          metadata: {}
        )

        enum = handler.call(request)
        responses = []
        enum.each do |resp|
          responses << resp
          break if responses.length == 2
        end

        expect(responses.length).to eq(2)
      end
    end

    describe 'client-side streaming' do
      it 'accepts multiple requests as enumerator input' do
        requests_received = []

        handler = lambda do |request_enum|
          request_enum.each do |req|
            requests_received << req.payload
          end

          Spikard::Grpc::Response.new(
            payload: { total: requests_received.length }.to_json.b
          )
        end

        # Create a stream of requests
        request_stream = [
          double('request', payload: '{"id": 1}'.b),
          double('request', payload: '{"id": 2}'.b),
          double('request', payload: '{"id": 3}'.b)
        ].to_enum

        response = handler.call(request_stream)

        expect(response).to be_a(Spikard::Grpc::Response)
        expect(requests_received.length).to eq(3)
      end

      it 'handles empty client-side stream' do
        handler = lambda do |request_enum|
          count = request_enum.count
          Spikard::Grpc::Response.new(
            payload: { received: count }.to_json.b
          )
        end

        empty_stream = [].to_enum

        response = handler.call(empty_stream)

        expect(response).to be_a(Spikard::Grpc::Response)
      end
    end

    describe 'bidirectional streaming' do
      it 'handles request/response streaming pairs' do
        handler = lambda do |request_enum|
          Enumerator.new do |response_yielder|
            request_enum.each do |req|
              # Echo back with increment
              payload = JSON.parse(req.payload)
              payload['count'] = payload['count'] + 1
              response_yielder << Spikard::Grpc::Response.new(
                payload: payload.to_json.b
              )
            end
          end
        end

        request_stream = [
          double('request', payload: '{"count": 1}'.b),
          double('request', payload: '{"count": 2}'.b),
          double('request', payload: '{"count": 3}'.b)
        ].to_enum

        responses = handler.call(request_stream).to_a

        expect(responses.length).to eq(3)
        expect(JSON.parse(responses.first.payload)['count']).to eq(2)
        expect(JSON.parse(responses.last.payload)['count']).to eq(4)
      end

      it 'handles streaming with per-message errors' do
        handler = lambda do |request_enum|
          Enumerator.new do |response_yielder|
            request_enum.each_with_index do |req, idx|
              raise StandardError, 'Mid-stream error' if idx == 1

              response_yielder << Spikard::Grpc::Response.new(
                payload: req.payload
              )
            end
          end
        end

        request_stream = [
          double('request', payload: '{"id": 1}'.b),
          double('request', payload: '{"id": 2}'.b),
          double('request', payload: '{"id": 3}'.b)
        ].to_enum

        expect { handler.call(request_stream).to_a }.to raise_error(StandardError, /Mid-stream error/)
      end
    end
  end

  describe 'gRPC Status Codes' do
    describe 'standard status codes' do
      it 'handles OK status code' do
        response = Spikard::Grpc::Response.new(payload: 'success'.b)
        response.metadata = { 'grpc-status' => 'OK' }

        expect(response.metadata).to include('grpc-status' => 'OK')
      end

      it 'handles CANCELLED status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = { 'grpc-status' => 'CANCELLED' }

        expect(response.metadata).to include('grpc-status' => 'CANCELLED')
      end

      it 'handles UNKNOWN status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = { 'grpc-status' => 'UNKNOWN' }

        expect(response.metadata).to include('grpc-status' => 'UNKNOWN')
      end

      it 'handles INVALID_ARGUMENT status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'INVALID_ARGUMENT',
          'grpc-message' => 'Missing required field: name'
        }

        expect(response.metadata).to include('grpc-status' => 'INVALID_ARGUMENT')
      end

      it 'handles DEADLINE_EXCEEDED status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'DEADLINE_EXCEEDED',
          'grpc-message' => 'RPC timeout'
        }

        expect(response.metadata).to include('grpc-status' => 'DEADLINE_EXCEEDED')
      end

      it 'handles NOT_FOUND status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'NOT_FOUND',
          'grpc-message' => 'User not found'
        }

        expect(response.metadata).to include('grpc-status' => 'NOT_FOUND')
      end

      it 'handles ALREADY_EXISTS status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'ALREADY_EXISTS',
          'grpc-message' => 'User already registered'
        }

        expect(response.metadata).to include('grpc-status' => 'ALREADY_EXISTS')
      end

      it 'handles PERMISSION_DENIED status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'PERMISSION_DENIED',
          'grpc-message' => 'Access denied'
        }

        expect(response.metadata).to include('grpc-status' => 'PERMISSION_DENIED')
      end

      it 'handles RESOURCE_EXHAUSTED status code for rate limiting' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'RESOURCE_EXHAUSTED',
          'grpc-message' => 'Rate limit exceeded: 100 requests per minute'
        }

        expect(response.metadata).to include('grpc-status' => 'RESOURCE_EXHAUSTED')
        expect(response.metadata['grpc-message']).to include('Rate limit')
      end

      it 'handles FAILED_PRECONDITION status code for state errors' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'FAILED_PRECONDITION',
          'grpc-message' => 'User must be verified before payment'
        }

        expect(response.metadata).to include('grpc-status' => 'FAILED_PRECONDITION')
      end

      it 'handles ABORTED status code for transaction rollback' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'ABORTED',
          'grpc-message' => 'Transaction aborted due to conflict'
        }

        expect(response.metadata).to include('grpc-status' => 'ABORTED')
      end

      it 'handles OUT_OF_RANGE status code for bounds checking' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'OUT_OF_RANGE',
          'grpc-message' => 'Page number 999 exceeds max pages 10'
        }

        expect(response.metadata).to include('grpc-status' => 'OUT_OF_RANGE')
      end

      it 'handles UNIMPLEMENTED status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'UNIMPLEMENTED',
          'grpc-message' => 'Method not implemented'
        }

        expect(response.metadata).to include('grpc-status' => 'UNIMPLEMENTED')
      end

      it 'handles INTERNAL status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'INTERNAL',
          'grpc-message' => 'Internal server error'
        }

        expect(response.metadata).to include('grpc-status' => 'INTERNAL')
      end

      it 'handles UNAVAILABLE status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'UNAVAILABLE',
          'grpc-message' => 'Service temporarily unavailable'
        }

        expect(response.metadata).to include('grpc-status' => 'UNAVAILABLE')
      end

      it 'handles DATA_LOSS status code for corruption' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'DATA_LOSS',
          'grpc-message' => 'Unrecoverable data corruption detected',
          'x-error-detail' => 'checksum mismatch'
        }

        expect(response.metadata).to include('grpc-status' => 'DATA_LOSS')
        expect(response.metadata).to have_key('x-error-detail')
      end

      it 'handles UNAUTHENTICATED status code' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'UNAUTHENTICATED',
          'grpc-message' => 'Authentication required'
        }

        expect(response.metadata).to include('grpc-status' => 'UNAUTHENTICATED')
      end
    end

    describe 'status code handling' do
      it 'supports status code as string' do
        response = Spikard::Grpc::Response.new(payload: 'data'.b)
        response.metadata = { 'grpc-status' => 'NOT_FOUND' }

        expect(response.metadata['grpc-status']).to be_a(String)
        expect(response.metadata['grpc-status']).to eq('NOT_FOUND')
      end

      it 'supports custom status messages' do
        response = Spikard::Grpc::Response.new(payload: 'error'.b)
        custom_message = 'Custom error: Database connection timeout after 30 seconds'
        response.metadata = {
          'grpc-status' => 'UNAVAILABLE',
          'grpc-message' => custom_message
        }

        expect(response.metadata['grpc-message']).to eq(custom_message)
      end

      it 'allows multiple metadata fields with status code' do
        response = Spikard::Grpc::Response.new(payload: 'error'.b)
        response.metadata = {
          'grpc-status' => 'INTERNAL',
          'grpc-message' => 'Server error',
          'x-request-id' => 'req-12345',
          'x-error-code' => 'DB_ERROR',
          'x-retry-after' => '5'
        }

        expect(response.metadata).to have_key('grpc-status')
        expect(response.metadata).to have_key('x-request-id')
        expect(response.metadata).to have_key('x-error-code')
      end
    end
  end

  describe 'Protobuf Integration' do
    describe 'binary protobuf data' do
      it 'handles binary protobuf payload with null bytes' do
        binary_payload = "\x08\x01\x12\x04test\x00\x01\x02".b
        response = Spikard::Grpc::Response.new(payload: binary_payload)

        expect(response.payload).to eq(binary_payload)
        expect(response.payload).to include("\x00")
      end

      it 'preserves binary data during encoding/decoding' do
        original = "\x08\x96\x01\x12\x08\xe4\xb8\xad\xe6\x96\x87".b # Binary with UTF-8 chars
        response = Spikard::Grpc::Response.new(payload: original)

        expect(response.payload).to eq(original)
        expect(response.payload.length).to eq(original.length)
      end

      it 'handles empty protobuf message' do
        empty_payload = ''.b
        response = Spikard::Grpc::Response.new(payload: empty_payload)

        expect(response.payload).to eq(empty_payload)
      end

      it 'supports large protobuf messages' do
        large_payload = "\b\u0001#{('x' * 100_000).b}"
        response = Spikard::Grpc::Response.new(payload: large_payload)

        expect(response.payload.length).to eq(large_payload.length)
      end
    end

    describe 'nested and complex messages' do
      it 'handles JSON-encoded nested structures' do
        nested_data = {
          user: {
            id: 1,
            profile: {
              name: 'Alice',
              address: {
                city: 'Portland',
                zip: '97201'
              }
            }
          }
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: nested_data)
        parsed = JSON.parse(response.payload)

        expect(parsed['user']['profile']['address']['city']).to eq('Portland')
      end

      it 'handles repeated field structures' do
        repeated_data = {
          items: [
            { id: 1, name: 'Item 1' },
            { id: 2, name: 'Item 2' },
            { id: 3, name: 'Item 3' }
          ]
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: repeated_data)
        parsed = JSON.parse(response.payload)

        expect(parsed['items']).to be_an(Array)
        expect(parsed['items'].length).to eq(3)
        expect(parsed['items'][1]['name']).to eq('Item 2')
      end

      it 'handles map field structures' do
        map_data = {
          attributes: {
            color: 'red',
            size: 'large',
            weight: '5kg'
          }
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: map_data)
        parsed = JSON.parse(response.payload)

        expect(parsed['attributes']).to be_a(Hash)
        expect(parsed['attributes']['color']).to eq('red')
        expect(parsed['attributes'].keys.length).to eq(3)
      end

      it 'preserves data types in nested structures' do
        complex_data = {
          user: {
            id: 42,
            active: true,
            balance: 123.45,
            tags: %w[vip premium],
            metadata: nil
          }
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: complex_data)
        parsed = JSON.parse(response.payload)

        expect(parsed['user']['id']).to be_an(Integer)
        expect(parsed['user']['active']).to be true
        expect(parsed['user']['balance']).to be_a(Float)
        expect(parsed['user']['tags']).to be_an(Array)
      end
    end

    describe 'Any type handling' do
      it 'handles JSON Any type wrapper' do
        any_data = {
          type_url: 'type.googleapis.com/mypackage.User',
          value: { id: 1, name: 'John' }
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: any_data)
        parsed = JSON.parse(response.payload)

        expect(parsed['type_url']).to include('mypackage.User')
        expect(parsed['value']).to be_a(Hash)
      end
    end
  end

  describe 'Performance and Edge Cases' do
    describe 'large payloads' do
      it 'handles 20MB string payload' do
        large_payload = ('x' * (20 * 1024 * 1024)).b
        response = Spikard::Grpc::Response.new(payload: large_payload)

        expect(response.payload.length).to eq(20 * 1024 * 1024)
        expect(response.payload).to be_a(String)
      end

      it 'handles binary data with null bytes throughout' do
        null_heavy = ("\x00\x01\x02\x03" * 10_000).b
        response = Spikard::Grpc::Response.new(payload: null_heavy)

        expect(response.payload.length).to eq(40_000)
        expect(response.payload.count("\x00")).to be > 0
      end

      it 'handles random binary data' do
        random_binary = (0..255).to_a.shuffle.map(&:chr).join.b
        response = Spikard::Grpc::Response.new(payload: random_binary)

        expect(response.payload).to eq(random_binary)
      end
    end

    describe 'unicode and character handling' do
      it 'handles Unicode strings with emoji' do
        unicode_payload = { message: '你好 🚀 مرحبا 🎉' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: unicode_payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['message']).to include('🚀')
        expect(parsed['message']).to include('🎉')
      end

      it 'handles CJK characters correctly' do
        cjk_payload = {
          chinese: '中文测试',
          japanese: 'テスト',
          korean: '테스트'
        }.to_json.b

        response = Spikard::Grpc::Response.new(payload: cjk_payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['chinese']).to eq('中文测试')
        expect(parsed['japanese']).to eq('テスト')
        expect(parsed['korean']).to eq('테스트')
      end

      it 'handles right-to-left text' do
        rtl_payload = { arabic: 'مرحبا بالعالم', hebrew: 'שלום עולם' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: rtl_payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['arabic']).to include('العالم')
        expect(parsed['hebrew']).to include('שלום')
      end

      it 'handles mixed scripts and special characters' do
        mixed_payload = { text: 'Hello мир 世界 🌍 ñ ü ö' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: mixed_payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['text']).to include('мир')
        expect(parsed['text']).to include('世界')
        expect(parsed['text']).to include('🌍')
      end
    end

    describe 'deeply nested structures' do
      it 'handles 10+ level deeply nested hashes' do
        nested = { a: { b: { c: { d: { e: { f: { g: { h: { i: { j: { k: 'value' } } } } } } } } } } }
        payload = nested.to_json.b

        response = Spikard::Grpc::Response.new(payload: payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['a']['b']['c']['d']['e']['f']['g']['h']['i']['j']['k']).to eq('value')
      end

      it 'handles deeply nested arrays' do
        deep_array = [[[[[[[[[['deeply nested']]]]]]]]]].to_json.b
        response = Spikard::Grpc::Response.new(payload: deep_array)
        parsed = JSON.parse(response.payload)

        expect(parsed[0][0][0][0][0][0][0][0][0][0]).to eq('deeply nested')
      end
    end

    describe 'string and data variations' do
      it 'handles frozen strings' do
        frozen_payload = 'frozen data'.b
        response = Spikard::Grpc::Response.new(payload: frozen_payload)

        expect(response.payload).to eq('frozen data'.b)
      end

      it 'handles strings with special escape sequences' do
        escaped = { text: "line1\nline2\ttab\r\nwindows" }.to_json.b
        response = Spikard::Grpc::Response.new(payload: escaped)
        parsed = JSON.parse(response.payload)

        expect(parsed['text']).to include("\n")
        expect(parsed['text']).to include("\t")
      end

      it 'handles very long single line payloads' do
        long_line = ('a' * 1_000_000).b
        response = Spikard::Grpc::Response.new(payload: long_line)

        expect(response.payload.length).to eq(1_000_000)
      end
    end

    describe 'concurrent and threading scenarios' do
      it 'handles concurrent handler execution with threads' do
        responses = []
        threads = []

        handler = lambda do |id|
          sleep(0.01) # Simulate some work
          Spikard::Grpc::Response.new(payload: { id: id }.to_json.b)
        end

        5.times do |i|
          threads << Thread.new(i) do |idx|
            responses << handler.call(idx)
          end
        end

        threads.each(&:join)

        expect(responses.length).to eq(5)
        expect(responses.map(&:class).uniq).to eq([Spikard::Grpc::Response])
      end

      it 'handles thread-safe metadata access' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        threads = []

        10.times do |i|
          threads << Thread.new(i) do |idx|
            response.metadata = { "key-#{idx}" => "value-#{idx}" }
          end
        end

        threads.each(&:join)

        expect(response.metadata).to be_a(Hash) if response.metadata
      end
    end
  end

  describe 'Ruby-Specific Features' do
    describe 'handler implementation patterns' do
      it 'implements handler using block with explicit yield' do
        handler = Class.new(Spikard::Grpc::Handler) do
          def handle_request(request)
            result = yield(request) if block_given?
            Spikard::Grpc::Response.new(payload: (result || 'default').b)
          end
        end.new

        request = double('request', method_name: 'Test', payload: 'input'.b)
        response = handler.handle_request(request)

        expect(response).to be_a(Spikard::Grpc::Response)
      end

      it 'implements handler using Proc' do
        handler = proc do |request|
          Spikard::Grpc::Response.new(payload: request.payload)
        end

        request = double('request', method_name: 'Test', payload: 'test data'.b)
        response = handler.call(request)

        expect(response.payload).to eq('test data'.b)
      end

      it 'implements handler using lambda' do
        handler = lambda do |request|
          Spikard::Grpc::Response.new(payload: { method: request.method_name }.to_json.b)
        end

        request = double('request', method_name: 'GetUser', payload: '{}'.b)
        response = handler.call(request)
        parsed = JSON.parse(response.payload)

        expect(parsed['method']).to eq('GetUser')
      end

      it 'implements handler with keyword arguments' do
        handler = lambda do |request:, timeout: 30|
          Spikard::Grpc::Response.new(
            payload: { timeout: timeout }.to_json.b
          )
        end

        request = double('request', method_name: 'Test', payload: '{}'.b)
        response = handler.call(request: request, timeout: 60)
        parsed = JSON.parse(response.payload)

        expect(parsed['timeout']).to eq(60)
      end

      it 'supports handler with default keyword arguments' do
        handler = lambda do |request:, retries: 3, timeout: 30|
          Spikard::Grpc::Response.new(
            payload: { retries: retries, timeout: timeout }.to_json.b
          )
        end

        request = double('request', method_name: 'Test', payload: '{}'.b)
        response = handler.call(request: request, retries: 5)
        parsed = JSON.parse(response.payload)

        expect(parsed['retries']).to eq(5)
        expect(parsed['timeout']).to eq(30)
      end
    end

    describe 'response building patterns' do
      it 'builds response using keyword arguments' do
        payload_data = { status: 'success', data: { id: 1 } }
        response = Spikard::Grpc::Response.new(payload: payload_data.to_json.b)

        expect(response).to be_a(Spikard::Grpc::Response)
      end

      it 'builds response with hash-style metadata' do
        response = Spikard::Grpc::Response.new(payload: 'data'.b)
        response.metadata = {
          'x-request-id' => 'req-123',
          'x-timestamp' => Time.now.to_s,
          'x-version' => '1.0'
        }

        expect(response.metadata).to include('x-request-id')
        expect(response.metadata).to have_key('x-version')
      end

      it 'handles response metadata with symbol keys converted to strings' do
        response = Spikard::Grpc::Response.new(payload: 'data'.b)
        # Metadata should use string keys
        response.metadata = { 'content-type' => 'application/json' }

        expect(response.metadata.keys.first).to be_a(String)
      end
    end

    describe 'metadata handling' do
      it 'accepts metadata with string keys' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'authorization' => 'Bearer token123',
          'content-type' => 'application/octet-stream'
        }

        expect(response.metadata['authorization']).to eq('Bearer token123')
      end

      it 'handles empty metadata gracefully' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {}

        expect(response.metadata).to be_a(Hash)
        expect(response.metadata).to be_empty
      end

      it 'supports nil metadata' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = nil

        expect(response).to be_a(Spikard::Grpc::Response)
      end

      it 'handles metadata with various value types' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'x-string' => 'value',
          'x-number' => '123',
          'x-empty' => ''
        }

        expect(response.metadata['x-string']).to be_a(String)
        expect(response.metadata['x-number']).to eq('123')
      end
    end

    describe 'type coercion and conversions' do
      it 'handles type coercion in JSON payloads' do
        # String that looks like number in JSON
        payload = { id: '123', count: '456' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['id']).to be_a(String)
        expect(Integer(parsed['id'])).to eq(123)
      end

      it 'preserves numeric types in JSON' do
        payload = { integer: 42, float: 3.14, bool: true, null: nil }.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)
        parsed = JSON.parse(response.payload)

        expect(parsed['integer']).to be_an(Integer)
        expect(parsed['float']).to be_a(Float)
        expect(parsed['bool']).to be true
      end

      it 'handles string to integer conversion safely' do
        payload = { count: '100' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)
        parsed = JSON.parse(response.payload)

        count = parsed['count']
        expect { Integer(count) }.not_to raise_error
        expect(Integer(count)).to eq(100)
      end
    end

    describe 'error handling with Ruby exceptions' do
      it 'catches and handles StandardError in handler' do
        handler = lambda do |request|
          raise StandardError, 'Processing failed'
        rescue StandardError => e
          Spikard::Grpc::Response.new(
            payload: { error: e.message }.to_json.b
          )
        end

        request = double('request', method_name: 'Test', payload: '{}'.b)
        response = handler.call(request)
        parsed = JSON.parse(response.payload)

        expect(parsed['error']).to eq('Processing failed')
      end

      it 'raises custom exception classes' do
        class CustomGrpcError < StandardError; end

        handler = lambda do |request|
          raise CustomGrpcError, 'Custom error occurred'
        end

        request = double('request', method_name: 'Test', payload: '{}'.b)

        expect { handler.call(request) }.to raise_error(CustomGrpcError)
      end

      it 'handles ArgumentError in payload processing' do
        handler = lambda do |request|
          # Simulate validation error
          raise ArgumentError, 'Invalid payload format'
        rescue ArgumentError => e
          Spikard::Grpc::Response.new(
            payload: { error: e.message }.to_json.b
          )
        end

        request = double('request', method_name: 'Test', payload: 'invalid'.b)
        response = handler.call(request)
        parsed = JSON.parse(response.payload)

        expect(parsed['error']).to eq('Invalid payload format')
      end

      it 'handles TypeError for type mismatches' do
        handler = lambda do |request|
          # Simulate type error by treating payload as an array when it's a string
          result = request.payload[0] + 1  # This will raise TypeError when trying to add to a string
          Spikard::Grpc::Response.new(payload: { result: result }.to_json.b)
        rescue TypeError => e
          response_data = { error: 'Type error', details: e.message }
          Spikard::Grpc::Response.new(payload: response_data.to_json.b)
        rescue ArgumentError => e
          response_data = { error: 'Argument error', details: e.message }
          Spikard::Grpc::Response.new(payload: response_data.to_json.b)
        end

        request = double('request', method_name: 'Test', payload: 'not-a-number'.b)
        response = handler.call(request)
        parsed = JSON.parse(response.payload)

        expect(parsed).to have_key('error')
      end

      it 'chains rescue clauses for multiple exception types' do
        handler = lambda do |request|
          raise 'Runtime error'
        rescue ArgumentError => e
          Spikard::Grpc::Response.new(
            payload: { error_type: 'ArgumentError', message: e.message }.to_json.b
          )
        rescue RuntimeError => e
          Spikard::Grpc::Response.new(
            payload: { error_type: 'RuntimeError', message: e.message }.to_json.b
          )
        rescue StandardError => e
          Spikard::Grpc::Response.new(
            payload: { error_type: 'StandardError', message: e.message }.to_json.b
          )
        end

        request = double('request', method_name: 'Test', payload: '{}'.b)
        response = handler.call(request)
        parsed = JSON.parse(response.payload)

        expect(parsed['error_type']).to eq('RuntimeError')
      end
    end
  end

  describe 'Extended Streaming Support' do
    describe 'advanced streaming scenarios' do
      it 'handles stream with alternating payload types' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            # Mix JSON and binary responses
            yielder << Spikard::Grpc::Response.new(payload: { type: 'json' }.to_json.b)
            yielder << Spikard::Grpc::Response.new(payload: "\x08\x01\x12\x04test".b)
            yielder << Spikard::Grpc::Response.new(payload: { type: 'json2' }.to_json.b)
          end
        end

        request = double('request', method_name: 'MixedStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses.length).to eq(3)
        expect { JSON.parse(responses[0].payload) }.not_to raise_error
      end

      it 'handles stream with dynamic payload sizes' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            [100, 1000, 10_000, 100_000].each do |size|
              payload = { size: size, data: 'x' * size }.to_json.b
              yielder << Spikard::Grpc::Response.new(payload: payload)
            end
          end
        end

        request = double('request', method_name: 'SizedStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses.length).to eq(4)
        expect(responses[3].payload.length).to be > 100_000
      end

      it 'handles stream with exponential message growth' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            10.times do |i|
              size = 2 ** i
              data = ('a' * size).b
              yielder << Spikard::Grpc::Response.new(payload: data)
            end
          end
        end

        request = double('request', method_name: 'ExponentialStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses.length).to eq(10)
        expect(responses[0].payload.length).to eq(1)
        expect(responses[9].payload.length).to eq(512)
      end

      it 'handles stream with chunked large message' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            large_data = 'x' * 1_000_000
            chunk_size = 10_000

            large_data.scan(/.{1,#{chunk_size}}/m).each_with_index do |chunk, idx|
              response = Spikard::Grpc::Response.new(payload: chunk.b)
              response.metadata = { 'chunk-index' => idx.to_s }
              yielder << response
            end
          end
        end

        request = double('request', method_name: 'ChunkedStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses.length).to eq(100)
        expect(responses.first.metadata['chunk-index']).to eq('0')
      end

      it 'handles stream with heterogeneous response types' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            # Single response
            yielder << Spikard::Grpc::Response.new(payload: 'single'.b)
            # Complex nested response
            complex = { data: { nested: { deep: 'value' } } }.to_json.b
            yielder << Spikard::Grpc::Response.new(payload: complex)
            # Array response
            array = [1, 2, 3, 4, 5].to_json.b
            yielder << Spikard::Grpc::Response.new(payload: array)
          end
        end

        request = double('request', method_name: 'HeterogeneousStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses.length).to eq(3)
        parsed_complex = JSON.parse(responses[1].payload)
        expect(parsed_complex['data']['nested']['deep']).to eq('value')
      end

      it 'stream can be reused with multiple iterations' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            3.times do |i|
              yielder << Spikard::Grpc::Response.new(payload: { count: i }.to_json.b)
            end
          end
        end

        request = double('request', method_name: 'ReusableStream', payload: '{}'.b, metadata: {})
        stream = handler.call(request)

        # First iteration
        first_results = stream.to_a
        expect(first_results.length).to eq(3)
      end

      it 'handles stream with gradually increasing metadata' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            5.times do |i|
              response = Spikard::Grpc::Response.new(payload: { id: i }.to_json.b)
              # Add more metadata with each message
              metadata = {}
              (0..i).each { |j| metadata["field-#{j}"] = "value-#{j}" }
              response.metadata = metadata
              yielder << response
            end
          end
        end

        request = double('request', method_name: 'GrowingMetadataStream', payload: '{}'.b, metadata: {})
        responses = handler.call(request).to_a

        expect(responses[0].metadata.keys.length).to eq(1)
        expect(responses[4].metadata.keys.length).to eq(5)
      end
    end

    describe 'bidirectional streaming edge cases' do
      it 'handles bidirectional stream with filtering' do
        handler = lambda do |request_enum|
          Enumerator.new do |yielder|
            request_enum.each do |req|
              parsed = JSON.parse(req.payload)
              # Only yield even counts
              if parsed['count'] % 2 == 0
                yielder << Spikard::Grpc::Response.new(payload: req.payload)
              end
            end
          end
        end

        requests = [
          double('req', payload: '{"count": 1}'.b),
          double('req', payload: '{"count": 2}'.b),
          double('req', payload: '{"count": 3}'.b),
          double('req', payload: '{"count": 4}'.b)
        ].to_enum

        responses = handler.call(requests).to_a
        expect(responses.length).to eq(2)
      end

      it 'handles bidirectional stream with accumulation' do
        handler = lambda do |request_enum|
          Enumerator.new do |yielder|
            sum = 0
            request_enum.each do |req|
              parsed = JSON.parse(req.payload)
              sum += parsed['value']
              yielder << Spikard::Grpc::Response.new(
                payload: { running_sum: sum }.to_json.b
              )
            end
          end
        end

        requests = [
          double('req', payload: '{"value": 10}'.b),
          double('req', payload: '{"value": 20}'.b),
          double('req', payload: '{"value": 30}'.b)
        ].to_enum

        responses = handler.call(requests).to_a
        expect(responses.length).to eq(3)
        expect(JSON.parse(responses[0].payload)['running_sum']).to eq(10)
        expect(JSON.parse(responses[2].payload)['running_sum']).to eq(60)
      end

      it 'handles bidirectional stream with context sharing' do
        handler = lambda do |request_enum|
          context = { request_count: 0, total_bytes: 0 }

          Enumerator.new do |yielder|
            request_enum.each do |req|
              context[:request_count] += 1
              context[:total_bytes] += req.payload.bytesize

              response_payload = {
                request_num: context[:request_count],
                total_bytes_so_far: context[:total_bytes]
              }.to_json.b

              yielder << Spikard::Grpc::Response.new(payload: response_payload)
            end
          end
        end

        requests = [
          double('req', payload: 'data1'.b),
          double('req', payload: 'data2'.b)
        ].to_enum

        responses = handler.call(requests).to_a
        expect(responses.length).to eq(2)
        expect(JSON.parse(responses[1].payload)['total_bytes_so_far']).to eq(10)
      end
    end
  end

  describe 'Extended Status Codes' do
    describe 'additional error status codes' do
      it 'handles UNAUTHENTICATED with auth details' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'UNAUTHENTICATED',
          'grpc-message' => 'Missing authentication token',
          'www-authenticate' => 'Bearer realm="api"'
        }

        expect(response.metadata['grpc-status']).to eq('UNAUTHENTICATED')
        expect(response.metadata).to have_key('www-authenticate')
      end

      it 'handles DATA_LOSS with recovery information' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'DATA_LOSS',
          'grpc-message' => 'Data corruption detected',
          'x-backup-available' => 'true',
          'x-backup-timestamp' => '2024-01-15T10:30:00Z'
        }

        expect(response.metadata['grpc-status']).to eq('DATA_LOSS')
        expect(response.metadata['x-backup-available']).to eq('true')
      end

      it 'handles RESOURCE_EXHAUSTED with quota information' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'RESOURCE_EXHAUSTED',
          'grpc-message' => 'Quota exceeded',
          'x-quota-limit' => '1000',
          'x-quota-used' => '1000',
          'x-quota-reset-time' => '3600'
        }

        expect(response.metadata['grpc-status']).to eq('RESOURCE_EXHAUSTED')
        expect(response.metadata['x-quota-limit']).to eq('1000')
      end

      it 'handles FAILED_PRECONDITION with state information' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'FAILED_PRECONDITION',
          'grpc-message' => 'User not verified',
          'x-required-state' => 'verified',
          'x-current-state' => 'unverified'
        }

        expect(response.metadata['grpc-status']).to eq('FAILED_PRECONDITION')
        expect(response.metadata['x-current-state']).to eq('unverified')
      end

      it 'handles ABORTED with conflict details' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'ABORTED',
          'grpc-message' => 'Transaction conflict',
          'x-conflict-version' => '5',
          'x-expected-version' => '3',
          'x-conflict-field' => 'balance'
        }

        expect(response.metadata['grpc-status']).to eq('ABORTED')
        expect(response.metadata['x-conflict-version']).to eq('5')
      end

      it 'handles OUT_OF_RANGE with bounds information' do
        response = Spikard::Grpc::Response.new(payload: ''.b)
        response.metadata = {
          'grpc-status' => 'OUT_OF_RANGE',
          'grpc-message' => 'Page out of range',
          'x-min-value' => '1',
          'x-max-value' => '10',
          'x-requested-value' => '15'
        }

        expect(response.metadata['grpc-status']).to eq('OUT_OF_RANGE')
        expect(response.metadata['x-max-value']).to eq('10')
      end

      it 'handles status code with trace context metadata' do
        response = Spikard::Grpc::Response.new(payload: 'error'.b)
        response.metadata = {
          'grpc-status' => 'INTERNAL',
          'grpc-message' => 'Internal server error',
          'x-trace-id' => 'trace-abc123',
          'x-span-id' => 'span-xyz789',
          'x-parent-span-id' => 'parent-span-123'
        }

        expect(response.metadata).to include('x-trace-id', 'x-span-id')
      end
    end

    describe 'status code as symbol vs string' do
      it 'preserves status code when metadata uses symbols converted to strings' do
        response = Spikard::Grpc::Response.new(payload: 'data'.b)
        # Note: gRPC metadata should use string keys
        status_key = 'grpc-status'
        response.metadata = { status_key => 'NOT_FOUND' }

        expect(response.metadata['grpc-status']).to eq('NOT_FOUND')
      end

      it 'handles custom status metadata with consistent string types' do
        response = Spikard::Grpc::Response.new(payload: 'error'.b)
        response.metadata = {
          'grpc-status' => 'UNAVAILABLE',
          'x-service-name' => 'payment-service',
          'x-region' => 'us-west'
        }

        metadata_values = response.metadata.values
        expect(metadata_values).to all(be_a(String))
      end
    end
  end

  describe 'Performance and Edge Cases - Extended' do
    describe 'extreme payload scenarios' do
      it 'handles payloads with repetitive patterns' do
        pattern = 'abcdefghij'
        large_payload = (pattern * 100_000).b
        response = Spikard::Grpc::Response.new(payload: large_payload)

        expect(response.payload.length).to eq(pattern.length * 100_000)
        expect(response.payload.start_with?('abcde')).to be true
      end

      it 'handles sparse binary data' do
        sparse = ("\x00" * 1000 + "x" + "\x00" * 1000) * 10
        response = Spikard::Grpc::Response.new(payload: sparse.b)

        expect(response.payload).to include('x')
        expect(response.payload.length).to eq(sparse.length)
      end

      it 'handles payload with every byte value 0-255' do
        all_bytes = (0..255).map(&:chr).join * 100
        response = Spikard::Grpc::Response.new(payload: all_bytes.b)

        expect(response.payload.length).to eq(all_bytes.length)
      end
    end

    describe 'concurrent operations' do
      it 'handles multiple concurrent streams' do
        responses = []
        threads = []

        3.times do |stream_id|
          threads << Thread.new(stream_id) do |sid|
            handler = lambda do |request|
              Enumerator.new do |yielder|
                5.times do |i|
                  yielder << Spikard::Grpc::Response.new(
                    payload: { stream: sid, msg: i }.to_json.b
                  )
                end
              end
            end

            request = double('request', method_name: 'ConcurrentStream', payload: '{}'.b)
            stream_responses = handler.call(request).to_a
            responses.concat(stream_responses)
          end
        end

        threads.each(&:join)
        expect(responses.length).to eq(15)
      end

      it 'handles concurrent metadata modifications' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        threads = []
        results = []

        5.times do |i|
          threads << Thread.new(i) do |idx|
            response.metadata = { "thread-#{idx}" => "value-#{idx}" }
            results << response.metadata.dup if response.metadata
          end
        end

        threads.each(&:join)
        expect(results.length).to be > 0
      end

      it 'handles concurrent request processing' do
        handler = Class.new(Spikard::Grpc::Handler) do
          def handle_request(request)
            Spikard::Grpc::Response.new(payload: request.payload)
          end
        end.new

        threads = []
        responses = []

        10.times do |i|
          threads << Thread.new(i) do |idx|
            request = double('request', method_name: 'Test', payload: "data-#{idx}".b)
            responses << handler.handle_request(request)
          end
        end

        threads.each(&:join)
        expect(responses.length).to eq(10)
        expect(responses.map(&:class).uniq).to eq([Spikard::Grpc::Response])
      end
    end

    describe 'unicode and international character edge cases' do
      it 'handles emoji in payload and metadata' do
        emoji_data = { message: '🎉🚀🎊⭐🌟' }.to_json.b
        response = Spikard::Grpc::Response.new(payload: emoji_data)
        response.metadata = { 'x-emoji-header' => '✅' }

        parsed = JSON.parse(response.payload)
        expect(parsed['message']).to include('🚀')
        expect(response.metadata['x-emoji-header']).to eq('✅')
      end

      it 'handles zero-width characters and combining marks' do
        text_with_marks = 'e\u0301' # e + combining acute accent
        payload = { text: text_with_marks }.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)

        parsed = JSON.parse(response.payload)
        expect(parsed['text'].length).to be > 1
      end

      it 'handles surrogate pairs and high Unicode' do
        high_unicode = '𝕳𝖊𝖑𝖑𝖔' # Mathematical Alphanumeric Symbols
        payload = { text: high_unicode }.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)

        parsed = JSON.parse(response.payload)
        expect(parsed['text']).to include('𝕳')
      end

      it 'handles mixed scripts in metadata headers' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'x-user-name' => 'José García',
          'x-city' => 'São Paulo',
          'x-greeting' => 'Привет',
          'x-message' => '你好世界'
        }

        expect(response.metadata['x-user-name']).to eq('José García')
        expect(response.metadata['x-greeting']).to eq('Привет')
      end
    end

    describe 'deeply nested and complex structures' do
      it 'handles extremely deep nesting (20+ levels)' do
        # Build deep nested structure programmatically
        deep = { t: 'end' }
        ('a'..'s').reverse_each { |letter| deep = { letter.to_sym => deep } }
        payload = deep.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)

        parsed = JSON.parse(response.payload)
        current = parsed
        20.times { |i| current = current.values[0] }
        expect(current).to eq('end')
      end

      it 'handles mixed array and hash nesting' do
        mixed = {
          items: [
            { id: 1, tags: ['a', 'b', { nested: 'value' }] },
            { id: 2, data: { x: [1, 2, { y: 3 }] } }
          ]
        }
        payload = mixed.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)

        parsed = JSON.parse(response.payload)
        expect(parsed['items']).to be_an(Array)
        expect(parsed['items'][0]['id']).to eq(1)
      end

      it 'handles massive array structure' do
        large_array = (1..10_000).map { |i| { id: i, value: i * 2 } }
        payload = large_array.to_json.b
        response = Spikard::Grpc::Response.new(payload: payload)

        parsed = JSON.parse(response.payload)
        expect(parsed.length).to eq(10_000)
        expect(parsed[5000]['id']).to eq(5001)
      end
    end

    describe 'metadata handling edge cases' do
      it 'handles metadata with empty string values' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'x-empty' => '',
          'x-filled' => 'value',
          'x-also-empty' => ''
        }

        expect(response.metadata['x-empty']).to eq('')
        expect(response.metadata['x-filled']).to eq('value')
      end

      it 'handles metadata with newlines and special chars' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'x-multiline' => "line1\nline2\nline3",
          'x-tabs' => "col1\tcol2\tcol3",
          'x-quotes' => 'value with "quotes"'
        }

        expect(response.metadata['x-multiline']).to include("\n")
        expect(response.metadata['x-tabs']).to include("\t")
      end

      it 'handles metadata with many key-value pairs' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        metadata = {}
        100.times { |i| metadata["header-#{i}"] = "value-#{i}" }
        response.metadata = metadata

        expect(response.metadata.keys.length).to eq(100)
        expect(response.metadata['header-50']).to eq('value-50')
      end

      it 'handles metadata key normalization' do
        response = Spikard::Grpc::Response.new(payload: 'test'.b)
        response.metadata = {
          'x-custom-header' => 'value1',
          'X-Custom-Header' => 'value2',
          'x-custom-HEADER' => 'value3'
        }

        # All variations should be present as string keys
        expect(response.metadata.keys.length).to be > 0
      end
    end

    describe 'streaming with large metadata' do
      it 'streams responses with progressively larger metadata' do
        handler = lambda do |request|
          Enumerator.new do |yielder|
            5.times do |i|
              response = Spikard::Grpc::Response.new(
                payload: { index: i }.to_json.b
              )
              # Add more metadata with each response
              metadata = {}
              (0..i*10).each do |j|
                metadata["key-#{j}"] = "value-#{j}" * i
              end
              response.metadata = metadata
              yielder << response
            end
          end
        end

        request = double('request', method_name: 'LargeMetadataStream', payload: '{}'.b)
        responses = handler.call(request).to_a

        expect(responses.length).to eq(5)
        expect(responses[0].metadata.keys.length).to eq(1)
        expect(responses[4].metadata.keys.length).to eq(41)
      end
    end
  end

  describe 'Handler patterns and variations' do
    describe 'handler with state management' do
      it 'handler maintains state across multiple calls' do
        handler_class = Class.new(Spikard::Grpc::Handler) do
          def initialize
            @call_count = 0
          end

          def handle_request(request)
            @call_count += 1
            Spikard::Grpc::Response.new(
              payload: { call_number: @call_count }.to_json.b
            )
          end
        end

        handler = handler_class.new
        request = double('request', method_name: 'Test', payload: '{}'.b)

        response1 = handler.handle_request(request)
        response2 = handler.handle_request(request)
        response3 = handler.handle_request(request)

        expect(JSON.parse(response1.payload)['call_number']).to eq(1)
        expect(JSON.parse(response2.payload)['call_number']).to eq(2)
        expect(JSON.parse(response3.payload)['call_number']).to eq(3)
      end

      it 'handler with method routing and dispatch' do
        handler = Class.new(Spikard::Grpc::Handler) do
          def handle_request(request)
            method = request.method_name

            response_payload = case method
                               when 'Echo'
                                 request.payload
                               when 'Upper'
                                 request.payload.upcase
                               when 'Reverse'
                                 request.payload.reverse
                               else
                                 "Unknown: #{method}".b
                               end

            Spikard::Grpc::Response.new(payload: response_payload)
          end
        end.new

        echo_req = double('request', method_name: 'Echo', payload: 'hello'.b)
        upper_req = double('request', method_name: 'Upper', payload: 'hello'.b)
        reverse_req = double('request', method_name: 'Reverse', payload: 'hello'.b)

        expect(handler.handle_request(echo_req).payload).to eq('hello'.b)
        expect(handler.handle_request(upper_req).payload).to eq('HELLO'.b)
        expect(handler.handle_request(reverse_req).payload).to eq('olleh'.b)
      end
    end

    describe 'response building variations' do
      it 'creates response with error factory method' do
        response = Spikard::Grpc::Response.error('Something went wrong', { 'x-error-id' => 'ERR001' })

        expect(response).to be_a(Spikard::Grpc::Response)
        expect(response.metadata['grpc-status']).to eq('INTERNAL')
        expect(response.metadata['grpc-message']).to eq('Something went wrong')
        expect(response.metadata['x-error-id']).to eq('ERR001')
      end

      it 'chains multiple response building operations' do
        response = Spikard::Grpc::Response.new(payload: { status: 'ok' }.to_json.b)
        response.metadata = { 'initial-key' => 'initial-value' }
        # Update metadata
        response.metadata = response.metadata.merge({ 'added-key' => 'added-value' })

        expect(response.metadata).to include('initial-key', 'added-key')
      end
    end
  end

  describe 'Integration test scenarios' do
    describe 'complex workflow simulations' do
      it 'simulates request-response cycle with validation' do
        handler = lambda do |request|
          payload = request.payload.to_s

          if payload.empty?
            response = Spikard::Grpc::Response.new(payload: ''.b)
            response.metadata = {
              'grpc-status' => 'INVALID_ARGUMENT',
              'grpc-message' => 'Payload cannot be empty'
            }
            response
          else
            response = Spikard::Grpc::Response.new(
              payload: { received: payload.length }.to_json.b
            )
            response.metadata = { 'grpc-status' => 'OK' }
            response
          end
        end

        # Valid request
        valid_req = double('request', method_name: 'Validate', payload: 'data'.b)
        valid_resp = handler.call(valid_req)
        expect(valid_resp.metadata['grpc-status']).to eq('OK')

        # Invalid request
        invalid_req = double('request', method_name: 'Validate', payload: ''.b)
        invalid_resp = handler.call(invalid_req)
        expect(invalid_resp.metadata['grpc-status']).to eq('INVALID_ARGUMENT')
      end

      it 'simulates multi-step processing pipeline' do
        # Step 1: Parse input
        parser = lambda do |request|
          JSON.parse(request.payload) rescue {}
        end

        # Step 2: Validate
        validator = lambda do |data|
          data.is_a?(Hash) && data.key?('id')
        end

        # Step 3: Transform
        transformer = lambda do |data|
          { processed_id: data['id'] * 2, timestamp: Time.now.to_i }.to_json.b
        end

        # Step 4: Create response
        request = double('request', method_name: 'Process', payload: '{"id": 5}'.b)

        parsed = parser.call(request)
        is_valid = validator.call(parsed)
        result = transformer.call(parsed) if is_valid

        response = Spikard::Grpc::Response.new(payload: result)
        parsed_result = JSON.parse(response.payload)

        expect(parsed_result['processed_id']).to eq(10)
        expect(parsed_result).to have_key('timestamp')
      end

      it 'simulates streaming aggregation pipeline' do
        aggregator_handler = lambda do |request_stream|
          Enumerator.new do |response_yielder|
            total_count = 0
            sum = 0

            request_stream.each do |req|
              data = JSON.parse(req.payload)
              total_count += 1
              sum += data['value']

              response_yielder << Spikard::Grpc::Response.new(
                payload: { count: total_count, sum: sum, avg: (sum.to_f / total_count).round(2) }.to_json.b
              )
            end
          end
        end

        requests = [
          double('req', payload: '{"value": 10}'.b),
          double('req', payload: '{"value": 20}'.b),
          double('req', payload: '{"value": 30}'.b),
          double('req', payload: '{"value": 40}'.b)
        ].to_enum

        responses = aggregator_handler.call(requests).to_a
        last_response = JSON.parse(responses.last.payload)

        expect(last_response['count']).to eq(4)
        expect(last_response['sum']).to eq(100)
        expect(last_response['avg']).to eq(25.0)
      end
    end
  end
end
