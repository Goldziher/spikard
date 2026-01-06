# frozen_string_literal: true

require 'mkmf'
require 'rb_sys/mkmf'

default_profile = ENV.fetch('CARGO_PROFILE', 'release')

create_rust_makefile('spikard_rb') do |config|
  config.profile = default_profile.to_sym
  # Always use --locked to maintain consistency with vendored crates.
  # The vendor-crates.sh script patches Cargo.toml files and relies on a
  # committed Cargo.lock to avoid workspace collision issues during builds.
  config.extra_cargo_args = ['--locked']
end
