# frozen_string_literal: true

# Main Ruby namespace for the Spikard bindings.
module Spikard
end

require 'json'
require_relative 'spikard/version'
require_relative 'spikard/response'
require_relative 'spikard/app'
require_relative 'spikard/testing'

begin
  require 'spikard_rb'
rescue LoadError => e
  warn "Unable to load the spikard native extension: #{e.message} -- falling back to pure Ruby shim."

  module Spikard # :nodoc:
    # Namespace stub for the native extension when the compiled library is unavailable.
    module Native
    end

    def self.version
      VERSION
    end
  end
end
