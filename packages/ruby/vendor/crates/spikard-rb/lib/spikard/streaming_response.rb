# frozen_string_literal: true

module Spikard
  # Lightweight wrapper for streaming handler responses.
  class StreamingResponse
    attr_reader :stream, :status_code, :headers

    def initialize(stream, status_code: 200, headers: nil)
      unless stream.respond_to?(:next) || stream.respond_to?(:each)
        raise ArgumentError, 'StreamingResponse requires an Enumerator or object responding to #next'
      end

      @stream = stream.respond_to?(:to_enum) ? stream.to_enum : stream
      @status_code = Integer(status_code || 200)
      header_hash = headers || {}
      @headers = header_hash.each_with_object({}) do |(key, value), memo|
        memo[String(key)] = String(value)
      end
    end
  end
end
