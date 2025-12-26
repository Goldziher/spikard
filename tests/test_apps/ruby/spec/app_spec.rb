# frozen_string_literal: true

require 'rspec'
require 'net/http'
require 'json'
require_relative '../app'

RSpec.describe 'Spikard Ruby Test App' do
  let(:server) { SpikardTestApp.create_app }
  let(:base_url) do
    address = server.address
    "http://#{address[:host]}:#{address[:port]}"
  end

  before(:all) do
    @server = SpikardTestApp.create_app
    @server.start
    address = @server.address
    @base_url = "http://#{address[:host]}:#{address[:port]}"
  end

  after(:all) do
    @server&.stop
  end

  it 'uses the correct package version' do
    gemfile_lock = File.read(File.join(__dir__, '..', 'Gemfile.lock'))
    expect(gemfile_lock).to include('spikard (0.6.0)')
  end

  it 'responds to health check' do
    uri = URI("#{@base_url}/health")
    response = Net::HTTP.get_response(uri)
    expect(response.code).to eq('200')
    data = JSON.parse(response.body)
    expect(data).to eq({ 'status' => 'ok' })
  end

  it 'handles query parameters' do
    uri = URI("#{@base_url}/query")
    uri.query = URI.encode_www_form({ name: 'Alice', age: '30' })
    response = Net::HTTP.get_response(uri)
    expect(response.code).to eq('200')
    data = JSON.parse(response.body)
    expect(data).to eq({ 'name' => 'Alice', 'age' => 30 })
  end

  it 'echoes JSON requests' do
    uri = URI("#{@base_url}/echo")
    payload = { message: 'Hello from Ruby!' }
    request = Net::HTTP::Post.new(uri)
    request['Content-Type'] = 'application/json'
    request.body = JSON.generate(payload)

    response = Net::HTTP.start(uri.hostname, uri.port) do |http|
      http.request(request)
    end

    expect(response.code).to eq('200')
    data = JSON.parse(response.body)
    expect(data['received']).to eq(payload.transform_keys(&:to_s))
    expect(data['method']).to eq('POST')
  end

  it 'extracts path parameters' do
    uri = URI("#{@base_url}/users/42")
    response = Net::HTTP.get_response(uri)
    expect(response.code).to eq('200')
    data = JSON.parse(response.body)
    expect(data['userId']).to eq('42')
    expect(data['type']).to eq('String')
  end
end
