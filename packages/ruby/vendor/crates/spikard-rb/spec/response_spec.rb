# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path('../lib', __dir__)

require 'spikard'
require 'rspec'

RSpec.describe Spikard::Response do
  describe '#initialize' do
    it 'initializes from native response hash' do
      native_response = {
        status_code: 200,
        headers: { 'Content-Type' => 'application/json' },
        body: '{"message":"test"}'
      }

      response = Spikard::Response.new(native_response)

      expect(response.status_code).to eq(200)
      expect(response.headers).to eq({ 'Content-Type' => 'application/json' })
      expect(response.body_bytes).to eq('{"message":"test"}')
    end

    it 'handles missing headers' do
      native_response = {
        status_code: 404,
        body: 'Not found'
      }

      response = Spikard::Response.new(native_response)

      expect(response.status_code).to eq(404)
      expect(response.headers).to eq({})
      expect(response.body_bytes).to eq('Not found')
    end

    it 'handles missing body' do
      native_response = {
        status_code: 204,
        headers: {}
      }

      response = Spikard::Response.new(native_response)

      expect(response.status_code).to eq(204)
      expect(response.body_bytes).to eq('')
    end
  end

  describe '#headers' do
    it 'returns headers hash' do
      native_response = {
        status_code: 200,
        headers: { 'X-Custom' => 'value' },
        body: ''
      }

      response = Spikard::Response.new(native_response)

      expect(response.headers).to be_a(Hash)
      expect(response.headers['X-Custom']).to eq('value')
    end
  end

  describe '#text' do
    it 'returns body as text' do
      native_response = {
        status_code: 200,
        headers: {},
        body: 'Hello, world!'
      }

      response = Spikard::Response.new(native_response)

      expect(response.text).to eq('Hello, world!')
    end
  end

  describe '#json' do
    it 'parses JSON body' do
      native_response = {
        status_code: 200,
        headers: {},
        body: '{"message":"test","count":42}'
      }

      response = Spikard::Response.new(native_response)

      json = response.json
      expect(json).to be_a(Hash)
      expect(json['message']).to eq('test')
      expect(json['count']).to eq(42)
    end

    it 'returns nil for empty body' do
      native_response = {
        status_code: 204,
        headers: {},
        body: ''
      }

      response = Spikard::Response.new(native_response)

      expect(response.json).to be_nil
    end

    it 'handles arrays' do
      native_response = {
        status_code: 200,
        headers: {},
        body: '[1,2,3]'
      }

      response = Spikard::Response.new(native_response)

      json = response.json
      expect(json).to be_a(Array)
      expect(json).to eq([1, 2, 3])
    end
  end

  describe '#bytes' do
    it 'returns body as byte array' do
      native_response = {
        status_code: 200,
        headers: {},
        body: 'ABC'
      }

      response = Spikard::Response.new(native_response)

      bytes = response.bytes
      expect(bytes).to be_a(Array)
      expect(bytes).to eq([65, 66, 67]) # ASCII codes for 'A', 'B', 'C'
    end
  end
end
