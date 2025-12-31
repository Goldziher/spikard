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
          def handle_request(request)
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

        expect {
          service.register_handler('test.Service', invalid_handler)
        }.to raise_error(ArgumentError, /must respond to :handle_request/)
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
          def handle_request(request)
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
          def handle_request(request)
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
        def handle_request(request)
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
end
