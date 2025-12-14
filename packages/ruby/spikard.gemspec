# frozen_string_literal: true

require_relative 'lib/spikard/version'

# rubocop:disable Metrics/BlockLength
Gem::Specification.new do |spec|
  spec.name          = 'spikard'
  spec.version       = Spikard::VERSION
  spec.authors       = ["Na'aman Hirschfeld"]
  spec.email         = ['nhirschfeld@gmail.com']

  spec.summary       = 'High-performance HTTP toolkit with Rust core and Ruby bindings'
  spec.description   = <<~DESC
    Spikard is a Rust-centric multi-language HTTP toolkit providing a high-performance core library
    and language bindings (Python, Node.js, Ruby, PHP, WebAssembly) to build and validate typed web services.

    The Ruby binding uses Magnus for zero-overhead FFI, providing Sinatra-style routing, full async/await support,
    WebSockets, Server-Sent Events, request validation with JSON Schema and dry-schema, lifecycle hooks,
    dependency injection, and comprehensive middleware stack (compression, rate limiting, authentication).

    Features:
    - Zero-copy Rust-to-Ruby serialization via Magnus
    - Async-first with Tokio and Axum backing
    - Type-safe RBS type definitions for Steep
    - Tower-HTTP middleware stack
    - Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError)
    - Built-in WebSocket and SSE support
    - Request validation with JSON Schema
  DESC
  spec.homepage      = 'https://github.com/Goldziher/spikard'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2.0'
  spec.platform = Gem::Platform::CURRENT

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => spec.homepage,
    'changelog_uri' => "#{spec.homepage}/blob/main/CHANGELOG.md",
    'documentation_uri' => "#{spec.homepage}/tree/main/packages/ruby#documentation",
    'bug_tracker_uri' => "#{spec.homepage}/issues",
    'funding_uri' => spec.homepage.to_s,
    'rubygems_mfa_required' => 'true'
  }

  spec.files = Dir[
    'lib/**/*.rb',
    'ext/**/*.{rs,toml,lock,rb}',
    'sig/**/*.rbs',
    'vendor/**/*.{rs,toml}', # Vendored workspace crates
    'LICENSE',
    'README.md'
  ]
  spec.require_paths = ['lib']
  spec.extensions = ['ext/spikard_rb/extconf.rb']

  # Runtime dependency for WebSocket test client (subprocess approach)
  spec.add_dependency 'websocket-client-simple', '~> 0.8'
end
# rubocop:enable Metrics/BlockLength
