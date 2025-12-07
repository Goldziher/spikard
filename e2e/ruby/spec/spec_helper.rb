# frozen_string_literal: true

require "spikard"
require_relative "../app/main"

DRY_SCHEMA_AVAILABLE = begin
  require "dry-schema"
  Dry::Schema.load_extensions(:json_schema)
  true
rescue LoadError
  false
end

DRY_STRUCT_AVAILABLE = begin
  require "dry-struct"
  require "dry-types"
  unless defined?(Types)
    module Types
      include Dry.Types()
    end
  end
  true
rescue LoadError
  false
end

RSpec.configure do |config|
  config.expect_with :rspec do |expectations|
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end

  config.mock_with :rspec do |mocks|
    mocks.verify_partial_doubles = true
  end

  config.shared_context_metadata_behavior = :apply_to_host_groups
end
