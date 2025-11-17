# frozen_string_literal: true

module Spikard
  # Response object returned from route handlers.
  # Mirrors the Python/Node response helpers so the native layer
  # can extract status, headers, and JSON-serialisable content.
  class Response
    attr_accessor :content
    attr_reader :status_code, :headers

    def initialize(content: nil, body: nil, status_code: 200, headers: nil, content_type: nil)
      @content = content.nil? ? body : content
      self.status_code = status_code
      self.headers = headers
      set_header('content-type', content_type) if content_type
    end

    def status
      @status_code
    end

    def status_code=(value)
      @status_code = Integer(value)
    rescue ArgumentError, TypeError
      raise ArgumentError, 'status_code must be an integer'
    end

    def headers=(value)
      @headers = normalize_headers(value)
    end

    def set_header(name, value)
      @headers[name.to_s] = value.to_s
    end

    def set_cookie(name, value, **options)
      raise ArgumentError, 'cookie name required' if name.nil? || name.empty?

      header_value = ["#{name}=#{value}", *cookie_parts(options)].join('; ')
      set_header('set-cookie', header_value)
    end

    private

    def cookie_parts(options)
      [
        options[:max_age] && "Max-Age=#{Integer(options[:max_age])}",
        options[:domain] && "Domain=#{options[:domain]}",
        "Path=#{options.fetch(:path, '/') || '/'}",
        options[:secure] ? 'Secure' : nil,
        options[:httponly] ? 'HttpOnly' : nil,
        options[:samesite] && "SameSite=#{options[:samesite]}"
      ].compact
    end

    def normalize_headers(value)
      case value
      when nil
        {}
      when Hash
        value.each_with_object({}) do |(key, val), acc|
          acc[key.to_s] = val.to_s
        end
      else
        raise ArgumentError, 'headers must be a Hash'
      end
    end
  end

  module Testing
    # Lightweight wrapper around native response hashes.
    class Response
      attr_reader :status_code, :headers, :body

      def initialize(payload)
        @status_code = payload[:status_code]
        @headers = payload[:headers] || {}
        @body = payload[:body]
        @body_text = payload[:body_text]
      end

      def status
        @status_code
      end

      def body_bytes
        @body || ''.b
      end

      def body_text
        @body_text || @body&.dup&.force_encoding(Encoding::UTF_8)
      end

      def text
        body_text
      end

      def json
        return nil if @body.nil? || @body.empty?

        JSON.parse(@body)
      end

      def bytes
        body_bytes.bytes
      end
    end
  end
end
