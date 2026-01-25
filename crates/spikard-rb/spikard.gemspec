# frozen_string_literal: true

require_relative "lib/spikard/version"

Gem::Specification.new do |spec|
  spec.name = "spikard"
  spec.version = Spikard::VERSION
  spec.authors = ["Na'aman Hirschfeld"]
  spec.email = ["nhirschfeld@gmail.com"]

  spec.summary = "High-performance Ruby web framework with a Rust core"
  spec.description = "High-performance Ruby web framework with a Rust core. Build REST APIs with Sinatra-style blocks backed by Axum and Tower-HTTP. Features type-safe routing, validation, WebSocket/SSE support, and lifecycle hooks."
  spec.homepage = "https://github.com/Goldziher/spikard"
  spec.license = "MIT"
  spec.required_ruby_version = Gem::Requirement.new(">= 3.2.0", "< 5.0")

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Goldziher/spikard"
  spec.metadata["changelog_uri"] = "https://github.com/Goldziher/spikard/blob/main/CHANGELOG.md"
  spec.metadata["bug_tracker_uri"] = "https://github.com/Goldziher/spikard/issues"
  spec.metadata["documentation_uri"] = "https://github.com/Goldziher/spikard#readme"
  spec.metadata["discord_uri"] = "https://discord.gg/pXxagNK2zN"

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
