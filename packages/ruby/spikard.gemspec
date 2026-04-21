# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'spikard'
  spec.version = '0.13.0'
  spec.authors       = ['Na'aman Hirschfeld <nhirschfeld@gmail.com>']
  spec.summary       = 'Rust-centric multi-language HTTP framework with polyglot bindings'
  spec.description   = 'Rust-centric multi-language HTTP framework with polyglot bindings'
  spec.homepage      = 'https://github.com/Goldziher/spikard'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2.0'
  spec.metadata['keywords'] = ['http', 'web', 'framework', 'polyglot', 'rust'].join(',')
  spec.metadata['rubygems_mfa_required'] = 'true'

  spec.files         = Dir.glob(['lib/**/*', 'ext/**/*'])
  spec.require_paths = ['lib']
  spec.extensions    = ['ext/spikard_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9'
end
