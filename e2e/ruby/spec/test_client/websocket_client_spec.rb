# frozen_string_literal: true

require 'json'

RSpec.describe 'WebSocket Test Client', :websocket do
  def create_echo_app
    app = Spikard::App.new
    app.websocket('/echo') do |_req, ws|
      loop do
        msg = ws.receive_message
        break if msg.close?

        if msg.as_text
          ws.send_text(msg.as_text)
        elsif msg.as_binary
          ws.send_binary(msg.as_binary)
        end
      end
    end
    app
  end

  def create_json_echo_app
    app = Spikard::App.new
    app.websocket('/json-echo') do |_req, ws|
      loop do
        msg = ws.receive_message
        break if msg.close?

        if msg.as_json
          data = msg.as_json
          data['echoed'] = true
          ws.send_json(data)
        end
      end
    end
    app
  end

  describe 'basic text messaging' do
    it 'sends and receives text messages' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      ws.send_text('Hello, WebSocket!')
      response = ws.receive_text

      expect(response).to eq('Hello, WebSocket!')

      ws.close
      client.close
    end

    it 'handles empty strings' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      ws.send_text('')
      response = ws.receive_text

      expect(response).to eq('')

      ws.close
      client.close
    end

    it 'handles unicode text' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      message = '你好世界 🚀'
      ws.send_text(message)
      response = ws.receive_text

      expect(response).to eq(message)

      ws.close
      client.close
    end
  end

  describe 'JSON messaging' do
    it 'sends and receives JSON messages' do
      app = create_json_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/json-echo')

      message = { 'type' => 'greeting', 'text' => 'Hello' }
      ws.send_json(message)
      response = ws.receive_json

      expect(response['type']).to eq('greeting')
      expect(response['text']).to eq('Hello')
      expect(response['echoed']).to eq(true)

      ws.close
      client.close
    end

    it 'handles complex JSON structures' do
      app = create_json_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/json-echo')

      message = {
        'user' => {
          'id' => 123,
          'name' => 'Alice',
          'tags' => %w[admin user]
        },
        'timestamp' => 1_234_567_890
      }
      ws.send_json(message)
      response = ws.receive_json

      expect(response['user']['id']).to eq(123)
      expect(response['user']['name']).to eq('Alice')
      expect(response['user']['tags']).to eq(%w[admin user])
      expect(response['echoed']).to eq(true)

      ws.close
      client.close
    end
  end

  describe 'binary messaging' do
    it 'sends and receives binary data' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      # Send bytes
      bytes = [0x48, 0x65, 0x6c, 0x6c, 0x6f].pack('C*')
      ws.send_text(bytes) # send_text works for bytes too in Ruby
      response = ws.receive_bytes

      expect(response).to eq(bytes)

      ws.close
      client.close
    end
  end

  describe 'message types' do
    it 'receives message and checks type' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      ws.send_text('test message')
      msg = ws.receive_message

      expect(msg.as_text).to eq('test message')
      expect(msg.close?).to be false

      ws.close
      client.close
    end
  end

  describe 'sequential messages' do
    it 'sends and receives multiple messages in sequence' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      messages = ['first', 'second', 'third']
      messages.each do |msg|
        ws.send_text(msg)
      end

      responses = messages.map { ws.receive_text }
      expect(responses).to eq(messages)

      ws.close
      client.close
    end
  end

  describe 'special characters' do
    it 'handles special characters in text' do
      app = create_echo_app
      client = Spikard::Testing.create_test_client(app)
      ws = client.websocket('/echo')

      message = "Line 1\nLine 2\tTabbed\r\nWindows line"
      ws.send_text(message)
      response = ws.receive_text

      expect(response).to eq(message)

      ws.close
      client.close
    end
  end
end
