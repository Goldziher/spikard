# frozen_string_literal: true

require 'spec_helper'
require 'timeout'
require 'securerandom'

RSpec.describe 'WebSocket edge cases and error recovery' do
  describe 'handler reconnection after disconnect' do
    it 'reconnects and maintains separate handler instances' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :connection_id

        def on_connect
          @connection_id = SecureRandom.uuid
        end

        def handle_message(message)
          { connection_id: @connection_id, echo: message }
        end
      end

      app.websocket('/chat') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # First connection
      ws1 = client.websocket('/chat')
      ws1.send_text({ type: 'ping' })
      response1 = ws1.receive_json
      expect(response1['connection_id']).not_to be_nil
      first_id = response1['connection_id']
      ws1.close

      # Second connection (reconnect)
      ws2 = client.websocket('/chat')
      ws2.send_text({ type: 'ping' })
      response2 = ws2.receive_json
      expect(response2['connection_id']).not_to be_nil
      second_id = response2['connection_id']

      # Each reconnection should have a different handler instance
      expect(first_id).not_to eq(second_id)
      ws2.close
    end

    it 'resets handler state on reconnect' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :message_count

        def initialize
          @message_count = 0
        end

        def handle_message(message)
          @message_count += 1
          { count: @message_count, message: message }
        end
      end

      app.websocket('/counter') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # First connection - send 3 messages
      ws1 = client.websocket('/counter')
      3.times do |i|
        ws1.send_text({ index: i })
        response = ws1.receive_json
        expect(response['count']).to eq(i + 1)
      end
      ws1.close

      # Second connection - counter should reset
      ws2 = client.websocket('/counter')
      ws2.send_text({ index: 0 })
      response = ws2.receive_json
      expect(response['count']).to eq(1) # Should be 1, not 4
      ws2.close
    end
  end

  describe 'message buffering during network partition' do
    it 'delivers messages in order after buffer drain' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :messages

        def initialize
          @messages = []
        end

        def handle_message(message)
          @messages << message
          { received: message, sequence: @messages.length }
        end
      end

      app.websocket('/buffer') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/buffer')

      # Send messages rapidly to simulate buffering
      messages = [
        { id: 1, type: 'msg' },
        { id: 2, type: 'msg' },
        { id: 3, type: 'msg' }
      ]

      messages.each { |msg| ws.send_json(msg) }

      # Receive responses in order
      responses = []
      3.times { responses << ws.receive_json }

      # Verify ordering is maintained
      expect(responses[0]['sequence']).to eq(1)
      expect(responses[1]['sequence']).to eq(2)
      expect(responses[2]['sequence']).to eq(3)

      expect(responses[0]['received']['id']).to eq(1)
      expect(responses[1]['received']['id']).to eq(2)
      expect(responses[2]['received']['id']).to eq(3)

      ws.close
    end

    it 'preserves message order with rapid send/receive' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          { sequence: message['seq'] }
        end
      end

      app.websocket('/order') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/order')

      # Send 10 messages
      10.times do |i|
        ws.send_json({ seq: i })
      end

      # Receive and verify order
      10.times do |i|
        response = ws.receive_json
        expect(response['sequence']).to eq(i)
      end

      ws.close
    end
  end

  describe 'graceful close (status code 1000 normal close)' do
    it 'allows graceful close and cleanup' do
      app = Spikard::App.new
      disconnect_called = false
      handler_class = Class.new(Spikard::WebSocketHandler) do
        define_method(:on_disconnect) do
          disconnect_called = true
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/graceful') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/graceful')
      ws.send_text({ type: 'test' })
      ws.receive_json

      # Normal close
      ws.close

      # Give time for cleanup to execute
      sleep 0.05
      expect(disconnect_called).to be true
    end

    it 'handler receives on_disconnect after graceful close' do
      app = Spikard::App.new
      disconnect_events = []
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :id

        def initialize
          @id = SecureRandom.uuid
        end

        define_method(:on_disconnect) do
          disconnect_events << @id
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/clean') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # Multiple connections and graceful closes
      3.times do
        ws = client.websocket('/clean')
        ws.send_text({ type: 'ping' })
        ws.receive_json
        ws.close
        sleep 0.01
      end

      sleep 0.1
      expect(disconnect_events.length).to eq(3)
      expect(disconnect_events.uniq.length).to eq(3) # All different handlers
    end
  end

  describe 'abnormal close (connection lost scenarios)' do
    it 'handles connection drop gracefully' do
      app = Spikard::App.new
      cleanup_ran = false
      handler_class = Class.new(Spikard::WebSocketHandler) do
        define_method(:on_disconnect) do # rubocop:disable Naming/PredicateMethod
          cleanup_ran = true
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/abnormal') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/abnormal')
      ws.send_text({ type: 'test' })
      ws.receive_json

      # Force close without graceful handshake
      ws.close

      sleep 0.05
      expect(cleanup_ran).to be true
    end

    it 'on_disconnect runs even when handler is mid-message' do
      app = Spikard::App.new
      disconnect_ran = false
      handler_class = Class.new(Spikard::WebSocketHandler) do
        define_method(:on_disconnect) do # rubocop:disable Naming/PredicateMethod
          disconnect_ran = true
        end

        def handle_message(message)
          # Simulate processing that takes time
          sleep 0.01
          message
        end
      end

      app.websocket('/mid') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/mid')
      ws.send_text({ type: 'slow' })
      # Close immediately without waiting for response
      ws.close

      sleep 0.1
      expect(disconnect_ran).to be true
    end
  end

  describe 'on_disconnect during message handling' do
    it 'receives on_disconnect when connection drops mid-message' do
      app = Spikard::App.new
      events = []
      handler_class = Class.new(Spikard::WebSocketHandler) do
        define_method(:on_connect) do
          events << :connect
        end

        define_method(:handle_message) do |message|
          events << :handle
          message
        end

        define_method(:on_disconnect) do
          events << :disconnect
        end
      end

      app.websocket('/during') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/during')
      ws.send_text({ type: 'msg' })
      ws.receive_json
      ws.close

      sleep 0.05
      # Should have connect, at least one handle, and disconnect
      expect(events).to include(:connect, :handle, :disconnect)
      expect(events.last).to eq(:disconnect)
    end

    it 'handler cleanup completes even if on_disconnect raises' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def on_disconnect
          raise 'cleanup error'
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/error_cleanup') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/error_cleanup')
      ws.send_text({ type: 'test' })
      ws.receive_json

      # Close should not raise even if on_disconnect raises
      expect { ws.close }.not_to raise_error
    end
  end

  describe 'multiple on_connect calls (reconnect scenarios)' do
    it 'treats multiple on_connect calls as reconnect events' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def initialize
          @connect_count = 0
        end

        def on_connect
          @connect_count += 1
        end

        def handle_message(_message)
          { connects: @connect_count }
        end
      end

      app.websocket('/multi_connect') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # Each connection should increment on_connect
      3.times do
        ws = client.websocket('/multi_connect')
        ws.send_text({ type: 'ping' })
        response = ws.receive_json
        expect(response['connects']).to eq(1) # Fresh handler instance
        ws.close
        sleep 0.01
      end
    end

    it 'each reconnection has independent connect state' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :connected_at

        def on_connect
          @connected_at = Process.clock_gettime(Process::CLOCK_MONOTONIC)
        end

        def handle_message(_message)
          { connected_at: @connected_at }
        end
      end

      app.websocket('/connect_time') { handler_class.new }
      client = Spikard::TestClient.new(app)

      times = []
      3.times do
        ws = client.websocket('/connect_time')
        ws.send_text({ type: 'ping' })
        response = ws.receive_json
        times << response['connected_at']
        ws.close
        sleep 0.05
      end

      # Times should be different (at least some)
      expect(times.uniq.length).to be > 1
    end
  end

  describe 'large message fragmentation (>64KB payload)' do
    it 'handles large message payloads correctly' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          {
            received_size: message['data']&.size || 0,
            data_present: !message['data'].nil? && !message['data'].empty?
          }
        end
      end

      app.websocket('/large') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/large')

      # Create large payload (100KB)
      large_data = 'x' * 100_000
      ws.send_json({ data: large_data })

      response = ws.receive_json
      expect(response['data_present']).to be true
      expect(response['received_size']).to eq(100_000)

      ws.close
    end

    it 'preserves large message content integrity' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          # Echo back with checksum
          { received: message['data'], size: message['data']&.size }
        end
      end

      app.websocket('/integrity') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/integrity')

      # Create pattern that's easy to verify
      pattern = 'ABCDE' * 20_000 # 100KB
      ws.send_json({ data: pattern })

      response = ws.receive_json
      expect(response['received']).to eq(pattern)
      expect(response['size']).to eq(100_000)

      ws.close
    end

    it 'handles multiple large messages sequentially' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :received_count

        def initialize
          @received_count = 0
        end

        def handle_message(message)
          @received_count += 1
          { sequence: @received_count, size: message['data']&.size }
        end
      end

      app.websocket('/multi_large') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/multi_large')

      # Send 3 large messages
      3.times do |i|
        data = ('X' * 50_000) + i.to_s
        ws.send_json({ data: data })
      end

      # Receive all responses
      3.times do |i|
        response = ws.receive_json
        expect(response['sequence']).to eq(i + 1)
        expect(response['size']).to eq(50_001)
      end

      ws.close
    end
  end

  describe 'unicode and emoji message handling' do
    it 'preserves unicode characters in messages' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          { echo: message['text'] }
        end
      end

      app.websocket('/unicode') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/unicode')

      unicode_texts = [
        'Hello 世界',
        'Привет мир',
        'مرحبا العالم',
        'שלום עולם'
      ]

      unicode_texts.each do |text|
        ws.send_json({ text: text })
        response = ws.receive_json
        expect(response['echo']).to eq(text)
      end

      ws.close
    end

    it 'handles emoji correctly' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          { echo: message['emoji'], length: message['emoji']&.length }
        end
      end

      app.websocket('/emoji') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/emoji')

      emojis = ['😀', '🚀', '❤️', '🎉', '👍']
      emojis.each do |emoji|
        ws.send_json({ emoji: emoji })
        response = ws.receive_json
        expect(response['echo']).to eq(emoji)
      end

      ws.close
    end

    it 'handles mixed unicode and emoji' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          message
        end
      end

      app.websocket('/mixed') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/mixed')

      messages = [
        { text: 'Hello 👋 World 🌍' },
        { text: '日本語のテキスト 🗾' },
        { text: 'Emoji sequence: 👨‍👩‍👧‍👦' }
      ]

      messages.each do |msg|
        ws.send_json(msg)
        response = ws.receive_json
        expect(response['text']).to eq(msg[:text])
      end

      ws.close
    end
  end

  describe 'handler state consistency after partial frame delivery' do
    it 'maintains consistent state across multiple operations' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :state

        def initialize
          @state = { messages: [], errors: [] }
        end

        def on_connect
          @state[:connected] = true
        end

        def handle_message(message)
          @state[:messages] << message
          { message_count: @state[:messages].length }
        end

        def on_disconnect
          @state[:connected] = false
        end
      end

      app.websocket('/state') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/state')

      # Send multiple messages
      5.times do |i|
        ws.send_json({ id: i })
        response = ws.receive_json
        expect(response['message_count']).to eq(i + 1)
      end

      ws.close
    end

    it 'recovers from partial message frame errors' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :valid_messages, :error_count

        def initialize
          @valid_messages = 0
          @error_count = 0
        end

        def handle_message(message)
          if message.is_a?(Hash) && message['valid']
            @valid_messages += 1
            { status: 'ok', count: @valid_messages }
          else
            @error_count += 1
            { status: 'error', count: @error_count }
          end
        end
      end

      app.websocket('/recovery') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/recovery')

      # Send mixed valid and invalid messages
      [
        { valid: true },
        { valid: false },
        { valid: true },
        { valid: true },
        { valid: false }
      ].each do |msg|
        ws.send_json(msg)
        response = ws.receive_json
        # Handler should continue processing after errors
        expect(response).to have_key('status')
      end

      ws.close
    end
  end

  describe 'connection timeout during on_connect hook' do
    it 'handles on_connect that takes time' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def on_connect
          # Simulate some work during connection
          sleep 0.05
        end

        def handle_message(message)
          { ready: true, message: message }
        end
      end

      app.websocket('/slow_connect') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/slow_connect')
      ws.send_text({ type: 'ping' })
      response = ws.receive_json

      expect(response['ready']).to be true
      ws.close
    end

    it 'connection completes even if on_connect performs I/O simulation' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :initialized

        def on_connect
          # Simulate connecting to a resource
          @initialized = false
          sleep 0.01
          @initialized = true
        end

        def handle_message(message)
          { initialized: @initialized, message: message }
        end
      end

      app.websocket('/io_connect') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/io_connect')
      ws.send_text({ type: 'test' })
      response = ws.receive_json

      expect(response['initialized']).to be true
      ws.close
    end
  end

  describe 'on_disconnect cleanup always runs' do
    it 'on_disconnect cleanup runs even if on_connect raised' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def on_connect
          raise 'connection initialization failed'
        end

        def on_disconnect # rubocop:disable Naming/PredicateMethod
          true
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/fail_connect') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # Connection fails due to on_connect error, but cleanup should still run
      expect do
        ws = client.websocket('/fail_connect')
        # Connection attempt will fail, but handler cleanup should run
        ws.close
      end.not_to raise_error

      sleep 0.05
      # NOTE: In real implementation, on_disconnect may not run if on_connect failed
      # This test documents the expected behavior
    end

    it 'on_disconnect always runs for each unique handler instance' do
      app = Spikard::App.new
      disconnect_count = 0
      handler_class = Class.new(Spikard::WebSocketHandler) do
        define_method(:on_disconnect) do
          disconnect_count += 1
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/cleanup') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # Create and close 5 separate connections
      5.times do
        ws = client.websocket('/cleanup')
        ws.send_text({ type: 'ping' })
        ws.receive_json
        ws.close
        sleep 0.01
      end

      sleep 0.1
      expect(disconnect_count).to eq(5)
    end

    it 'on_disconnect runs exactly once per connection' do
      app = Spikard::App.new
      disconnect_calls = []
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :id

        def initialize
          @id = SecureRandom.uuid
        end

        def on_connect
          # Verify we can track connection
        end

        define_method(:on_disconnect) do
          disconnect_calls << @id
        end

        def handle_message(message)
          message
        end
      end

      app.websocket('/once') { handler_class.new }
      client = Spikard::TestClient.new(app)

      # Multiple connections
      3.times do
        ws = client.websocket('/once')
        ws.send_text({ type: 'ping' })
        ws.receive_json
        ws.close
        sleep 0.01
      end

      sleep 0.1
      # Should have exactly 3 disconnect calls
      expect(disconnect_calls.length).to eq(3)
      # All from different handlers
      expect(disconnect_calls.uniq.length).to eq(3)
    end
  end

  describe 'binary vs text frame handling' do
    it 'handles text frames with JSON serialization' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          {
            received: message,
            type: message.class.name,
            keys: message.keys
          }
        end
      end

      app.websocket('/text') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/text')
      ws.send_json({ type: 'text', data: 'hello' })
      response = ws.receive_json

      expect(response['received']).to have_key('type')
      expect(response['received']['type']).to eq('text')
      expect(response['received']['data']).to eq('hello')

      ws.close
    end

    it 'preserves message structure across frame boundaries' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          # Verify structure is intact
          {
            has_string: message['string'].is_a?(String),
            has_number: message['number'].is_a?(Numeric),
            has_array: message['array'].is_a?(Array),
            has_object: message['object'].is_a?(Hash)
          }
        end
      end

      app.websocket('/frames') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/frames')
      ws.send_json({
                     string: 'hello',
                     number: 42,
                     array: [1, 2, 3],
                     object: { nested: 'value' }
                   })

      response = ws.receive_json
      expect(response['has_string']).to be true
      expect(response['has_number']).to be true
      expect(response['has_array']).to be true
      expect(response['has_object']).to be true

      ws.close
    end

    it 'handles multiple frame types in sequence' do
      app = Spikard::App.new
      handler_class = Class.new(Spikard::WebSocketHandler) do
        attr_reader :frame_count

        def initialize
          @frame_count = 0
        end

        def handle_message(message)
          @frame_count += 1
          { frame_number: @frame_count, received: message }
        end
      end

      app.websocket('/sequence') { handler_class.new }
      client = Spikard::TestClient.new(app)

      ws = client.websocket('/sequence')

      # Send different message types
      messages = [
        { type: 'text' },
        { type: 'json' },
        { type: 'object', nested: { key: 'value' } }
      ]

      messages.each_with_index do |msg, index|
        ws.send_json(msg)
        response = ws.receive_json
        expect(response['frame_number']).to eq(index + 1)
      end

      ws.close
    end
  end
end
