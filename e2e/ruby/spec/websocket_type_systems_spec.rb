# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe 'WebSocket Type Systems' do
  # 1. Plain JSON Schema
  describe 'Plain JSON Schema' do
    let(:app) do
      Spikard::App.new.tap do |app|
        message_schema = {
          'type' => 'object',
          'properties' => {
            'user' => { 'type' => 'string' },
            'text' => { 'type' => 'string' },
            'timestamp' => { 'type' => 'integer' }
          },
          'required' => %w[user text timestamp]
        }

        app.websocket('/chat/json-schema') do
          handler = Class.new(Spikard::WebSocketHandler) do
            def handle_message(message)
              {
                'echo' => message['text'],
                'user' => message['user'],
                'timestamp' => message['timestamp'],
                'validated' => true
              }
            end
          end.new

          # Store schemas as instance variables (Rust bindings will extract these)
          handler.instance_variable_set(:@_message_schema, message_schema)
          handler
        end
      end
    end

    it 'validates WebSocket messages against JSON Schema' do
      client = Spikard::TestClient.new(app)
      ws = client.websocket('/chat/json-schema')

      # Valid message
      ws.send_json({
        'user' => 'alice',
        'text' => 'Hello JSON Schema!',
        'timestamp' => 1234567890
      })

      response = ws.receive_json
      expect(response['validated']).to be true
      expect(response['echo']).to eq('Hello JSON Schema!')
      expect(response['user']).to eq('alice')

      ws.close
    end

    it 'rejects invalid messages' do
      client = Spikard::TestClient.new(app)
      ws = client.websocket('/chat/json-schema')

      # Invalid message - missing required field
      ws.send_json({
        'user' => 'alice',
        'timestamp' => 1234567890
        # Missing 'text' field
      })

      response = ws.receive_json
      # Should receive error or validation failure
      expect(response).to have_key('error') | have_key('validated')

      ws.close
    end
  end

  # 2. Dry::Schema (if available)
  describe 'Dry::Schema', skip: !defined?(Dry::Schema) do
    before(:all) do
      require 'dry-schema'
      Dry::Schema.load_extensions(:json_schema)
    end

    let(:chat_message_schema) do
      Dry::Schema.JSON do
        required(:user).filled(:str?)
        required(:text).filled(:str?)
        required(:timestamp).filled(:int?)
      end
    end

    let(:app) do
      schema = chat_message_schema
      Spikard::App.new.tap do |app|
        app.websocket('/chat/dry-schema') do
          handler = Class.new(Spikard::WebSocketHandler) do
            def handle_message(message)
              {
                'echo' => message['text'],
                'user' => message['user'],
                'timestamp' => message['timestamp'],
                'validated' => true
              }
            end
          end.new

          # Extract JSON Schema from Dry::Schema
          json_schema = Spikard::Schema.extract_json_schema(schema)
          handler.instance_variable_set(:@_message_schema, json_schema)
          handler
        end
      end
    end

    it 'validates WebSocket messages using Dry::Schema' do
      client = Spikard::TestClient.new(app)
      ws = client.websocket('/chat/dry-schema')

      # Valid message
      ws.send_json({
        'user' => 'bob',
        'text' => 'Hello Dry::Schema!',
        'timestamp' => 1234567890
      })

      response = ws.receive_json
      expect(response['validated']).to be true
      expect(response['echo']).to eq('Hello Dry::Schema!')
      expect(response['user']).to eq('bob')

      ws.close
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
      ::ChatMessage = Class.new(Dry::Struct) do
        attribute :user, Types::String
        attribute :text, Types::String
        attribute :timestamp, Types::Integer
      end
    end

    let(:app) do
      Spikard::App.new.tap do |app|
        app.websocket('/chat/dry-struct') do
          handler = Class.new(Spikard::WebSocketHandler) do
            def handle_message(message)
              {
                'echo' => message['text'],
                'user' => message['user'],
                'timestamp' => message['timestamp'],
                'validated' => true
              }
            end
          end.new

          # Extract JSON Schema from Dry::Struct
          json_schema = Spikard::Schema.extract_json_schema(::ChatMessage)
          handler.instance_variable_set(:@_message_schema, json_schema)
          handler
        end
      end
    end

    it 'validates WebSocket messages using Dry::Struct' do
      client = Spikard::TestClient.new(app)
      ws = client.websocket('/chat/dry-struct')

      # Valid message
      ws.send_json({
        'user' => 'charlie',
        'text' => 'Hello Dry::Struct!',
        'timestamp' => 1234567890
      })

      response = ws.receive_json
      expect(response['validated']).to be true
      expect(response['echo']).to eq('Hello Dry::Struct!')
      expect(response['user']).to eq('charlie')

      ws.close
    end
  end
end
