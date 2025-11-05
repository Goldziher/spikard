# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Spikard::Testing::TestClient integration' do
  let(:app) do
    Spikard::App.new.tap do |spikard_app|
      spikard_app.get '/hello/:name', handler_name: 'hello' do |request|
        {
          'greeting' => "hello #{request[:params]['name']}",
          'query' => request[:query],
        }
      end

      spikard_app.post '/echo', handler_name: 'echo' do |request|
        body = request[:body] || {}
        Spikard::Response.new(
          content: body.merge('ack' => true),
          status_code: 201,
          headers: { 'X-Test' => '1' },
        )
      end
    end
  end

  let(:client) { Spikard::Testing.create_test_client(app) }

  it 'returns JSON responses for GET requests' do
    response = client.get('/hello/Alice', query: { lang: 'en' })

    expect(response.status_code).to eq(200)
    expect(response.json).to eq(
      'greeting' => 'hello Alice',
      'query' => { 'lang' => 'en' },
    )
  end

  it 'supports custom responses with status and headers' do
    response = client.post('/echo', json: { 'message' => 'hi' })

    expect(response.status_code).to eq(201)
    expect(response.headers['x-test']).to eq('1')
    expect(response.json).to eq('message' => 'hi', 'ack' => true)
  end
end
