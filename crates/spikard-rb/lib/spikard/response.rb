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

    # Extract GraphQL data from response
    # @return [Hash, nil] The data field from GraphQL response
    def graphql_data
      return nil if @body_bytes.nil? || @body_bytes.empty?

      parsed = JSON.parse(@body_bytes)
      parsed['data']
    rescue JSON::ParserError => e
      raise "Failed to parse GraphQL response: #{e.message}"
    end

    # Extract GraphQL errors from response
    # @return [Array<Hash>] Array of GraphQL error objects
    def graphql_errors
      return [] if @body_bytes.nil? || @body_bytes.empty?

      parsed = JSON.parse(@body_bytes)
      parsed.fetch('errors', [])
    rescue JSON::ParserError => e
      raise "Failed to parse GraphQL response: #{e.message}"
    end
  end
end
