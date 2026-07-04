# frozen_string_literal: true

# Runtime handler introspection for extracting parameter definitions.
#
# Provides Method#parameters analysis to classify parameters into
# path, query, header, cookie, and body sources.

require_relative "params"

module Spikard
  # Handler introspection via Method#parameters.
  module Introspection
    # Resolved binding for a single non-body handler parameter.
    class Binding
      attr_reader :name, :source, :target_type, :param_default

      def initialize(name, source, target_type, param_default)
        @name = name
        @source = source
        @target_type = target_type
        @param_default = param_default
      end

      def has_plain_default?
        @param_default && !@param_default.is_a?(ParamBase)
      end
    end

    # Introspected, per-route metadata derived from a handler at registration time.
    class RouteSpec
      attr_reader :handler, :bindings, :body_param_name, :body_type

      def initialize(handler, bindings, body_param_name, body_type)
        @handler = handler
        @bindings = bindings
        @body_param_name = body_param_name
        @body_type = body_type
      end
    end

    # Analyse handler and produce a RouteSpec describing how to bind requests.
    def self.introspect(handler, method, path)
      path_names = path_param_names(path)
      method_has_body = !BODYLESS_METHODS.include?(method)

      bindings = []
      body_param_name = nil
      body_type = nil

      # Get method parameters using Ruby's Method#parameters
      # Each param is [param_type, param_name] where param_type is one of:
      # :req (required), :opt (optional), :rest, :keyreq (required keyword), :key (optional keyword), :block
      handler.method(:call).parameters.each do |param_type, param_name|
        next if SPECIAL_PARAM_NAMES.include?(param_name.to_s)

        # Skip *args and **kwargs parameters
        next if param_type == :rest || param_type == :keyreq || param_type == :rest
        next if param_name.nil?

        param_name_str = param_name.to_s
        default = nil

        # For blocks passed to add_route, we can't easily get default values
        # Ruby's Method#parameters doesn't provide type info or defaults directly
        # We'll treat all parameters as potentially required unless marked otherwise

        source, target_type, is_body = classify_parameter(
          param_name_str,
          nil,
          default,
          path_names,
          method_has_body,
          body_param_name.nil?
        )

        if is_body
          body_param_name = param_name_str
          body_type = target_type || Object
          next
        end

        bindings <<
          Binding.new(
            param_name_str,
            source,
            target_type || Object,
            default
          )
      end

      RouteSpec.new(handler, bindings, body_param_name, body_type)
    end

    # Classify a parameter into (source, target_type, is_body).
    #
    # Precedence:
    # 1. Explicit Header/Cookie/Path/Query/Body markers in defaults
    # 2. Path membership via {id} patterns
    # 3. Structured first parameter (implicit body)
    # 4. Query parameter (default)
    def self.classify_parameter(name, annotation, default, path_names, method_has_body, body_not_taken)
      # Check if default is a ParamBase subclass instance
      if default.is_a?(Header)
        return ["header", annotation || String, false]
      elsif default.is_a?(Cookie)
        return ["cookie", annotation || String, false]
      elsif default.is_a?(Path)
        return ["path", annotation || String, false]
      elsif default.is_a?(Query)
        return ["query", annotation || String, false]
      elsif default.is_a?(Body)
        return ["body", annotation || Object, true]
      end

      # Check if name is a path parameter
      if path_names.include?(name)
        return ["path", annotation || String, false]
      end

      # For first structured param with body-capable method, treat as implicit body
      if method_has_body && body_not_taken && annotation&.is_a?(Class) && annotation < Dry::Struct
        return ["body", annotation, true]
      end

      # Default to query parameter
      ["query", annotation || String, false]
    end

    # Return the set of path-parameter names in path (handles {id} and {id:int}).
    def self.path_param_names(path)
      # Match {name} and typed {name:converter} path segments
      path.scan(/\{(\w+)(?::[^{}]+)?\}/).map(&:first).to_set
    end

    BODYLESS_METHODS = Set.new(%w[GET HEAD OPTIONS TRACE CONNECT DELETE]).freeze
    SPECIAL_PARAM_NAMES = Set.new(%w[self]).freeze
  end
end
