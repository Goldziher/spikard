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
    expect(gemfile_lock).to include('spikard (0.7.4)')
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
end
