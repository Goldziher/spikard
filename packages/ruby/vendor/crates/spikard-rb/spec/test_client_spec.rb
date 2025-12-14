# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path('../lib', __dir__)

require 'spikard'
require 'rspec'

RSpec.describe Spikard::TestClient do
  let(:routes) do
    [
      {
        method: 'GET',
        path: '/',
        handler_name: 'root',
        is_async: false
      },
      {
        method: 'POST',
        path: '/echo',
        handler_name: 'echo',
        is_async: false
      },
      {
        method: 'GET',
        path: '/json',
        handler_name: 'json',
        is_async: false
      }
    ]
  end

  let(:handlers) do
    {
      'root' => ->(_request) { { message: 'Hello, world!' } },
      'echo' => ->(request) { request['body'] },
      'json' => ->(_request) { { data: [1, 2, 3], nested: { key: 'value' } } }
    }
  end

  let(:app) { Spikard.create_app(routes, handlers) }
  let(:client) { Spikard.create_test_client(app) }

  describe '#get' do
    it 'makes a GET request and returns Response' do
      response = client.get('/')
      expect(response).to be_a(Spikard::Response)
      expect(response.status_code).to eq(200)
      expect(response.json).to eq({ 'message' => 'Hello, world!' })
    end

    it 'handles custom headers' do
      response = client.get('/', { 'X-Custom-Header' => 'test-value' })
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#post' do
    it 'makes a POST request with body' do
      body = { test: 'data' }.to_json
      response = client.post('/echo', {}, body)
      expect(response).to be_a(Spikard::Response)
    end

    it 'handles POST with headers and body' do
      body = { test: 'data' }.to_json
      headers = { 'Content-Type' => 'application/json' }
      response = client.post('/echo', headers, body)
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#put' do
    it 'makes a PUT request' do
      response = client.put('/echo', {}, '{"updated": true}')
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#delete' do
    it 'makes a DELETE request' do
      response = client.delete('/')
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#patch' do
    it 'makes a PATCH request' do
      response = client.patch('/echo', {}, '{"patched": true}')
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#head' do
    it 'makes a HEAD request' do
      response = client.head('/')
      expect(response).to be_a(Spikard::Response)
    end
  end

  describe '#options' do
    it 'makes an OPTIONS request' do
      response = client.options('/')
      expect(response).to be_a(Spikard::Response)
    end
  end
end
