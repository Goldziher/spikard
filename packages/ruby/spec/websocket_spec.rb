# frozen_string_literal: true

require 'spec_helper'
require 'securerandom'

RSpec.describe Spikard::WebSocketHandler do
  describe '#handle_message' do
    it 'raises NotImplementedError when not implemented' do
      handler = described_class.new
      expect do
        handler.handle_message({ type: 'test' })
      end.to raise_error(NotImplementedError, /must implement #handle_message/)
    end

    it 'raises NotImplementedError with class name in message' do
      handler = described_class.new
      expect do
        handler.handle_message({})
      end.to raise_error(NotImplementedError, /Spikard::WebSocketHandler/)
    end

    it 'allows subclasses to override handle_message' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          { echoed: message }
        end
      end

      handler = subclass.new
      result = handler.handle_message({ data: 'test' })
      expect(result).to eq({ echoed: { data: 'test' } })
    end

    it 'accepts hash messages' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          message
        end
      end

      handler = subclass.new
      message = { type: 'ping', id: 123 }
      result = handler.handle_message(message)
      expect(result).to eq(message)
    end

    it 'returns nil when handler returns nil' do
      subclass = Class.new(described_class) do
        def handle_message(_message)
          nil
        end
      end

      handler = subclass.new
      result = handler.handle_message({ data: 'test' })
      expect(result).to be_nil
    end

    it 'returns hash responses' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          { response: 'received', original_type: message[:type] }
        end
      end

      handler = subclass.new
      result = handler.handle_message({ type: 'hello' })
      expect(result).to be_a(Hash)
      expect(result[:response]).to eq('received')
      expect(result[:original_type]).to eq('hello')
    end

    it 'allows modification of message before returning' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          message.merge(processed: true, timestamp: Time.now.to_i)
        end
      end

      handler = subclass.new
      result = handler.handle_message({ data: 'input' })
      expect(result[:data]).to eq('input')
      expect(result[:processed]).to be true
      expect(result).to have_key(:timestamp)
    end

    it 'can access message nested structures' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          { user_id: message.dig(:user, :id) }
        end
      end

      handler = subclass.new
      result = handler.handle_message({ user: { id: 'u-123', name: 'Alice' } })
      expect(result[:user_id]).to eq('u-123')
    end

    it 'handles empty messages' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          message.empty? ? { empty: true } : { empty: false }
        end
      end

      handler = subclass.new
      result = handler.handle_message({})
      expect(result).to eq({ empty: true })
    end
  end

  describe '#on_connect' do
    it 'has default implementation that does nothing' do
      handler = described_class.new
      expect { handler.on_connect }.not_to raise_error
    end

    it 'can be overridden in subclass' do
      connected = false
      subclass = Class.new(described_class) do
        define_method(:on_connect) do
          connected = true
        end
      end

      handler = subclass.new
      handler.on_connect
      expect(connected).to be true
    end

    it 'allows subclasses to perform initialization' do
      subclass = Class.new(described_class) do
        attr_reader :initialized

        def on_connect
          @initialized = true
        end
      end

      handler = subclass.new
      expect(handler.initialized).to be_nil
      handler.on_connect
      expect(handler.initialized).to be true
    end

    it 'can access instance state on connect' do
      subclass = Class.new(described_class) do
        attr_accessor :client_count

        def on_connect
          @client_count = (@client_count || 0) + 1
        end
      end

      handler = subclass.new
      expect(handler.client_count).to be_nil
      handler.on_connect
      expect(handler.client_count).to eq(1)
      handler.on_connect
      expect(handler.client_count).to eq(2)
    end

    it 'can initialize resources on connect' do
      subclass = Class.new(described_class) do
        attr_reader :connection_id

        def on_connect
          @connection_id = SecureRandom.uuid
        end
      end

      handler = subclass.new
      handler.on_connect
      expect(handler.connection_id).not_to be_nil
      expect(handler.connection_id).to match(/\A[0-9a-f-]{36}\z/)
    end

    it 'does not break when called multiple times' do
      subclass = Class.new(described_class) do
        attr_reader :call_count

        def on_connect
          @call_count = (@call_count || 0) + 1
        end
      end

      handler = subclass.new
      3.times { handler.on_connect }
      expect(handler.call_count).to eq(3)
    end

    it 'returns nil by default' do
      handler = described_class.new
      result = handler.on_connect
      expect(result).to be_nil
    end
  end

  describe '#on_disconnect' do
    it 'has default implementation that does nothing' do
      handler = described_class.new
      expect { handler.on_disconnect }.not_to raise_error
    end

    it 'can be overridden in subclass' do
      disconnected = false
      subclass = Class.new(described_class) do
        define_method(:on_disconnect) do
          disconnected = true
        end
      end

      handler = subclass.new
      handler.on_disconnect
      expect(disconnected).to be true
    end

    it 'allows subclasses to perform cleanup' do
      subclass = Class.new(described_class) do
        attr_reader :cleaned_up

        def on_disconnect
          @cleaned_up = true
        end
      end

      handler = subclass.new
      expect(handler.cleaned_up).to be_nil
      handler.on_disconnect
      expect(handler.cleaned_up).to be true
    end

    it 'can access instance state on disconnect' do
      subclass = Class.new(described_class) do
        attr_accessor :active

        def initialize
          @active = true
        end

        def on_disconnect
          @active = false
        end
      end

      handler = subclass.new
      expect(handler.active).to be true
      handler.on_disconnect
      expect(handler.active).to be false
    end

    it 'can log or record disconnect events' do
      subclass = Class.new(described_class) do
        attr_reader :disconnect_times

        def initialize
          @disconnect_times = []
        end

        def on_disconnect
          @disconnect_times << Time.now
        end
      end

      handler = subclass.new
      handler.on_disconnect
      handler.on_disconnect
      expect(handler.disconnect_times.length).to eq(2)
      expect(handler.disconnect_times.all? { |t| t.is_a?(Time) }).to be true
    end

    it 'returns nil by default' do
      handler = described_class.new
      result = handler.on_disconnect
      expect(result).to be_nil
    end

    it 'does not affect instance state of other handlers' do
      subclass = Class.new(described_class) do
        attr_accessor :disconnected

        def on_disconnect
          @disconnected = true
        end
      end

      handler1 = subclass.new
      handler2 = subclass.new

      handler1.on_disconnect
      expect(handler1.disconnected).to be true
      expect(handler2.disconnected).to be_nil
    end
  end

  describe 'handler lifecycle integration' do
    it 'follows proper connect, handle, disconnect sequence' do
      call_sequence = []
      subclass = Class.new(described_class) do
        define_method(:on_connect) do
          call_sequence << :connect
        end

        define_method(:handle_message) do |message|
          call_sequence << :handle
          message
        end

        define_method(:on_disconnect) do
          call_sequence << :disconnect
        end
      end

      handler = subclass.new
      handler.on_connect
      handler.handle_message({ type: 'test' })
      handler.on_disconnect

      expect(call_sequence).to eq(%i[connect handle disconnect])
    end

    it 'allows multiple messages between connect and disconnect' do
      subclass = Class.new(described_class) do
        attr_reader :message_count

        def initialize
          @message_count = 0
        end

        def handle_message(message)
          @message_count += 1
          message
        end
      end

      handler = subclass.new
      3.times { handler.handle_message({ id: 1 }) }
      expect(handler.message_count).to eq(3)
    end

    it 'maintains state through message handling' do
      subclass = Class.new(described_class) do
        attr_reader :messages

        def initialize
          @messages = []
        end

        def handle_message(message)
          @messages << message
          message
        end
      end

      handler = subclass.new
      handler.handle_message({ type: 'msg1' })
      handler.handle_message({ type: 'msg2' })
      handler.handle_message({ type: 'msg3' })

      expect(handler.messages.length).to eq(3)
      expect(handler.messages.map { |m| m[:type] }).to eq(%w[msg1 msg2 msg3])
    end
  end

  describe 'subclass implementation patterns' do
    it 'supports echo handler pattern' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          message
        end
      end

      handler = subclass.new
      test_msg = { type: 'echo', data: 'hello' }
      result = handler.handle_message(test_msg)
      expect(result).to eq(test_msg)
    end

    it 'supports transformation handler pattern' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          {
            original: message,
            processed_at: Time.now.to_i,
            uppercase_data: message[:data]&.upcase
          }
        end
      end

      handler = subclass.new
      result = handler.handle_message({ data: 'hello' })
      expect(result[:original]).to eq({ data: 'hello' })
      expect(result[:uppercase_data]).to eq('HELLO')
      expect(result).to have_key(:processed_at)
    end

    it 'supports conditional response handler pattern' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          case message[:type]
          when 'ping'
            { type: 'pong' }
          when 'echo'
            { type: 'echo', data: message[:data] }
          end
        end
      end

      handler = subclass.new
      ping_result = handler.handle_message({ type: 'ping' })
      expect(ping_result).to eq({ type: 'pong' })

      echo_result = handler.handle_message({ type: 'echo', data: 'test' })
      expect(echo_result).to eq({ type: 'echo', data: 'test' })

      unknown_result = handler.handle_message({ type: 'unknown' })
      expect(unknown_result).to be_nil
    end

    it 'supports stateful handler pattern' do
      subclass = Class.new(described_class) do
        attr_reader :state

        def initialize
          @state = {}
        end

        def on_connect
          @state[:connected_at] = Time.now
          @state[:message_count] = 0
        end

        def handle_message(message)
          @state[:message_count] += 1
          @state[:last_message] = message
          { acknowledged: true, count: @state[:message_count] }
        end

        def on_disconnect
          @state[:disconnected_at] = Time.now
        end
      end

      handler = subclass.new
      handler.on_connect
      expect(handler.state[:connected_at]).not_to be_nil

      result1 = handler.handle_message({ data: 'msg1' })
      expect(result1[:count]).to eq(1)

      result2 = handler.handle_message({ data: 'msg2' })
      expect(result2[:count]).to eq(2)

      handler.on_disconnect
      expect(handler.state[:disconnected_at]).not_to be_nil
    end
  end

  describe 'error handling and edge cases' do
    it 'propagates errors raised in handle_message' do
      subclass = Class.new(described_class) do
        def handle_message(_message)
          raise StandardError, 'processing failed'
        end
      end

      handler = subclass.new
      expect do
        handler.handle_message({})
      end.to raise_error(StandardError, 'processing failed')
    end

    it 'propagates errors raised in on_connect' do
      subclass = Class.new(described_class) do
        def on_connect
          raise 'connection failed'
        end
      end

      handler = subclass.new
      expect do
        handler.on_connect
      end.to raise_error(RuntimeError, 'connection failed')
    end

    it 'propagates errors raised in on_disconnect' do
      subclass = Class.new(described_class) do
        def on_disconnect
          raise ArgumentError, 'cleanup failed'
        end
      end

      handler = subclass.new
      expect do
        handler.on_disconnect
      end.to raise_error(ArgumentError, 'cleanup failed')
    end

    it 'handles nil messages gracefully' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          message.nil? ? { error: 'nil message' } : message
        end
      end

      handler = subclass.new
      result = handler.handle_message(nil)
      expect(result).to eq({ error: 'nil message' })
    end

    it 'handles messages with string keys' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          { received: message['type'] }
        end
      end

      handler = subclass.new
      result = handler.handle_message({ 'type' => 'test' })
      expect(result[:received]).to eq('test')
    end

    it 'handles complex nested messages' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          {
            user_id: message.dig(:user, :id),
            items: message.dig(:payload, :items),
            nested_value: message.dig(:a, :b, :c, :d)
          }
        end
      end

      handler = subclass.new
      message = {
        user: { id: 'u123', name: 'Bob' },
        payload: { items: [1, 2, 3] },
        a: { b: { c: { d: 'deep' } } }
      }
      result = handler.handle_message(message)
      expect(result[:user_id]).to eq('u123')
      expect(result[:items]).to eq([1, 2, 3])
      expect(result[:nested_value]).to eq('deep')
    end

    it 'handles large message payloads' do
      subclass = Class.new(described_class) do
        def handle_message(message)
          { size: message.to_s.length }
        end
      end

      handler = subclass.new
      large_message = { data: 'x' * 10_000 }
      result = handler.handle_message(large_message)
      expect(result[:size]).to be > 10_000
    end
  end

  describe 'integration with app websocket routes' do
    it 'works with Spikard::App' do
      app = Spikard::App.new

      subclass = Class.new(described_class) do
        def handle_message(message)
          { echo: message }
        end
      end

      app.websocket('/chat') do
        subclass.new
      end

      expect(app.websocket_handlers).to have_key('/chat')
    end

    it 'allows multiple websocket handlers' do
      app = Spikard::App.new

      handler1 = Class.new(described_class) do
        def handle_message(message)
          message
        end
      end

      handler2 = Class.new(described_class) do
        def handle_message(message)
          message
        end
      end

      app.websocket('/ws1') { handler1.new }
      app.websocket('/ws2') { handler2.new }

      expect(app.websocket_handlers.keys).to match_array(['/ws1', '/ws2'])
    end

    it 'handler factory returns instance on each call' do
      app = Spikard::App.new
      subclass = Class.new(described_class) do
        def handle_message(message)
          message
        end
      end

      app.websocket('/ws') { subclass.new }

      factory = app.websocket_handlers['/ws']
      handler1 = factory.call(nil)
      handler2 = factory.call(nil)

      expect(handler1).to be_a(described_class)
      expect(handler2).to be_a(described_class)
      expect(handler1).not_to equal(handler2)
    end
  end

  describe 'documentation compliance' do
    it 'supports documented example pattern' do
      # From the example in websocket.rb
      handler_class = Class.new(described_class) do
        def handle_message(message)
          message
        end

        def on_connect
          puts 'Client connected'
        end

        def on_disconnect
          puts 'Client disconnected'
        end
      end

      app = Spikard::App.new
      app.websocket('/chat') do
        handler_class.new
      end

      expect(app.websocket_handlers).to have_key('/chat')
      factory = app.websocket_handlers['/chat']
      instance = factory.call(nil)
      expect(instance).to be_a(handler_class)
    end
  end
end
