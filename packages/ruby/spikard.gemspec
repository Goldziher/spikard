# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "spikard"
  spec.version = "0.15.6.pre.rc.12"
  spec.authors       = ["Na'aman Hirschfeld <nhirschfeld@gmail.com>"]
  spec.summary       = "Rust-centric multi-language HTTP framework with polyglot bindings"
  spec.description   = "Rust-centric multi-language HTTP framework with polyglot bindings"
  spec.homepage      = "https://github.com/Goldziher/spikard"

  spec.license       = "MIT"

  spec.required_ruby_version = ">= 3.2.0"
  spec.metadata["keywords"] = %w[framework http polyglot rust web].join(",")
  spec.metadata["rubygems_mfa_required"] = "true"

  candidate_files    = Dir.glob(%w[README* LICENSE* lib/**/* ext/**/* sig/**/* Steepfile]).select { |f| File.file?(f) }
  spec.files         = candidate_files.reject { |f| f.include?("/native/target/") || f.include?("/native/tmp/") }
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/spikard_rb/native/extconf.rb"]

  spec.add_dependency "rb_sys", ">= 0.9", "< 0.9.128"
  spec.add_dependency "sorbet-runtime", "~> 0.5"
end
