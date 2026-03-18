# frozen_string_literal: true

module Spikard
  # Wrapper class for dependency providers
  #
  # This class wraps factory functions and configuration for dependency injection.
  # It provides a consistent API across Python, Node.js, and Ruby bindings.
  #
  # @example Factory with caching
  #   app.provide("db", Spikard::Provide.new(method("create_db"), cacheable: true))
  #
  # @example Factory with dependencies
  #   app.provide("auth", Spikard::Provide.new(
  #     method("create_auth_service"),
  #     depends_on: ["db", "cache"],
  #     singleton: true
  #   ))
  class Provide
    attr_reader :factory, :depends_on, :singleton, :cacheable

    # Create a new dependency provider
    #
    # @param factory [Proc, Method] The factory function that creates the dependency value
    # @param depends_on [Array<String, Symbol>] List of dependency keys this factory depends on
    # @param singleton [Boolean] Whether to cache the value globally (default: false)
    # @param cacheable [Boolean] Whether to cache the value per-request (default: true)
    def initialize(factory, depends_on: [], singleton: false, cacheable: true)
      @factory = factory
      @depends_on = Array(depends_on).map(&:to_s)
      @singleton = singleton
      @cacheable = cacheable
    end

    # Check if the factory is async (based on method arity or other heuristics)
    #
    # @return [Boolean] True if the factory appears to be async
    def async?
      # Ruby doesn't have explicit async/await like Python/JS
      # We could check if it returns a Thread or uses Fiber
      false
    end

    # Check if the factory is an async generator
    #
    # @return [Boolean] True if the factory is an async generator
    def async_generator?
      false
    end
  end

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
  #
  # @example Using Provide wrapper
  #   app.provide("db", Spikard::Provide.new(method("create_db"), cacheable: true))
  module ProvideSupport
    # Register a dependency in the DI container
    #
    # This method supports three patterns:
    # 1. **Value dependency**: Pass a value directly (e.g., string, number, object)
    # 2. **Factory dependency**: Pass a block that computes the value
    # 3. **Provide wrapper**: Pass a Spikard::Provide instance
    #
    # @param key [String, Symbol] Unique identifier for the dependency
    # @param value [Object, Provide, nil] Static value, Provide instance, or nil
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
    #
    # @example Using Provide wrapper
    #   app.provide("db", Spikard::Provide.new(method("create_db"), cacheable: true))
    def provide(key, value = nil, depends_on: [], singleton: false, cacheable: true, &block)
      key_str = key.to_s
      registry = ensure_native_dependencies!

      # Handle Provide wrapper instances
      if value.is_a?(Provide)
        registry.register_factory(key_str, value.factory, value.depends_on, value.singleton, value.cacheable)
      elsif block
        registry.register_factory(key_str, block, Array(depends_on).map(&:to_s), singleton, cacheable)
      else
        raise ArgumentError, 'Either provide a value or a block, not both' if value.nil?

        registry.register_value(key_str, value)
      end

      self
    end

    # Get all registered dependencies
    #
    # @return [Hash] Dictionary mapping dependency keys to their definitions
    # @api private
    def dependencies
      ensure_native_dependencies!
    end

    private

    def ensure_native_dependencies!
      registry = (@native_dependencies if instance_variable_defined?(:@native_dependencies) && @native_dependencies)
      raise 'Spikard native dependency registry unavailable' unless registry

      registry
    end
  end

  # Internal helpers for resolving and caching DI dependencies.
  module DependencyResolutionHelpers
    UNRESOLVED_DEPENDENCY = Object.new.freeze

    module_function

    def build_resolution_context(*resolution_args)
      dependencies, request_cache, singleton_cache, stack = resolution_args
      {
        dependencies:,
        request_cache: request_cache || {},
        singleton_cache: singleton_cache || {},
        stack: stack || []
      }
    end

    def resolve_factory_dependency(dep_def, request, context, resolver)
      factory = dep_def[:factory]
      depends_on = Array(dep_def[:depends_on]).map(&:to_s)
      resolved_kwargs = resolver.call(
        depends_on,
        context[:dependencies],
        request,
        context[:request_cache],
        context[:singleton_cache],
        context[:stack]
      )
      invoke_factory(factory, depends_on, resolved_kwargs)
    end

    def fetch_dependency_definition(key, dependencies, stack)
      dep_def = dependencies[key]
      raise KeyError, "Missing dependency: #{key}" unless dep_def
      raise ArgumentError, "Circular dependency detected: #{(stack + [key]).join(' -> ')}" if stack.include?(key)

      dep_def
    end

    def lookup_cached_dependency(key, dep_def, request_cache, singleton_cache)
      return singleton_cache[key] if dep_def[:singleton] && singleton_cache.key?(key)
      return request_cache[key] if dep_def.fetch(:cacheable, true) && request_cache.key?(key)

      UNRESOLVED_DEPENDENCY
    end

    def cache_dependency_value(key, dep_def, value, request_cache, singleton_cache)
      singleton_cache[key] = value if dep_def[:singleton]
      request_cache[key] = value if dep_def.fetch(:cacheable, true)
    end

    def invoke_factory(factory, depends_on, resolved_kwargs)
      return factory.call if depends_on.empty?

      parameters = factory.parameters
      ordered_values = depends_on.map { |dependency_key| resolved_kwargs.fetch(dependency_key.to_sym) }

      if parameters.any? { |type, _name| %i[key keyreq keyrest].include?(type) }
        factory.call(**resolved_kwargs)
      else
        factory.call(*ordered_values)
      end
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
    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
    def self.wrap_handler(handler, dependencies)
      # Extract parameter names from the handler
      params = handler.parameters.map { |_type, name| name.to_s }
      singleton_cache = {}

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
        request_cache = {}

        injectable_params.each do |param_name|
          kwargs[param_name.to_sym] = resolve_dependency_by_key(
            param_name,
            dependencies,
            request,
            request_cache,
            singleton_cache,
            []
          )
        end

        # Call original handler with injected dependencies
        if handler.arity.zero?
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
    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    # Resolve a dependency definition
    #
    # @param dep_def [Hash] Dependency definition
    # @param request [Hash] Request context (unused for now, future: per-request deps)
    # @return [Object] Resolved dependency value
    # @api private
    def self.resolve_dependency(dep_def, request, *resolution_args)
      context = DependencyResolutionHelpers.build_resolution_context(*resolution_args)
      return dep_def[:value] if dep_def[:type] == :value
      raise ArgumentError, "Unknown dependency type: #{dep_def[:type].inspect}" unless dep_def[:type] == :factory

      DependencyResolutionHelpers.resolve_factory_dependency(
        dep_def,
        request,
        context,
        method(:resolve_factory_dependencies)
      )
    end

    def self.resolve_dependency_by_key(key, dependencies, request, request_cache, singleton_cache, stack)
      key = key.to_s
      dep_def = DependencyResolutionHelpers.fetch_dependency_definition(key, dependencies, stack)
      cached_value = DependencyResolutionHelpers.lookup_cached_dependency(key, dep_def, request_cache, singleton_cache)
      return cached_value unless cached_value.equal?(DependencyResolutionHelpers::UNRESOLVED_DEPENDENCY)

      value = resolve_dependency(dep_def, request, dependencies, request_cache, singleton_cache, stack + [key])
      DependencyResolutionHelpers.cache_dependency_value(key, dep_def, value, request_cache, singleton_cache)
      value
    end

    def self.resolve_factory_dependencies(depends_on, dependencies, request, request_cache, singleton_cache, stack)
      return {} if depends_on.empty?

      raise ArgumentError, 'Dependency registry is required for nested dependency resolution' unless dependencies

      depends_on.to_h do |dependency_key|
        [dependency_key.to_sym, resolve_dependency_by_key(
          dependency_key,
          dependencies,
          request,
          request_cache,
          singleton_cache,
          stack
        )]
      end
    end
  end
end
