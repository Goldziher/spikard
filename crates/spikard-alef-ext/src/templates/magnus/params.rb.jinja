# frozen_string_literal: true

# Parameter types for dependency injection.
#
# These types are used to extract values from request headers, cookies, etc.
# and to specify default values and factories for query/body/path parameters.

module Spikard
  # Base class for all parameter wrappers.
  #
  # Provides common functionality for default values and default factories.
  class ParamBase
    attr_reader :default, :default_factory, :schema

    def initialize(default: nil, default_factory: nil, schema: nil)
      if default && default_factory
        raise ArgumentError, "Cannot specify both 'default' and 'default_factory'"
      end

      @default = default
      @default_factory = default_factory
      @schema = schema
    end

    # Get the default value, invoking factory if needed.
    def get_default
      if @default_factory
        @default_factory.call
      else
        @default
      end
    end

    # Check if this parameter has a default value.
    def has_default?
      !@default.nil? || !@default_factory.nil?
    end
  end

  # Query parameter with optional default or default_factory.
  #
  # Use this to specify defaults for query string parameters, similar to FastAPI.
  #
  # @example
  #   app.get("/items") do |tags: Query(default_factory: -> { [] })|
  #     { tags: tags }
  #   end
  #
  #   app.get("/items") do |limit: Query(default: 10)|
  #     { limit: limit }
  #   end
  #
  # @param default [Object] Static default value (if no default_factory provided)
  # @param default_factory [Proc] Callable that generates default value when invoked
  # @param schema [Hash] Optional JSON schema dict for custom validation (passed to Rust)
  class Query < ParamBase
  end

  # Request body parameter with optional default or default_factory.
  #
  # Use this to specify defaults for request body parameters.
  #
  # @example
  #   app.post("/items") do |data: Body(default_factory: -> { {} })|
  #     data
  #   end
  #
  # @param default [Object] Static default value (if no default_factory provided)
  # @param default_factory [Proc] Callable that generates default value when invoked
  # @param schema [Hash] Optional JSON schema dict for custom validation (passed to Rust)
  class Body < ParamBase
  end

  # Path parameter metadata.
  #
  # Note: Path parameters are typically required and don't use defaults,
  # but this class is provided for API consistency.
  #
  # @param default [Object] Static default value (rarely used for path params)
  # @param default_factory [Proc] Callable that generates default value (rarely used)
  # @param schema [Hash] Optional JSON schema dict for custom validation (passed to Rust)
  class Path < ParamBase
  end

  # Extract a value from request headers.
  #
  # Use this as a default parameter value to inject header values into route handlers.
  #
  # @example
  #   app.get("/items") do |user_agent: Header(default: "unknown")|
  #     { user_agent: user_agent }
  #   end
  #
  #   app.get("/users/me") do |authorization: Header(default: nil)|
  #     if authorization
  #       { authenticated: true }
  #     else
  #       { authenticated: false }
  #     end
  #   end
  #
  # @param default [Object] Default value if header is not present
  # @param default_factory [Proc] Callable that generates default value when invoked
  # @param header_alias [String] Alternative header name (e.g., "X-API-Key")
  # @param convert_underscores [Boolean] Convert underscores to hyphens in header name
  # @param schema [Hash] Optional JSON schema dict for custom validation (passed to Rust)
  class Header < ParamBase
    attr_reader :header_alias, :convert_underscores

    def initialize(default = nil, default_factory: nil, header_alias: nil, convert_underscores: true, schema: nil)
      super(default, default_factory: default_factory, schema: schema)
      @header_alias = header_alias
      @convert_underscores = convert_underscores
    end
  end

  # Extract a value from request cookies.
  #
  # Use this as a default parameter value to inject cookie values into route handlers.
  #
  # @example
  #   app.get("/items") do |session_id: Cookie(default: nil)|
  #     { session_id: session_id }
  #   end
  #
  #   app.get("/users/me") do |key: Cookie(schema: { minLength: 10 })|
  #     if key == "secret"
  #       { username: "secret" }
  #     else
  #       { error: "Invalid key" }
  #     end
  #   end
  #
  # @param default [Object] Default value if cookie is not present
  # @param default_factory [Proc] Callable that generates default value when invoked
  # @param schema [Hash] Optional JSON schema dict for custom validation (passed to Rust)
  class Cookie < ParamBase
  end
end
