# frozen_string_literal: true

require 'rspec'
require 'json'
require_relative '../app'

RSpec.describe 'Spikard Ruby Test App' do
  let(:app) { SpikardTestApp.create_app }
  let(:client) { Spikard::TestClient.new(app) }

  before(:all) do
    @app = SpikardTestApp.create_app
    @client = Spikard::TestClient.new(@app)
  end

  after(:all) do
    @client&.close
  end

  it 'uses the correct package version' do
    gemfile_lock = File.read(File.join(__dir__, '..', 'Gemfile.lock'))
    expect(gemfile_lock).to include('spikard (0.10.1)')
  end

  it 'responds to health check' do
    response = @client.get('/health')
    expect(response.status).to eq(200)
    data = response.json
    expect(data).to eq({ 'status' => 'ok' })
  end

  it 'handles query parameters' do
    response = @client.get('/query', query: { name: 'Alice', age: '30' })
    expect(response.status).to eq(200)
    data = response.json
    expect(data).to eq({ 'name' => 'Alice', 'age' => 30 })
  end

  it 'echoes JSON requests' do
    payload = { message: 'Hello from Ruby!' }
    response = @client.post('/echo', json: payload)
    expect(response.status).to eq(200)
    data = response.json
    expect(data['received']).to eq(payload.transform_keys(&:to_s))
    expect(data['method']).to eq('POST')
  end

  it 'extracts path parameters' do
    response = @client.get('/users/42')
    expect(response.status).to eq(200)
    data = response.json
    expect(data['userId']).to eq('42')
    expect(data['type']).to eq('String')
  end

  it 'handles PUT method' do
    payload = { name: 'Widget' }
    response = @client.put('/items/1', json: payload)
    expect(response.status).to eq(200)
    data = response.json
    expect(data['itemId']).to eq('1')
    expect(data['updated']).to eq(payload.transform_keys(&:to_s))
    expect(data['method']).to eq('PUT')
  end

  it 'handles DELETE method' do
    response = @client.delete('/items/1')
    expect(response.status).to eq(200)
    data = response.json
    expect(data['itemId']).to eq('1')
    expect(data['deleted']).to eq(true)
    expect(data['method']).to eq('DELETE')
  end

  it 'handles PATCH method' do
    payload = { name: 'Updated' }
    response = @client.patch('/items/1', json: payload)
    expect(response.status).to eq(200)
    data = response.json
    expect(data['itemId']).to eq('1')
    expect(data['patched']).to eq(payload.transform_keys(&:to_s))
    expect(data['method']).to eq('PATCH')
  end

  it 'extracts custom headers' do
    response = @client.get('/headers', headers: { 'X-Custom-Header' => 'test-value' })
    expect(response.status).to eq(200)
    data = response.json
    expect(data['x-custom-header']).to eq('test-value')
  end

  it 'extracts cookies' do
    response = @client.get('/cookies', headers: { 'Cookie' => 'session=abc123' })
    expect(response.status).to eq(200)
    data = response.json
    expect(data['session']).to eq('abc123')
  end

  it 'returns 404 for unknown routes' do
    response = @client.get('/nonexistent')
    expect(response.status).to eq(404)
  end

  it 'returns 500 for error handler' do
    response = @client.get('/error')
    expect(response.status).to eq(500)
  end

  it 'has importable public API' do
    expect(defined?(Spikard::App)).to be_truthy
    expect(defined?(Spikard::TestClient)).to be_truthy
  end
end
