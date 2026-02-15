# frozen_string_literal: true

require 'spec_helper'

# Minimal Dry::Schema/Dry::Struct stubs so we can exercise the extraction helpers
module Dry
  module Schema
    class Processor
      def initialize(result)
        @result = result
      end

      def json_schema
        @result
      end
    end
  end

  # rubocop:disable Lint/EmptyClass
  class Struct; end
  # rubocop:enable Lint/EmptyClass
end

# Helper struct for required/optional attribute extraction.
class FakeStruct < Dry::Struct
  def self.schema
    {
      email: FakeType.new(String),
      age: FakeType.new(Integer),
      nickname: FakeType.new(String, optional: true)
    }
  end
end

# Helper struct used to verify metadata propagation.
class MetadataStruct < Dry::Struct
  def self.schema
    {
      email: FakeType.new(String, meta: { enum: %w[user@example.com admin@example.com], format: :email, min_size: 10 }),
      scores: FakeType.new(Array, member: FakeType.new(Integer))
    }
  end
end

# Simple fake type that carries primitive/metadata info.
class FakeType
  attr_reader :member, :primitive, :meta

  def initialize(primitive, optional: false, meta: nil, member: nil)
    @primitive = primitive
    @optional = optional
    @meta = meta || {}
    @member = member
  end

  def optional?
    @optional
  end
end

RSpec.describe Spikard::Schema do
  describe '.extract_json_schema' do
    it 'returns hash schemas unchanged' do
      schema = { 'type' => 'object', 'properties' => {} }
      expect(described_class.extract_json_schema(schema)).to equal(schema)
    end

    it 'extracts schema via Dry::Schema#json_schema when available' do
      processor = Dry::Schema::Processor.new('json_schema' => { 'type' => 'object' })
      expect(described_class.extract_json_schema(processor)).to eq('json_schema' => { 'type' => 'object' })
    end

    it 'converts Dry::Struct attribute metadata into JSON Schema' do
      schema = described_class.extract_json_schema(FakeStruct)
      expect(schema['properties'].keys).to contain_exactly('email', 'age', 'nickname')
      expect(schema['required']).to contain_exactly('email', 'age')
    end

    it 'applies Dry type metadata such as enum, format, and array members' do
      schema = described_class.extract_json_schema(MetadataStruct)
      email_schema = schema['properties']['email']
      expect(email_schema['enum']).to eq(%w[user@example.com admin@example.com])
      expect(email_schema['format']).to eq('email')
      expect(email_schema['minLength']).to eq(10)

      scores_schema = schema['properties']['scores']
      expect(scores_schema['items']).to eq('type' => 'integer')
    end
  end
end
