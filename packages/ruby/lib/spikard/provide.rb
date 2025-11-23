# frozen_string_literal: true

module Spikard
  # Dependency Injection support for Spikard applications
  #
  # Provides methods for registering and managing dependencies that can be
  # automatically injected into route handlers.
  #
  # @example Registering a value dependency
  #   app.provide("database_url", "postgresql://localhost/mydb")
  #
  # @example Registering a factory dependency
  #   app.provide("db_pool", depends_on: ["database_url"]) do |database_url:|
  #     ConnectionPool.new(database_url)
  #   end
  #
  # @example Singleton dependency (shared across all requests)
  #   app.provide("config", singleton: true) do
  #     Config.load_from_file("config.yml")
  #   end
  module ProvideSupport
    # Register a dependency in the DI container
    #
    # This method supports two patterns:
    # 1. **Value dependency**: Pass a value directly (e.g., string, number, object)
    # 2. **Factory dependency**: Pass a block that computes the value
    #
    # @param key [String, Symbol] Unique identifier for the dependency
    # @param value [Object, nil] Static value for the dependency (if no block given)
    # @param depends_on [Array<String, Symbol>] List of dependency keys this factory depends on
    # @param singleton [Boolean] Whether to cache the value globally (default: false)
    # @param cacheable [Boolean] Whether to cache the value per-request (default: true)
    # @yield Optional factory block that receives dependencies as keyword arguments
    # @yieldparam **deps [Hash] Resolved dependencies as keyword arguments
    # @yieldreturn [Object] The computed dependency value
    # @return [self] Returns self for method chaining
    #
    # @example Value dependency
    #   app.provide("app_name", "MyApp")
    #   app.provide("port", 8080)
    #
    # @example Factory with dependencies
    #   app.provide("database", depends_on: ["config"]) do |config:|
    #     Database.connect(config["db_url"])
    #   end
    #
    # @example Singleton factory
    #   app.provide("thread_pool", singleton: true) do
    #     ThreadPool.new(size: 10)
    #   end
    #
    # @example Non-cacheable factory (resolves every time)
    #   app.provide("request_id", cacheable: false) do
    #     SecureRandom.uuid
    #   end
    def provide(key, value = nil, depends_on: [], singleton: false, cacheable: true, &block)
      key_str = key.to_s
      @dependencies ||= {}

      if block
        # Factory dependency
        @dependencies[key_str] = {
          type: :factory,
          factory: block,
          depends_on: Array(depends_on).map(&:to_s),
          singleton: singleton,
          cacheable: cacheable
        }
      else
        # Value dependency
        raise ArgumentError, "Either provide a value or a block, not both" if value.nil?

        @dependencies[key_str] = {
          type: :value,
          value: value,
          singleton: true, # Values are always singleton
          cacheable: true
        }
      end

      self
    end

    # Get all registered dependencies
    #
    # @return [Hash] Dictionary mapping dependency keys to their definitions
    # @api private
    def dependencies
      @dependencies ||= {}
      @dependencies.dup
    end
  end

  # Dependency injection handler wrapper
  #
  # Wraps a route handler to inject dependencies based on parameter names.
  # Dependencies are resolved from the DI container and passed as keyword arguments.
  #
  # @api private
  module DIHandlerWrapper
    # Wrap a handler to inject dependencies
    #
    # @param handler [Proc] The original route handler
    # @param dependencies [Hash] Available dependencies from the app
    # @return [Proc] Wrapped handler with DI support
    def self.wrap_handler(handler, dependencies)
      # Extract parameter names from the handler
      params = handler.parameters.map { |_type, name| name.to_s }

      # Find which parameters match registered dependencies
      injectable_params = params & dependencies.keys

      if injectable_params.empty?
        # No DI needed, return original handler
        return handler
      end

      # Create wrapped handler that injects dependencies
      lambda do |request|
        # Build kwargs with injected dependencies
        kwargs = {}

        injectable_params.each do |param_name|
          dep_def = dependencies[param_name]
          kwargs[param_name.to_sym] = resolve_dependency(dep_def, request)
        end

        # Call original handler with injected dependencies
        if handler.arity == 0
          # Handler takes no arguments (dependencies injected via closure or instance vars)
          handler.call
        elsif injectable_params.length == params.length
          # All parameters are dependencies
          handler.call(**kwargs)
        else
          # Mix of request data and dependencies
          handler.call(request, **kwargs)
        end
      end
    end

    # Resolve a dependency definition
    #
    # @param dep_def [Hash] Dependency definition
    # @param request [Hash] Request context (unused for now, future: per-request deps)
    # @return [Object] Resolved dependency value
    # @api private
    def self.resolve_dependency(dep_def, _request)
      case dep_def[:type]
      when :value
        dep_def[:value]
      when :factory
        # For factories, resolve dependencies and call
        factory = dep_def[:factory]
        depends_on = dep_def[:depends_on]

        if depends_on.empty?
          factory.call
        else
          # TODO: Implement nested dependency resolution
          # For now, factories with dependencies should be pre-resolved
          factory.call
        end
      end
    end
  end
end
