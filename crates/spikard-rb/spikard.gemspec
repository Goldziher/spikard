# frozen_string_literal: true

require_relative "lib/spikard/version"

Gem::Specification.new do |spec|
  spec.name = "spikard"
  spec.version = Spikard::VERSION
  spec.authors = ["Na'aman Hirschfeld"]
  spec.email = ["nhirschfeld@gmail.com"]

  spec.summary = "High-performance Ruby HTTP framework powered by Rust"
  spec.description = "Spikard provides Ruby bindings for a high-performance HTTP server built in Rust using Axum and Tokio"
  spec.homepage = "https://github.com/Goldziher/spikard"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Goldziher/spikard"
  spec.metadata["changelog_uri"] = "https://github.com/Goldziher/spikard/blob/main/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  spec.files = Dir[
    "lib/**/*.rb",
    "lib/**/*.{so,bundle,dylib}",  # Native extensions
    "Cargo.toml",
    "src/**/*.rs",
    "build.rs",
    "README.md",
    "LICENSE"
  ]
  spec.require_paths = ["lib"]

  # Extensions to build
  spec.extensions = ["build.rs"]

  # Runtime dependencies
  # (none for now - native extension only)

  # Development dependencies
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "rb_sys", "~> 0.9"
end
