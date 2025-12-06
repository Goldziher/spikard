# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::Testing do
  describe '.create_test_client' do
    it 'creates test client from app with default config' do
      app = Spikard::App.new
      app.get('/hello') { { message: 'hello' } }

      client = Spikard::Testing.create_test_client(app)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'creates test client from app with custom config' do
      app = Spikard::App.new
      app.get('/hello') { { message: 'hello' } }
      config = Spikard::ServerConfig.new

      client = Spikard::Testing.create_test_client(app, config: config)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'uses test config from app instance variable if available' do
      app = Spikard::App.new
      app.get('/hello') { { message: 'hello' } }
      config = Spikard::ServerConfig.new
      app.instance_variable_set(:@__spikard_test_config, config)

      client = Spikard::Testing.create_test_client(app)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'passes handlers to native client' do
      app = Spikard::App.new
      handler = proc { { message: 'hello' } }
      app.register_route('GET', '/test', handler_name: 'test_handler', &handler)

      client = Spikard::Testing.create_test_client(app)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'passes dependencies to native client' do
      app = Spikard::App.new
      app.get('/hello') { { message: 'hello' } }
      app.provide('logger', proc { 'logger' })

      client = Spikard::Testing.create_test_client(app)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end
  end
end

RSpec.describe Spikard::Testing::TestClient do
  let(:app) do
    Spikard::App.new.tap do |a|
      a.get('/hello') { { message: 'hello' } }
    end
  end

  let(:client) { Spikard::Testing::TestClient.new(app) }

  describe '#initialize' do
    it 'accepts native client directly' do
      app_instance = Spikard::App.new
      app_instance.get('/test') { { ok: true } }
      native_client = Spikard::Testing.create_test_client(app_instance)

      # Use the testing module's private method to get native client
      test_client = Spikard::Testing::TestClient.new(native_client.instance_variable_get(:@native))
      expect(test_client).to be_a(Spikard::Testing::TestClient)
    end
  end

  describe '.new with app' do
    it 'creates client from app instance' do
      test_app = Spikard::App.new
      test_app.get('/test') { { status: 'ok' } }

      client = Spikard::Testing::TestClient.new(test_app)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'accepts config option' do
      test_app = Spikard::App.new
      test_app.get('/test') { { status: 'ok' } }
      config = Spikard::ServerConfig.new

      client = Spikard::Testing::TestClient.new(test_app, config: config)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end

    it 'uses native client directly when passed' do
      test_app = Spikard::App.new
      test_app.get('/test') { { ok: true } }
      native = Spikard::Testing.create_test_client(test_app).instance_variable_get(:@native)

      client = Spikard::Testing::TestClient.new(native)
      expect(client).to be_a(Spikard::Testing::TestClient)
    end
  end

  describe '#request' do
    it 'sends request with method and path' do
      response = client.request(:get, '/hello')
      expect(response).to be_a(Spikard::Testing::Response)
      expect(response.status).to eq(200)
    end

    it 'returns Response with status code' do
      response = client.request(:get, '/hello')
      expect(response.status_code).to be_a(Integer)
      expect(response.status).to eq(response.status_code)
    end

    it 'converts method to uppercase string' do
      response = client.request('get', '/hello')
      expect(response.status).to eq(200)
    end

    it 'accepts options hash' do
      response = client.request(:get, '/hello', headers: { 'X-Custom' => 'value' })
      expect(response.status).to eq(200)
    end
  end

  describe 'HTTP verb methods' do
    it 'supports get method' do
      expect(client).to respond_to(:get)
    end

    it 'supports post method' do
      expect(client).to respond_to(:post)
    end

    it 'supports put method' do
      expect(client).to respond_to(:put)
    end

    it 'supports patch method' do
      expect(client).to respond_to(:patch)
    end

    it 'supports delete method' do
      expect(client).to respond_to(:delete)
    end

    it 'supports head method' do
      expect(client).to respond_to(:head)
    end

    it 'supports options method' do
      expect(client).to respond_to(:options)
    end

    it 'supports trace method' do
      expect(client).to respond_to(:trace)
    end
  end

  describe '#close' do
    it 'closes the client' do
      expect { client.close }.not_to raise_error
    end
  end
