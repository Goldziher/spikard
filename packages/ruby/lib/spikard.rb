# frozen_string_literal: true

# Main Ruby namespace for the Spikard bindings.
module Spikard
end

begin
  require 'json'
rescue LoadError
  # Fallback to pure-Ruby implementation when native JSON extension is unavailable
  require 'json/pure'
end
require_relative 'spikard/version'
require_relative 'spikard/config'
require_relative 'spikard/response'
require_relative 'spikard/background'
require_relative 'spikard/schema'
require_relative 'spikard/websocket'
require_relative 'spikard/sse'
require_relative 'spikard/grpc'
require_relative 'spikard/upload_file'
require_relative 'spikard/converters'
require_relative 'spikard/provide'
require_relative 'spikard/handler_wrapper'
require_relative 'spikard/app'
require_relative 'spikard/testing'

begin
  require 'spikard_rb'
rescue LoadError => e
  raise LoadError, <<~MSG, e.backtrace
    Failed to load the Spikard native extension (spikard_rb). Run `bundle exec rake ext:build` to compile it before executing tests.
    Original error: #{e.message}
  MSG
end

# Convenience aliases and methods at top level
module Spikard
  TestClient = Testing::TestClient

  # Handler wrapper utilities
  extend HandlerWrapper
end
