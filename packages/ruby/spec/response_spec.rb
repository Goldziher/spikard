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

  describe '#initialize' do
    it 'sets content-type header and normalizes headers when native responds' do
      native = instance_double('NativeResponse', status_code: 201, headers: { 'content-type' => 'application/json' })
      allow(Spikard::Native).to receive(:build_response).and_return(native)

      response = described_class.new(status_code: 202, headers: { 'X-Test' => 'ok' }, content_type: 'application/json')

      expect(response.status).to eq(201)
      expect(response.headers).to eq('content-type' => 'application/json')
      expect(response.to_native_response).to eq(native)
    end
  end

  describe '#status_code=' do
    it 'raises when value cannot be coerced to integer' do
      response = described_class.new

      expect { response.status_code = 'abc' }.to raise_error(ArgumentError)
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

RSpec.describe Spikard::StreamingResponse do
  describe '#initialize' do
    it 'normalizes headers and returns native response data' do
      native = instance_double('NativeStreaming', status_code: 206, headers: { 'x-stream' => 'yes' })
      allow(Spikard::Native).to receive(:build_streaming_response).and_return(native)

      response = described_class.new([1, 2].to_enum, status_code: 200, headers: { foo: :bar })

      expect(response.status_code).to eq(206)
      expect(response.headers).to eq('x-stream' => 'yes')
      expect(response.to_native_response).to eq(native)
    end

    it 'raises when stream does not support enumeration' do
      expect { described_class.new(Object.new) }.to raise_error(ArgumentError)
    end
  end
end
