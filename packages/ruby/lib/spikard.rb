# frozen_string_literal: true

# Main Ruby namespace for the Spikard bindings.
module Spikard
end

require_relative 'spikard/version'

begin
  require 'spikard_rb'
rescue LoadError => e
  warn "Unable to load the spikard native extension: #{e.message} -- falling back to pure Ruby shim."

  module Spikard # :nodoc:
    def self.version
      VERSION
    end
  end
end
