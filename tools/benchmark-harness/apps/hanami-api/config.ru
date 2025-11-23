# frozen_string_literal: true

# Rack configuration file for Hanami API benchmark server.
# This allows running the server with standard Rack handlers (Puma, WEBrick, etc.)

require_relative 'server'

# Run the Hanami API application
run BenchmarkApp.new
