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
  #   class CreateUserRequest < Dry::Struct
  #     attribute :name, Types::String
  #     attribute :age, Types::Integer
  #   end
  #
  #   app.post("/users") do |user: CreateUserRequest|
  #     { id: 1, name: user.name, age: user.age }
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
    def add_route(method, path, handler)
      method_upper = method.upcase
      raise "Unsupported HTTP method: #{method.inspect}" unless VALID_METHODS.include?(method_upper)

      spec = Introspection.introspect(handler, method_upper, path)

      # Convert method string to Method enum value
      method_enum = method_string_to_enum(method_upper)

      # Create a RouteBuilder and attach schemas for Rust-side validation
      builder = create_route_builder_with_schemas(method_enum, path, spec)

      # Build the adapter that bridges Rust RequestData → handler kwargs → response
      adapter = make_adapter(spec)

      # Store as a "route" registration with the RouteBuilder and adapter
      @registrations.push(["route", [builder], adapter])

      handler
    rescue StandardError => e
      raise "Failed to register route #{method_upper} #{path}: #{e.message}"
    end

    # Convert HTTP method string to Method enum value
    def method_string_to_enum(method_upper)
      case method_upper
      when "GET"
        Spikard::Method::Get
      when "POST"
        Spikard::Method::Post
      when "PUT"
        Spikard::Method::Put
      when "PATCH"
        Spikard::Method::Patch
      when "DELETE"
        Spikard::Method::Delete
      when "HEAD"
        Spikard::Method::Head
      when "OPTIONS"
        Spikard::Method::Options
      when "CONNECT"
        Spikard::Method::Connect
      when "TRACE"
        Spikard::Method::Trace
      else
        raise "Unsupported HTTP method: #{method_upper}"
      end
    end

    # Register a GET route.
    def get(path, &block)
      add_route("GET", path, block)
    end

    # Register a POST route.
    def post(path, &block)
      add_route("POST", path, block)
    end

    # Register a PUT route.
    def put(path, &block)
      add_route("PUT", path, block)
    end

    # Register a PATCH route.
    def patch(path, &block)
      add_route("PATCH", path, block)
    end

    # Register a DELETE route.
    def delete(path, &block)
      add_route("DELETE", path, block)
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
    def route(path, method: "GET", &block)
      add_route(method, path, block)
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
    def create_route_builder_with_schemas(method_enum, path, spec)
      # Create the base RouteBuilder with the HTTP method and path
      # NOTE: This requires the native Spikard::RouteBuilder to be available
      builder = Spikard::RouteBuilder.new(method_enum, path)
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
        kwargs = build_kwargs(request_dict, spec)
        result = handler.call(**kwargs)
        to_envelope(result)
      end
    end

    # Split request_dict by source and hydrate each handler kwarg into its Ruby type.
    def build_kwargs(request_dict, spec)
      validated = request_dict["validated_params"] || {}
      sources = {
        "path" => request_dict["path_params"] || {},
        "query" => request_dict["query_params"] || {},
        "header" => request_dict["headers"] || {},
        "cookie" => request_dict["cookies"] || {}
      }

      kwargs = {}
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

      if spec.body_param_name
        body = request_dict["body"]
        kwargs[spec.body_param_name.to_sym] = convert_value(body, spec.body_type)
      end

      kwargs
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

      # Check if target_type is a Dry::Struct subclass
      if target_type.is_a?(Class) && target_type < Dry::Struct
        return target_type.new(raw)
      end

      raw
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
