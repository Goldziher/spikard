Gem::Specification.new do |spec|
  spec.name          = "spikard"
  spec.version       = "0.13.0"
  spec.authors       = ["Kreuzberg Team"]
  spec.summary       = "Polyglot web framework powered by Rust"
  spec.description   = "Polyglot web framework powered by Rust"
  spec.homepage      = "https://github.com/kreuzberg-dev/spikard"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 2.7.0"
  spec.keywords       = ["web", "framework", "http", "api"]

  spec.files         = Dir.glob("{"lib/**/*", "ext/**/*"}")
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/spikard/extconf.rb"]
end
