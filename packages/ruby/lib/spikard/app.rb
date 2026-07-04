# frozen_string_literal: true

# Ergonomic typed-handler application layer for Spikard.
#
# This module provides the public +App+ — the ergonomic, FastAPI-style entry
# point for the Ruby binding. It sits on top of the thin, Alef-generated low-level
# +Spikard.service.App+ (which forwards registrations to the Rust core) and adds:
#
# - verb methods (+get+, +post+, +put+, +patch+, +delete+, +head+, +options+);
# - automatic parameter extraction and validation via Method#parameters introspection;
# - a per-route Ruby adapter that bridges the Rust +handler_bridge+ contract:
#   the bridge calls +adapter.call(request_data_hash)+ with one positional hash
#   (a JSON-parsed +spikard::RequestData+) and returns the Rust struct encoding
#   the wire +{content, status_code, headers}+ response envelope.
#
# All business logic (routing, validation, middleware) lives in the Rust core. This
# layer only performs type introspection at registration time and type hydration/
# serialization at request time using Dry::Struct.

require_relative "params"
require_relative "introspection"

module Spikard
  # Ergonomic application with typed handlers and DTO binding.
  #
  # @example
  #   app = Spikard::App.new
  #
  #   class CreateUser < Dry::Struct
  #     attribute :name, Types::String
  #     attribute :age, Types::Integer
  #   end
  #
  #   app.post("/users", body: CreateUser) do |user|
  #     user
  #   end
  #
  #   app.run
  class App
    # Valid HTTP methods
    VALID_METHODS = Set.new(%w[GET POST PUT PATCH DELETE HEAD OPTIONS CONNECT TRACE]).freeze

    # Parameter names that are never treated as request-derived kwargs
    SPECIAL_PARAM_NAMES = Set.new(%w[self]).freeze

    def initialize(config = nil)
      # Create a low-level App via the native extension interface
      # The low-level App is already available in the Spikard namespace
      @registrations = []
      @config = config
    end

    # Set the server configuration and return self for chaining.
    def config(config)
      @config = config
      self
    end

    # Register a handler for method and path and return it unchanged.
    def add_route(method, path, handler, body_type: nil)
      method_upper = method.upcase
      raise "Unsupported HTTP method: #{method.inspect}" unless VALID_METHODS.include?(method_upper)

      spec = Introspection.introspect(handler, method_upper, path, body_type)

      # Create a RouteBuilder and attach schemas for Rust-side validation.
      # The native RouteBuilder.new accepts the lowercase HTTP method string
      # (e.g. "post") and maps it to the core Method internally.
      builder = create_route_builder_with_schemas(method_upper.downcase, path, spec)

      # Build the adapter that bridges Rust RequestData → handler kwargs → response
      adapter = make_adapter(spec)

      # Store as a "route" registration with the RouteBuilder and adapter
      @registrations.push(["route", [builder], adapter])

      handler
    rescue StandardError => e
      raise "Failed to register route #{method_upper} #{path}: #{e.message}"
    end

    # Register a GET route.
    def get(path, &block)
      add_route("GET", path, block)
    end

    # Register a POST route with optional body DTO class.
    def post(path, body: nil, &block)
      add_route("POST", path, block, body_type: body)
    end

    # Register a PUT route with optional body DTO class.
    def put(path, body: nil, &block)
      add_route("PUT", path, block, body_type: body)
    end

    # Register a PATCH route with optional body DTO class.
    def patch(path, body: nil, &block)
      add_route("PATCH", path, block, body_type: body)
    end

    # Register a DELETE route with optional body DTO class.
    def delete(path, body: nil, &block)
      add_route("DELETE", path, block, body_type: body)
    end

    # Register a HEAD route.
    def head(path, &block)
      add_route("HEAD", path, block)
    end

    # Register an OPTIONS route.
    def options(path, &block)
      add_route("OPTIONS", path, block)
    end

    # Register a TRACE route.
    def trace(path, &block)
      add_route("TRACE", path, block)
    end

    # Register a CONNECT route.
    def connect(path, &block)
      add_route("CONNECT", path, block)
    end

    # Register a route with an explicit HTTP method (defaults to GET).
    def route(path, method: "GET", body: nil, &block)
      add_route(method, path, block, body_type: body)
    end

    # Run the HTTP server using the configured routes.
    def run
      Spikard.app_run(@registrations)
    end

    # Build the underlying Axum router (for embedding/testing).
    def into_router
      Spikard.app_into_router(@registrations)
    end

    private

    # Create a RouteBuilder with request and params schemas attached for Rust-side validation.
    def create_route_builder_with_schemas(method_name, path, spec)
      # Create the base RouteBuilder with the HTTP method and path.
      # method_name is the lowercase HTTP verb (e.g. "post"); the native
      # RouteBuilder maps it to the core Method.
      builder = Spikard::RouteBuilder.new(method_name, path)
      builder = builder.handler_name(spec.handler.respond_to?(:name) ? spec.handler.name : "handler")

      # Attach request schema for body validation if a body parameter exists.
      # Introspection.introspect only sets body_param_name for body-capable
      # methods, so no HTTP-method re-check is needed here.
      if spec.body_param_name
        request_schema = Introspection.derive_json_schema(spec.body_type)
        if request_schema
          builder = builder.request_schema_json(JSON.dump(request_schema))
        end
      end

      # Attach params schema for query/path/header/cookie validation
      params_schema = build_params_schema(spec.bindings)
      if params_schema
        builder = builder.params_schema_json(JSON.dump(params_schema))
      end

      builder
    end

    # Create the synchronous Ruby callable registered with the Rust bridge for one route.
    #
    # The Rust bridge invokes the returned callable as adapter.call(request_data_hash)
    # and expects it to return { status_code:, content:, headers: }.
    #
    # Validation errors are caught by Rust core (via attached JSON schemas) before
    # the adapter is called, so only unexpected errors are handled here.
    def make_adapter(spec)
      handler = spec.handler

      lambda do |request_dict|
        positional, kwargs = build_call_args(request_dict, spec)
        result = handler.call(*positional, **kwargs)
        to_envelope(result)
      end
    end

    # Build positional and keyword arguments for handler invocation.
    #
    # If body_type is provided, the hydrated body DTO is the FIRST positional arg.
    # Remaining bindings become keyword args (path, query, header, cookie by name).
    def build_call_args(request_dict, spec)
      validated = request_dict["validated_params"] || {}
      sources = {
        "path" => request_dict["path_params"] || {},
        "query" => request_dict["query_params"] || {},
        "header" => request_dict["headers"] || {},
        "cookie" => request_dict["cookies"] || {}
      }

      positional = []
      kwargs = {}

      # First positional arg: the hydrated body DTO (if body_type provided)
      if spec.body_param_name && spec.body_type
        body = request_dict["body"]
        positional << convert_value(body, spec.body_type)
      end

      # Keyword args: non-body bindings by name
      spec.bindings.each do |binding|
        raw = nil
        found = false

        if validated.is_a?(Hash) && validated.key?(binding.name)
          raw = validated[binding.name]
          found = true
        else
          container = sources[binding.source] || {}
          if container.is_a?(Hash) && container.key?(binding.name)
            raw = container[binding.name]
            found = true
          end
        end

        unless found
          if binding.param_default.is_a?(ParamBase) && binding.param_default.has_default?
            kwargs[binding.name.to_sym] = binding.param_default.get_default
          end

          next
        end

        kwargs[binding.name.to_sym] = convert_value(raw, binding.target_type)
      end

      [positional, kwargs]
    end

    # Interpret a handler return value into the wire response envelope.
    #
    # An object exposing +status_code+, +content+, +headers+ becomes a custom
    # response; anything else is a plain 200 whose body is the serialised value.
    def to_envelope(result)
      return {status_code: 200, content: nil, headers: {}} if result.nil?

      if result.is_a?(Hash) && result.key?(:status_code) && result.key?(:content) && result.key?(:headers)
        return result
      end

      # If the result is a Dry::Struct, serialize it to hash
      content = if result.respond_to?(:to_h)
        result.to_h
      else
        result
      end

      {status_code: 200, content: content, headers: {}}
    end

    # Convert a raw request value to the target type.
    # For Dry::Struct classes, instantiate with the hash; otherwise return as-is.
    def convert_value(raw, target_type)
      return raw if raw.nil?

      # Check if target_type is a Dry::Struct subclass. The core delivers the
      # JSON body as a Hash with STRING keys; Dry::Struct expects symbol keys,
      # so deep-symbolize before instantiating.
      if target_type.is_a?(Class) && target_type < Dry::Struct
        return target_type.new(deep_symbolize_keys(raw))
      end

      raw
    end

    # Recursively convert Hash string keys to symbols (Array elements included)
    # so a JSON-decoded body hydrates cleanly into a Dry::Struct.
    def deep_symbolize_keys(value)
      case value
      when Hash
        value.each_with_object({}) { |(k, v), acc| acc[k.to_sym] = deep_symbolize_keys(v) }
      when Array
        value.map { |element| deep_symbolize_keys(element) }
      else
        value
      end
    end

    # Build JSON Schema for query/path/header/cookie parameters.
    #
    # Returns a hash suitable for validation by Rust core, or nil if no bindings.
    def build_params_schema(bindings)
      return nil if bindings.empty?

      properties = {}
      required = []

      bindings.each do |binding|
        properties[binding.name] = {
          "type" => type_to_json_schema_type(binding.target_type),
          "source" => binding.source
        }

        # Path parameters are always required; others depend on defaults
        if binding.source == "path" || !binding.param_default&.is_a?(ParamBase) || !binding.param_default.has_default?
          required << binding.name
        end
      end

      {
        "type" => "object",
        "properties" => properties,
        "required" => required
      }
    end

    # Map Ruby types to JSON Schema type strings.
    def type_to_json_schema_type(type)
      case type.to_s
      when /String/
        "string"
      when /Integer/
        "integer"
      when /Float/
        "number"
      when /TrueClass|FalseClass/
        "boolean"
      when /Array/
        "array"
      when /Hash/
        "object"
      else
        # Default to string for unknown types
        "string"
      end
    end
  end
end
