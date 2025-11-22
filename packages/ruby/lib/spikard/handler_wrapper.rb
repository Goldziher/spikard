# frozen_string_literal: true

require_relative 'converters'

module Spikard
  # Handler wrapper utilities for automatic file metadata conversion
  #
  # Provides ergonomic handler patterns that automatically convert
  # file metadata to UploadFile instances, eliminating boilerplate.
  #
  # @example Basic usage with body only
  #   app.post('/upload', &wrap_body_handler do |body|
  #     {
  #       filename: body[:file].filename,
  #       content: body[:file].read
  #     }
  #   end)
  #
  # @example With all parameters
  #   app.post('/upload', &wrap_handler do |params, query, body|
  #     {
  #       id: params[:id],
  #       search: query[:q],
  #       file: body[:file].filename
  #     }
  #   end)
  module HandlerWrapper
    module_function

    # Wrap a handler that receives only the request body
    #
    # Automatically converts file metadata in the body to UploadFile instances.
    #
    # @yield [body] Handler block that receives converted body
    # @yieldparam body [Hash] Request body with file metadata converted to UploadFile
    # @yieldreturn [Hash, Spikard::Response] Response data or Response object
    # @return [Proc] Wrapped handler proc
    #
    # @example
    #   app.post('/upload', &wrap_body_handler do |body|
    #     { filename: body[:file].filename }
    #   end)
    def wrap_body_handler(&handler)
      raise ArgumentError, 'block required for wrap_body_handler' unless handler

      # Return a proc that matches the signature expected by Spikard::App
      # The actual handler receives path params, query params, and body from Rust
      lambda do |_params, _query, body|
        converted_body = Converters.convert_handler_body(body)
        handler.call(converted_body)
      end
    end

    # Wrap a handler that receives path params, query params, and body
    #
    # Automatically converts file metadata in the body to UploadFile instances.
    #
    # @yield [params, query, body] Handler block that receives all request data
    # @yieldparam params [Hash] Path parameters
    # @yieldparam query [Hash] Query parameters
    # @yieldparam body [Hash] Request body with file metadata converted to UploadFile
    # @yieldreturn [Hash, Spikard::Response] Response data or Response object
    # @return [Proc] Wrapped handler proc
    #
    # @example
    #   app.post('/users/{id}/upload', &wrap_handler do |params, query, body|
    #     {
    #       user_id: params[:id],
    #       description: query[:desc],
    #       file: body[:file].filename
    #     }
    #   end)
    def wrap_handler(&handler)
      raise ArgumentError, 'block required for wrap_handler' unless handler

      lambda do |params, query, body|
        converted_body = Converters.convert_handler_body(body)
        handler.call(params, query, converted_body)
      end
    end

    # Wrap a handler that receives a context hash with all request data
    #
    # Automatically converts file metadata in the body to UploadFile instances.
    # Useful when you want all request data in a single hash.
    #
    # @yield [context] Handler block that receives context hash
    # @yieldparam context [Hash] Request context with:
    #   - :params [Hash] Path parameters
    #   - :query [Hash] Query parameters
    #   - :body [Hash] Request body with file metadata converted to UploadFile
    # @yieldreturn [Hash, Spikard::Response] Response data or Response object
    # @return [Proc] Wrapped handler proc
    #
    # @example
    #   app.post('/upload', &wrap_handler_with_context do |ctx|
    #     {
    #       file: ctx[:body][:file].filename,
    #       query_params: ctx[:query]
    #     }
    #   end)
    def wrap_handler_with_context(&handler)
      raise ArgumentError, 'block required for wrap_handler_with_context' unless handler

      lambda do |params, query, body|
        converted_body = Converters.convert_handler_body(body)
        context = {
          params: params,
          query: query,
          body: converted_body
        }
        handler.call(context)
      end
    end
  end
end
