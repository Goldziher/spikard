# frozen_string_literal: true

require 'simplecov'
require 'simplecov-lcov'

SimpleCov::Formatter::LcovFormatter.config do |c|
  c.report_with_single_file = true
  c.single_report_path = 'coverage/lcov.info'
end

SimpleCov.formatters = [
  SimpleCov::Formatter::HTMLFormatter,
  SimpleCov::Formatter::LcovFormatter
]

SimpleCov.start do
  minimum_coverage 80
  minimum_coverage_by_file 75

  add_filter '/spec/'
  add_filter '/vendor/'
  add_filter '/lib/spikard/version.rb'

  add_group 'Main Logic', '/lib/'
end

SimpleCov.at_exit do
  SimpleCov.result.format!
  Spikard::Background.shutdown if defined?(Spikard::Background)
  Thread.list.each do |thread|
    next if thread == Thread.current

    thread.kill
    thread.join(1)
  end
end

require 'bundler/setup'
$LOAD_PATH.unshift File.expand_path('../lib', __dir__)
require 'spikard'

RSpec.configure do |config|
  config.expect_with :rspec do |expectations|
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end

  config.mock_with :rspec do |mocks|
    mocks.verify_partial_doubles = true
  end

  config.shared_context_metadata_behavior = :apply_to_host_groups

end