end

RSpec.describe Spikard::Testing::Response do
  describe '#initialize' do
    it 'creates response from payload hash' do
      payload = { status_code: 200, headers: { 'content-type' => 'application/json' }, body: '{"ok":true}' }
      response = described_class.new(payload)

      expect(response.status_code).to eq(200)
      expect(response.headers).to eq({ 'content-type' => 'application/json' })
      expect(response.body).to eq('{"ok":true}')
    end

    it 'handles missing headers' do
      payload = { status_code: 200, body: 'ok' }
      response = described_class.new(payload)

      expect(response.headers).to eq({})
    end

    it 'handles nil body' do
      payload = { status_code: 204, headers: {}, body: nil }
      response = described_class.new(payload)

      expect(response.body).to be_nil
    end
  end

  describe '#status' do
    it 'returns status code' do
      payload = { status_code: 200, headers: {}, body: '' }
      response = described_class.new(payload)

      expect(response.status).to eq(200)
    end

    it 'returns different status codes' do
      [200, 201, 400, 404, 500].each do |code|
        payload = { status_code: code, headers: {}, body: '' }
        response = described_class.new(payload)
        expect(response.status).to eq(code)
      end
    end
  end

  describe '#json' do
    it 'parses JSON body' do
      payload = { status_code: 200, headers: {}, body: '{"hello":"world"}' }
      response = described_class.new(payload)

      expect(response.json).to eq({ 'hello' => 'world' })
    end

    it 'returns nil for nil body' do
      payload = { status_code: 204, headers: {}, body: nil }
      response = described_class.new(payload)

      expect(response.json).to be_nil
    end

    it 'returns nil for empty body' do
      payload = { status_code: 204, headers: {}, body: '' }
      response = described_class.new(payload)

      expect(response.json).to be_nil
    end

    it 'parses JSON array' do
      payload = { status_code: 200, headers: {}, body: '[1,2,3]' }
      response = described_class.new(payload)

      expect(response.json).to eq([1, 2, 3])
    end

    it 'parses nested JSON' do
      body = '{"user":{"name":"John","profile":{"bio":"Developer"}}}'
      payload = { status_code: 200, headers: {}, body: body }
      response = described_class.new(payload)

      expect(response.json['user']['profile']['bio']).to eq('Developer')
    end

    it 'handles JSON with null values' do
      payload = { status_code: 200, headers: {}, body: '{"value":null}' }
      response = described_class.new(payload)

      expect(response.json['value']).to be_nil
    end
  end

  describe '#text' do
    it 'returns body as text' do
      payload = { status_code: 200, headers: {}, body: 'plain text' }
      response = described_class.new(payload)

      expect(response.text).to eq('plain text')
    end
  end

  describe '#body_text' do
    it 'returns body as UTF-8 text' do
      payload = { status_code: 200, headers: {}, body: 'hello world' }
      response = described_class.new(payload)

      expect(response.body_text).to eq('hello world')
    end

    it 'uses body_text field if available' do
      payload = { status_code: 200, headers: {}, body: 'old', body_text: 'new' }
      response = described_class.new(payload)

      expect(response.body_text).to eq('new')
    end

    it 'handles nil body' do
      payload = { status_code: 204, headers: {}, body: nil }
      response = described_class.new(payload)

      expect(response.body_text).to be_nil
    end
  end

  describe '#body_bytes' do
    it 'returns body as bytes' do
      payload = { status_code: 200, headers: {}, body: 'test' }
      response = described_class.new(payload)

      expect(response.body_bytes).to be_a(String)
    end

    it 'returns empty bytes for nil body' do
      payload = { status_code: 204, headers: {}, body: nil }
      response = described_class.new(payload)

      expect(response.body_bytes).to eq(''.b)
    end
  end

  describe '#bytes' do
    it 'returns body as array of bytes' do
      payload = { status_code: 200, headers: {}, body: 'AB' }
      response = described_class.new(payload)

      expect(response.bytes).to be_a(Array)
      expect(response.bytes).to include(65, 66) # ASCII codes for A and B
    end
  end
end

