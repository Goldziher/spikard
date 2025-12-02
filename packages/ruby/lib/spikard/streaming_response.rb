# frozen_string_literal: true

module Spikard
  # Represents a streaming HTTP response made of chunks produced lazily.
  class StreamingResponse
    attr_reader :stream, :status_code, :headers, :native_response

    def initialize(stream, status_code: 200, headers: nil)
      unless stream.respond_to?(:next) || stream.respond_to?(:each)
        raise ArgumentError, 'StreamingResponse requires an object responding to #next or #each'
      end

      @stream = stream.respond_to?(:to_enum) ? stream.to_enum : stream
      @status_code = Integer(status_code || 200)
      header_hash = headers || {}
      @headers = header_hash.each_with_object({}) do |(key, value), memo|
        memo[String(key)] = String(value)
      end

      return unless defined?(Spikard::Native) && Spikard::Native.respond_to?(:build_streaming_response)

      @native_response = Spikard::Native.build_streaming_response(@stream, @status_code, @headers)
    end

    def to_native_response
      @native_response
    end
  end
end
