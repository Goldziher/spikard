# frozen_string_literal: true

require_relative 'lib/spikard/version'

# rubocop:disable Metrics/BlockLength
Gem::Specification.new do |spec|
  spec.name          = 'spikard'
  spec.version       = Spikard::VERSION
  spec.authors       = ["Na'aman Hirschfeld"]
  spec.email         = ['nhirschfeld@gmail.com']

  spec.summary       = 'Ruby bindings for the Spikard HTTP toolkit'
  spec.description   = <<~DESC
    Spikard provides a high-performance HTTP toolkit with a Rust core and thin language bindings.
    This gem bundles the Ruby bridge implemented with Magnus.
  DESC
  spec.homepage      = 'https://github.com/Goldziher/spikard'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2.0'

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => spec.homepage,
    'changelog_uri' => "#{spec.homepage}/blob/main/CHANGELOG.md",
    'rubygems_mfa_required' => 'true'
  }

  spec.files = Dir[
    'lib/**/*.rb',
    'ext/**/*.{rs,toml,lock,rb}',
    'sig/**/*.rbs',
    'LICENSE',
    'README.md'
  ]
  spec.require_paths = ['lib']
  spec.extensions = ['ext/spikard_rb/extconf.rb']

  # Runtime dependency for WebSocket test client (subprocess approach)
  spec.add_dependency 'websocket-client-simple', '~> 0.8'
end
# rubocop:enable Metrics/BlockLength