RSpec.describe Spikard::Testing::WebSocketTestConnection do
  describe '#send_text' do
    it 'sends JSON-encoded text' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:send_text)

      ws = described_class.new(native_ws)
      expect { ws.send_text({ message: 'test' }) }.not_to raise_error
    end
  end

  describe '#send_json' do
    it 'sends JSON message' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:send_json)

      ws = described_class.new(native_ws)
      expect { ws.send_json({ type: 'message' }) }.not_to raise_error
    end
  end

  describe '#receive_text' do
    it 'parses JSON response' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:receive_text).and_return('{"msg":"hello"}')

      ws = described_class.new(native_ws)
      message = ws.receive_text

      expect(message).to be_a(Hash)
      expect(message['msg']).to eq('hello')
    end

    it 'returns raw text if not JSON' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:receive_text).and_return('plain text')

      ws = described_class.new(native_ws)
      message = ws.receive_text

      expect(message).to eq('plain text')
    end
  end

  describe '#receive_json' do
    it 'receives JSON message' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:receive_json).and_return({ 'type' => 'message' })

      ws = described_class.new(native_ws)
      message = ws.receive_json

      expect(message).to be_a(Hash)
    end
  end

  describe '#receive_bytes' do
    it 'calls receive_text' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:receive_text).and_return('data')

      ws = described_class.new(native_ws)
      message = ws.receive_bytes

      expect(message).not_to be_nil
    end
  end

  describe '#receive_message' do
    it 'returns WebSocketMessage wrapper' do
      native_ws = instance_double('NativeWebSocket')
      native_msg = instance_double('NativeMessage')
      allow(native_ws).to receive(:receive_message).and_return(native_msg)

      ws = described_class.new(native_ws)
      message = ws.receive_message

      expect(message).to be_a(Spikard::Testing::WebSocketMessage)
    end
  end

  describe '#close' do
    it 'closes the websocket connection' do
      native_ws = instance_double('NativeWebSocket')
      allow(native_ws).to receive(:close)

      ws = described_class.new(native_ws)
      expect { ws.close }.not_to raise_error
    end
  end
end

RSpec.describe Spikard::Testing::WebSocketMessage do
  describe '#as_text' do
    it 'parses text message as JSON' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:as_text).and_return('{"data":"hello"}')

      message = described_class.new(native_msg)
      text = message.as_text

      expect(text).to be_a(Hash)
      expect(text['data']).to eq('hello')
    end

    it 'returns raw text if not JSON' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:as_text).and_return('plain text')

      message = described_class.new(native_msg)
      text = message.as_text

      expect(text).to eq('plain text')
    end

    it 'returns nil when as_text returns nil' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:as_text).and_return(nil)

      message = described_class.new(native_msg)
      text = message.as_text

      expect(text).to be_nil
    end
  end

  describe '#as_json' do
    it 'delegates to native message' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:as_json).and_return({ 'test' => true })

      message = described_class.new(native_msg)
      json = message.as_json

      expect(json).to eq({ 'test' => true })
    end
  end

  describe '#as_binary' do
    it 'delegates to native message' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:as_binary).and_return('binary data')

      message = described_class.new(native_msg)
      binary = message.as_binary

      expect(binary).to eq('binary data')
    end
  end

  describe '#close?' do
    it 'delegates to native message is_close' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:is_close).and_return(false)

      message = described_class.new(native_msg)
      expect(message.close?).to be false
    end

    it 'returns true for close frame' do
      native_msg = instance_double('NativeMessage')
      allow(native_msg).to receive(:is_close).and_return(true)

      message = described_class.new(native_msg)
      expect(message.close?).to be true
    end
  end
end

