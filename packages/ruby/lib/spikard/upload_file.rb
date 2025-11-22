# frozen_string_literal: true

require 'stringio'
require 'base64'

module Spikard
  # File upload handling for multipart/form-data requests
  #
  # This class provides an interface for handling file uploads,
  # designed to be compatible with Rails patterns while optimized
  # for Spikard's Rust-backed request processing.
  #
  # @example
  #   app.post('/upload') do |body|
  #     file = body[:file]  # UploadFile instance
  #     content = file.read
  #     {
  #       filename: file.filename,
  #       size: file.size,
  #       content_type: file.content_type,
  #       description: body[:description]
  #     }
  #   end
  class UploadFile
    # @return [String] Original filename from the client
    attr_reader :filename

    # @return [String] MIME type of the uploaded file
    attr_reader :content_type

    # @return [Integer] Size of the file in bytes
    attr_reader :size

    # @return [Hash<String, String>] Additional headers associated with this file field
    attr_reader :headers

    # Create a new UploadFile instance
    #
    # @param filename [String] Original filename from the client
    # @param content [String] File contents (may be base64 encoded)
    # @param content_type [String, nil] MIME type (defaults to "application/octet-stream")
    # @param size [Integer, nil] File size in bytes (computed from content if not provided)
    # @param headers [Hash<String, String>, nil] Additional headers from the multipart field
    # @param content_encoding [String, nil] Encoding type (e.g., "base64")
    def initialize(filename, content, content_type: nil, size: nil, headers: nil, content_encoding: nil)
      @filename = filename
      @content_type = content_type || 'application/octet-stream'
      @headers = headers || {}

      # Decode content if base64 encoded
      @content = if content_encoding == 'base64' || base64_encoded?(content)
                   Base64.decode64(content)
                 else
                   content
                 end

      @size = size || @content.bytesize
      @io = StringIO.new(@content)
    end

    # Read file contents
    #
    # @param size [Integer, nil] Number of bytes to read (nil for all remaining)
    # @return [String] File contents
    def read(size = nil)
      @io.read(size)
    end

    # Read file contents as text
    #
    # @param encoding [String] Character encoding (defaults to UTF-8)
    # @return [String] File contents as text
    def text(encoding: 'UTF-8')
      @content.force_encoding(encoding)
    end

    # Seek to a specific position in the file
    #
    # @param offset [Integer] Byte offset
    # @param whence [Integer] Position reference (IO::SEEK_SET, IO::SEEK_CUR, IO::SEEK_END)
    # @return [Integer] New position
    def seek(offset, whence = IO::SEEK_SET)
      @io.seek(offset, whence)
    end

    # Get current position in the file
    #
    # @return [Integer] Current byte offset
    def tell
      @io.tell
    end
    alias pos tell

    # Rewind to the beginning of the file
    #
    # @return [Integer] Always returns 0
    def rewind
      @io.rewind
    end

    # Close the file (no-op for StringIO-based implementation)
    #
    # @return [nil]
    def close
      @io.close
    end

    # Check if file is closed
    #
    # @return [Boolean]
    def closed?
      @io.closed?
    end

    # Get the raw content as a string
    #
    # @return [String] Raw file content
    attr_reader :content

    private

    # Check if a string appears to be base64 encoded
    #
    # @param str [String] String to check
    # @return [Boolean]
    def base64_encoded?(str)
      # Simple heuristic: check if string matches base64 pattern
      str.is_a?(String) && str.match?(%r{\A[A-Za-z0-9+/]*={0,2}\z})
    end
  end
end
