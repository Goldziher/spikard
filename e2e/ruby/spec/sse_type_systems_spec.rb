# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe 'SSE Type Systems' do
  def extract_event_payloads(body)
    body.gsub("\r\n", "\n").split("\n\n").filter_map do |chunk|
      chunk
        .lines
        .find { |line| line.start_with?('data:') }
        &.split('data:', 2)
        &.last
        &.strip
    end
  end

  # 1. Plain JSON Schema
  describe 'Plain JSON Schema' do
    let(:app) do
      Spikard::App.new.tap do |app|
        event_schema = {
          'type' => 'object',
          'properties' => {
            'status' => { 'type' => 'string' },
            'message' => { 'type' => 'string' },
            'timestamp' => { 'type' => 'integer' }
          },
          'required' => %w[status message timestamp]
        }

        app.sse('/status/json-schema') do
          producer = Class.new(Spikard::SseEventProducer) do
            def next_event
              @count ||= 0
              return nil if @count >= 3

              event = Spikard::SseEvent.new(
                event_type: 'status',
                data: {
                  'status' => 'ok',
                  'message' => "Update #{@count}",
                  'timestamp' => 1234567890 + @count
                }
              )
              @count += 1
              event
            end
          end.new

          # Store schema as instance variable
          producer.instance_variable_set(:@_event_schema, event_schema)
          producer
        end
      end
    end

    it 'validates SSE events against JSON Schema' do
      client = Spikard::TestClient.new(app)
      response = client.get('/status/json-schema')

      expect(response.status).to eq(200)

      events = extract_event_payloads(response.body)

      expect(events.length).to eq(3)

      events.each_with_index do |event_json, i|
        event = JSON.parse(event_json)
        expect(event['status']).to eq('ok')
        expect(event['message']).to eq("Update #{i}")
        expect(event['timestamp']).to eq(1234567890 + i)
      end
    end
  end

  # 2. Dry::Schema (if available)
  describe 'Dry::Schema', skip: !defined?(Dry::Schema) do
    before(:all) do
      require 'dry-schema'
      Dry::Schema.load_extensions(:json_schema)
    end

    let(:status_event_schema) do
      Dry::Schema.JSON do
        required(:status).filled(:str?)
        required(:message).filled(:str?)
        required(:timestamp).filled(:int?)
      end
    end

    let(:app) do
      schema = status_event_schema
      Spikard::App.new.tap do |app|
        app.sse('/status/dry-schema') do
          producer = Class.new(Spikard::SseEventProducer) do
            def next_event
              @count ||= 0
              return nil if @count >= 3

              event = Spikard::SseEvent.new(
                event_type: 'status',
                data: {
                  'status' => 'ok',
                  'message' => "Update #{@count}",
                  'timestamp' => 1234567890 + @count
                }
              )
              @count += 1
              event
            end
          end.new

          # Extract JSON Schema from Dry::Schema
          json_schema = Spikard::Schema.extract_json_schema(schema)
          producer.instance_variable_set(:@_event_schema, json_schema)
          producer
        end
      end
    end

    it 'validates SSE events using Dry::Schema' do
      client = Spikard::TestClient.new(app)
      response = client.get('/status/dry-schema')

      expect(response.status).to eq(200)

      events = extract_event_payloads(response.body)

      expect(events.length).to eq(3)

      events.each_with_index do |event_json, i|
        event = JSON.parse(event_json)
        expect(event['status']).to eq('ok')
        expect(event['message']).to eq("Update #{i}")
        expect(event['timestamp']).to eq(1234567890 + i)
      end
    end
  end

  # 3. Dry::Struct (if available)
  describe 'Dry::Struct', skip: !defined?(Dry::Struct) do
    before(:all) do
      require 'dry-struct'
      require 'dry-types'

      module Types
        include Dry.Types()
      end

      # Define struct globally for tests
      ::StatusEvent = Class.new(Dry::Struct) do
        attribute :status, Types::String
        attribute :message, Types::String
        attribute :timestamp, Types::Integer
      end
    end

    let(:app) do
      Spikard::App.new.tap do |app|
        app.sse('/status/dry-struct') do
          producer = Class.new(Spikard::SseEventProducer) do
            def next_event
              @count ||= 0
              return nil if @count >= 3

              event = Spikard::SseEvent.new(
                event_type: 'status',
                data: {
                  'status' => 'ok',
                  'message' => "Update #{@count}",
                  'timestamp' => 1234567890 + @count
                }
              )
              @count += 1
              event
            end
          end.new

          # Extract JSON Schema from Dry::Struct
          json_schema = Spikard::Schema.extract_json_schema(::StatusEvent)
          producer.instance_variable_set(:@_event_schema, json_schema)
          producer
        end
      end
    end

    it 'validates SSE events using Dry::Struct' do
      client = Spikard::TestClient.new(app)
      response = client.get('/status/dry-struct')

      expect(response.status).to eq(200)

      events = extract_event_payloads(response.body)

      expect(events.length).to eq(3)

      events.each_with_index do |event_json, i|
        event = JSON.parse(event_json)
        expect(event['status']).to eq('ok')
        expect(event['message']).to eq("Update #{i}")
        expect(event['timestamp']).to eq(1234567890 + i)
      end
    end
  end
end
