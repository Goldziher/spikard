# frozen_string_literal: true

require_relative 'upload_file'

module Spikard
  # Type conversion utilities for handler parameters
  #
  # This module handles converting validated JSON data from Rust into Ruby types,
  # particularly for UploadFile instances.
  module Converters
    module_function

    # Check if a value looks like file metadata from Rust
    #
    # @param value [Object] Value to check
    # @return [Boolean]
    def file_metadata?(value)
      value.is_a?(Hash) && value.key?('filename') && value.key?('content')
    end

    # Convert file metadata hash to UploadFile instance
    #
    # @param file_data [Hash] File metadata from Rust (filename, content, size, content_type)
    # @return [UploadFile] UploadFile instance
    def convert_file_metadata_to_upload_file(file_data)
      UploadFile.new(
        file_data['filename'],
        file_data['content'],
        content_type: file_data['content_type'],
        size: file_data['size'],
        headers: file_data['headers'],
        content_encoding: file_data['content_encoding']
      )
    end

    # Process handler parameters, converting file metadata to UploadFile instances
    #
    # This method recursively processes the body parameter, looking for file metadata
    # structures and converting them to UploadFile instances.
    #
    # @param value [Object] The value to process (can be Hash, Array, or primitive)
    # @return [Object] Processed value with UploadFile instances
    def process_upload_file_fields(value)
      # Handle nil
      return value if value.nil?

      # Handle primitives (String, Numeric, Boolean)
      return value unless value.is_a?(Hash) || value.is_a?(Array)

      # Handle arrays - recursively process each element
      if value.is_a?(Array)
        return value.map do |item|
          # Check if this array item is file metadata
          if file_metadata?(item)
            convert_file_metadata_to_upload_file(item)
          else
            # Recursively process nested arrays/hashes
            process_upload_file_fields(item)
          end
        end
      end

      # Handle hashes - check if it's file metadata first
      return convert_file_metadata_to_upload_file(value) if file_metadata?(value)

      # Otherwise, recursively process hash values
      value.transform_values { |v| process_upload_file_fields(v) }
    end

    # Process handler body parameter, handling UploadFile conversion
    #
    # This is the main entry point for converting Rust-provided request data
    # into Ruby types. It handles:
    # - Single UploadFile
    # - Arrays of UploadFile
    # - Hashes with UploadFile fields
    # - Nested structures
    #
    # @param body [Object] The body parameter from Rust (already JSON-parsed)
    # @return [Object] Processed body with UploadFile instances
    def convert_handler_body(body)
      process_upload_file_fields(body)
    end
  end
end
