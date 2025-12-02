# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::Response do
  describe '#headers=' do
    it 'normalises header keys and values to strings' do
      response = described_class.new(headers: { 'X-Number' => 1, :symbol => :value })

      expect(response.headers).to eq('x-number' => '1', 'symbol' => 'value')
    end
  end

  describe '#set_cookie' do
    it 'builds a full Set-Cookie header with supplied options' do
      response = described_class.new
      response.set_cookie('session', 'abc123', domain: 'example.com', max_age: 60, httponly: true, secure: true)

      expect(response.headers['set-cookie']).to eq(
        'session=abc123; Max-Age=60; Domain=example.com; Path=/; Secure; HttpOnly'
      )
    end
  end
end

RSpec.describe Spikard::Testing::Response do
  describe '#json' do
    it 'parses JSON bodies while leaving nil body untouched' do
      response = described_class.new(status_code: 200, headers: {}, body: '{"hello":"world"}')
      expect(response.json).to eq('hello' => 'world')

      empty_response = described_class.new(status_code: 204, headers: {}, body: nil)
      expect(empty_response.json).to be_nil
    end
  end
end
