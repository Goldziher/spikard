# frozen_string_literal: true

require 'json'

RSpec.describe 'SSE Test Client' do
  def create_simple_sse_app
    app = Spikard::App.new
    app.get('/events', handler_name: 'sse_events') do |_req|
      events = [
        "data: first event\n\n",
        "data: second event\n\n",
        "data: third event\n\n"
      ].join
      Spikard::Response.new(body: events, content_type: 'text/event-stream')
    end
    app
  end

  def create_json_sse_app
    app = Spikard::App.new
    app.get('/json-events', handler_name: 'sse_json') do |_req|
      events = [
        { type: 'greeting', message: 'Hello' },
        { type: 'update', value: 42 },
        { type: 'farewell', message: 'Goodbye' }
      ]
      body = events.map { |e| "data: #{JSON.generate(e)}\n\n" }.join
      Spikard::Response.new(body: body, content_type: 'text/event-stream')
    end
    app
  end

  def create_complex_json_app
    app = Spikard::App.new
    app.get('/complex', handler_name: 'sse_complex') do |_req|
      event = {
        user: {
          id: 123,
          name: 'Alice',
          tags: %w[admin user]
        },
        nested: {
          deep: {
            value: 'found'
          }
        }
      }
      Spikard::Response.new(
        body: "data: #{JSON.generate(event)}\n\n",
        content_type: 'text/event-stream'
      )
    end
    app
  end

  describe 'basic SSE parsing' do
    it 'parses simple SSE events' do
      app = create_simple_sse_app
      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/events')

      events = stream.events
      expect(events.length).to eq(3)
      expect(events[0].data).to eq('first event')
      expect(events[1].data).to eq('second event')
      expect(events[2].data).to eq('third event')

      client.close
    end

    it 'gets raw body' do
      app = create_simple_sse_app
      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/events')

      body = stream.body
      expect(body).to include('first event')
      expect(body).to include('second event')
      expect(body).to include('third event')

      client.close
    end
  end

  describe 'JSON SSE events' do
    it 'parses events as JSON' do
      app = create_json_sse_app
      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/json-events')

      events = stream.events_as_json
      expect(events.length).to eq(3)

      expect(events[0]['type']).to eq('greeting')
      expect(events[0]['message']).to eq('Hello')

      expect(events[1]['type']).to eq('update')
      expect(events[1]['value']).to eq(42)

      expect(events[2]['type']).to eq('farewell')
      expect(events[2]['message']).to eq('Goodbye')

      client.close
    end

    it 'parses individual events as JSON' do
      app = create_json_sse_app
      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/json-events')

      events = stream.events
      expect(events.length).to eq(3)

      json_data = events[0].as_json
      expect(json_data['type']).to eq('greeting')
      expect(json_data['message']).to eq('Hello')

      client.close
    end
  end

  describe 'complex JSON structures' do
    it 'handles nested JSON in SSE events' do
      app = create_complex_json_app
      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/complex')

      events = stream.events_as_json
      expect(events.length).to eq(1)

      event = events[0]
      expect(event['user']['id']).to eq(123)
      expect(event['user']['name']).to eq('Alice')
      expect(event['user']['tags']).to eq(%w[admin user])
      expect(event['nested']['deep']['value']).to eq('found')

      client.close
    end
  end

  describe 'empty and malformed events' do
    it 'handles empty data lines' do
      app = Spikard::App.new
      app.get('/empty', handler_name: 'sse_empty') do |_req|
        # Empty data lines should be filtered per SSE spec
        Spikard::Response.new(
          body: "data: \n\ndata: real event\n\ndata: \n\n",
          content_type: 'text/event-stream'
        )
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/empty')

      events = stream.events
      # Only non-empty events should be returned
      expect(events.length).to eq(1)
      expect(events[0].data).to eq('real event')

      client.close
    end

    it 'handles malformed JSON gracefully' do
      app = Spikard::App.new
      app.get('/malformed', handler_name: 'sse_malformed') do |_req|
        Spikard::Response.new(
          body: "data: {invalid json\n\ndata: {\"valid\": true}\n\n",
          content_type: 'text/event-stream'
        )
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/malformed')

      # Should return all events, even if some have invalid JSON
      events = stream.events
      expect(events.length).to eq(2)

      # First event has invalid JSON - as_json should handle gracefully
      # Second event has valid JSON
      valid_json = events[1].as_json
      expect(valid_json['valid']).to eq(true)

      client.close
    end
  end

  describe 'special characters' do
    it 'handles special characters in event data' do
      app = Spikard::App.new
      app.get('/special', handler_name: 'sse_special_chars') do |_req|
        Spikard::Response.new(
          body: "data: Line 1\\nLine 2\\tTabbed\n\n",
          content_type: 'text/event-stream'
        )
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/special')

      events = stream.events
      expect(events.length).to eq(1)
      # Note: the actual data will have escaped characters as-is
      expect(events[0].data).to include('Line 1')

      client.close
    end
  end

  describe 'event order' do
    it 'preserves event order' do
      app = Spikard::App.new
      app.get('/ordered', handler_name: 'sse_ordered') do |_req|
        events = (1..10).map { |i| "data: event #{i}\n\n" }.join
        Spikard::Response.new(body: events, content_type: 'text/event-stream')
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/ordered')

      events = stream.events
      expect(events.length).to eq(10)
      events.each_with_index do |event, i|
        expect(event.data).to eq("event #{i + 1}")
      end

      client.close
    end
  end

  describe 'multiline events' do
    it 'handles multiline event data' do
      app = Spikard::App.new
      app.get('/multiline', handler_name: 'sse_multiline') do |_req|
        # Multiline events use multiple data: lines
        Spikard::Response.new(
          body: "data: line 1\ndata: line 2\ndata: line 3\n\n",
          content_type: 'text/event-stream'
        )
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/multiline')

      events = stream.events
      expect(events.length).to eq(1)
      # Multiline data is joined with newlines
      expect(events[0].data).to eq("line 1\nline 2\nline 3")

      client.close
    end
  end

  describe 'unicode content' do
    it 'handles unicode in event data' do
      app = Spikard::App.new
      app.get('/unicode', handler_name: 'sse_unicode') do |_req|
        Spikard::Response.new(
          body: "data: 你好世界 🚀\n\n",
          content_type: 'text/event-stream'
        )
      end

      client = Spikard::Testing.create_test_client(app)
      stream = client.sse('/unicode')

      events = stream.events
      expect(events.length).to eq(1)
      expect(events[0].data).to eq('你好世界 🚀')

      client.close
    end
  end
end
