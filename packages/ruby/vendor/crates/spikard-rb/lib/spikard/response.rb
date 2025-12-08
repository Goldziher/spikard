# frozen_string_literal: true

module Spikard
  # Wrapper for HTTP response from TestClient
  class Response
    attr_reader :status_code, :headers_hash, :body_bytes

    def initialize(native_response)
      @status_code = native_response[:status_code]
      @headers_hash = native_response[:headers] || {}
      @body_bytes = native_response[:body] || ''
    end

    def headers
      @headers_hash
    end

    def text
      @body_bytes
    end

    def json
      JSON.parse(@body_bytes) if @body_bytes && !@body_bytes.empty?
    end

    def bytes
      @body_bytes.bytes
    end
  end
end
