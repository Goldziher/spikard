# frozen_string_literal: true

module Spikard
  # Schema extraction helpers for Ruby type systems
  #
  # Supports:
  # - Plain JSON Schema (Hash)
  # - Dry::Schema with :json_schema extension
  # - Dry::Struct (Dry-Types)
  #
  # @example With Dry::Schema
  #   require 'dry-schema'
  #   Dry::Schema.load_extensions(:json_schema)
  #
  #   UserSchema = Dry::Schema.JSON do
  #     required(:email).filled(:str?)
  #     required(:age).filled(:int?)
  #   end
  #
  #   schema = Spikard::Schema.extract_json_schema(UserSchema)
  #
  # @example With Dry::Struct
  #   require 'dry-struct'
  #
  #   class User < Dry::Struct
  #     attribute :email, Types::String
  #     attribute :age, Types::Integer
  #   end
  #
  #   schema = Spikard::Schema.extract_json_schema(User)
  #
  # @example With plain JSON Schema
  #   schema_hash = {
  #     "type" => "object",
  #     "properties" => {
  #       "email" => { "type" => "string" },
  #       "age" => { "type" => "integer" }
  #     },
  #     "required" => ["email", "age"]
  #   }
  #
  #   schema = Spikard::Schema.extract_json_schema(schema_hash)
  module Schema
    class << self
      # Extract JSON Schema from various Ruby schema sources
      #
      # @param schema_source [Object] The schema source (Hash, Dry::Schema, Dry::Struct class)
      # @return [Hash, nil] JSON Schema hash or nil if extraction fails
      def extract_json_schema(schema_source)
        return nil if schema_source.nil?

        # 1. Check if plain JSON Schema hash
        return schema_source if schema_source.is_a?(Hash) && json_schema_hash?(schema_source)

        # 2. Check for Dry::Schema with json_schema extension
        return extract_from_dry_schema(schema_source) if dry_schema?(schema_source)

        # 3. Check for Dry::Struct (Dry-Types)
        return extract_from_dry_struct(schema_source) if dry_struct_class?(schema_source)

        # 4. Unknown type
        warn "Spikard: Unable to extract JSON Schema from #{schema_source.class}. " \
             'Supported types: Hash, Dry::Schema, Dry::Struct'
        nil
      end

      private

      # Check if object is a plain JSON Schema hash
      def json_schema_hash?(obj)
        return false unless obj.is_a?(Hash)

        # Must have 'type' key or '$schema' key
        obj.key?('type') || obj.key?('$schema') || obj.key?(:type) || obj.key?(:$schema)
      end

      # Check if object is a Dry::Schema
      def dry_schema?(obj)
        defined?(Dry::Schema::Processor) && obj.is_a?(Dry::Schema::Processor)
      end

      # Check if object is a Dry::Struct class
      def dry_struct_class?(obj)
        return false unless obj.is_a?(Class)

        defined?(Dry::Struct) && obj < Dry::Struct
      end

      # Extract JSON Schema from Dry::Schema
      def extract_from_dry_schema(schema)
        unless schema.respond_to?(:json_schema)
          warn 'Spikard: Dry::Schema instance does not have json_schema method. ' \
               'Did you load the :json_schema extension? ' \
               'Add: Dry::Schema.load_extensions(:json_schema)'
          return nil
        end

        begin
          schema.json_schema
        rescue StandardError => e
          warn "Spikard: Failed to extract JSON Schema from Dry::Schema: #{e.message}"
          nil
        end
      end

      # Extract JSON Schema from Dry::Struct class
      def extract_from_dry_struct(struct_class)
        # Dry::Struct doesn't have built-in JSON Schema export
        # We need to manually build it from the attribute schema

        properties = {}
        required = []

        struct_class.schema.each do |key, type_definition|
          # Extract attribute name
          attr_name = key.to_s

          # Determine if required (non-optional)
          is_required = !type_definition.optional?
          required << attr_name if is_required

          # Convert Dry::Types to JSON Schema type
          json_type = dry_type_to_json_schema(type_definition)
          properties[attr_name] = json_type if json_type
        end

        {
          'type' => 'object',
          'properties' => properties,
          'required' => required
        }
      rescue StandardError => e
        warn "Spikard: Failed to extract JSON Schema from Dry::Struct: #{e.message}"
        nil
      end

      # Convert Dry::Types type to JSON Schema type
      def dry_type_to_json_schema(type_def)
        # Get the primitive class name
        type_class = type_def.primitive

        # Map Ruby types to JSON Schema types
        case type_class.to_s
        when 'String'
          { 'type' => 'string' }
        when 'Integer'
          { 'type' => 'integer' }
        when 'Float', 'BigDecimal'
          { 'type' => 'number' }
        when 'TrueClass', 'FalseClass'
          { 'type' => 'boolean' }
        when 'Array'
          {
            'type' => 'array',
            'items' => {} # Could be more specific with member type
          }
        when 'Hash'
          {
            'type' => 'object'
          }
        when 'NilClass'
          { 'type' => 'null' }
        else
          # Default to object for unknown types
          { 'type' => 'object' }
        end
      rescue StandardError
        { 'type' => 'object' }
      end
    end
  end
end