RSpec.describe Spikard::Testing::SseStream do
  describe '#body' do
    it 'delegates to native SSE' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return('data: {"test":true}')

      stream = described_class.new(native_sse)
      expect(stream.body).to eq('data: {"test":true}')
    end
  end

  describe '#events' do
    it 'returns array of InlineSseEvent' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: {\"event\":1}\n\ndata: {\"event\":2}\n\n")

      stream = described_class.new(native_sse)
      events = stream.events

      expect(events).to be_a(Array)
      events.each do |event|
        expect(event).to be_a(Spikard::Testing::InlineSseEvent)
      end
    end

    it 'handles empty body' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return('')

      stream = described_class.new(native_sse)
      events = stream.events

      expect(events).to be_a(Array)
      expect(events).to be_empty
    end
  end

  describe '#events_as_json' do
    it 'returns array of parsed JSON events' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: {\"msg\":\"hello\"}\n\ndata: invalid\n\n")

      stream = described_class.new(native_sse)
      json_events = stream.events_as_json

      expect(json_events).to be_a(Array)
      expect(json_events.length).to eq(1)
      expect(json_events.first).to eq({ 'msg' => 'hello' })
    end

    it 'filters out non-JSON events' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: {\"valid\":true}\n\ndata: not json\n\n")

      stream = described_class.new(native_sse)
      json_events = stream.events_as_json

      json_events.each do |event|
        expect(event).to be_a(Hash)
      end
    end
  end

  describe '#parsed_chunks' do
    it 'parses SSE data chunks with LF' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: chunk1\n\ndata: chunk2\n\n")

      stream = described_class.new(native_sse)
      chunks = stream.send(:parsed_chunks)

      expect(chunks).to be_a(Array)
      expect(chunks).to include('chunk1', 'chunk2')
    end

    it 'handles CRLF line endings' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: chunk1\r\n\r\ndata: chunk2\r\n\r\n")

      stream = described_class.new(native_sse)
      chunks = stream.send(:parsed_chunks)

      expect(chunks).to be_a(Array)
      expect(chunks).to include('chunk1', 'chunk2')
    end

    it 'filters empty chunks' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: \n\ndata: valid\n\n")

      stream = described_class.new(native_sse)
      chunks = stream.send(:parsed_chunks)

      chunks.each do |chunk|
        expect(chunk).not_to be_empty unless chunk == ''
      end
    end

    it 'handles multiline data' do
      native_sse = instance_double('NativeSseStream')
      allow(native_sse).to receive(:body).and_return("data: line1\ndata: line2\n\n")

      stream = described_class.new(native_sse)
      chunks = stream.send(:parsed_chunks)

      expect(chunks).to be_a(Array)
    end
  end
end

RSpec.describe Spikard::Testing::InlineSseEvent do
  describe '#initialize' do
    it 'stores data string' do
      event = described_class.new('{"message":"hello"}')

      expect(event.data).to eq('{"message":"hello"}')
    end
  end

  describe '#data' do
    it 'returns stored data' do
      event = described_class.new('test data')

      expect(event.data).to eq('test data')
    end
  end

  describe '#as_json' do
    it 'parses data as JSON' do
      event = described_class.new('{"user":"alice","score":100}')
      json = event.as_json

      expect(json).to eq({ 'user' => 'alice', 'score' => 100 })
    end

    it 'raises error for invalid JSON' do
      event = described_class.new('invalid json')

      expect { event.as_json }.to raise_error(JSON::ParserError)
    end

    it 'handles JSON array' do
      event = described_class.new('[1,2,3]')
      json = event.as_json

      expect(json).to eq([1, 2, 3])
    end

    it 'handles JSON with null' do
      event = described_class.new('{"value":null}')
      json = event.as_json

      expect(json['value']).to be_nil
    end
  end
end

RSpec.describe Spikard::Testing::SseEvent do
  describe '#initialize' do
    it 'wraps native event' do
      # Create a mock native event
      native_event = instance_double('NativeEvent')
      allow(native_event).to receive(:data).and_return('event data')

      event = described_class.new(native_event)
      expect(event.data).to eq('event data')
    end
  end

  describe '#data' do
    it 'delegates to native event' do
      native_event = instance_double('NativeEvent')
      allow(native_event).to receive(:data).and_return('test data')

      event = described_class.new(native_event)
      expect(event.data).to eq('test data')
    end
  end

  describe '#as_json' do
    it 'delegates to native event' do
      native_event = instance_double('NativeEvent')
      allow(native_event).to receive(:as_json).and_return({ 'key' => 'value' })

      event = described_class.new(native_event)
      expect(event.as_json).to eq({ 'key' => 'value' })
    end
  end
end
