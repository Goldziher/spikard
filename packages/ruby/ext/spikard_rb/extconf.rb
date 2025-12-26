# frozen_string_literal: true

require 'mkmf'
require 'rb_sys/mkmf'

default_profile = ENV.fetch('CARGO_PROFILE', 'release')

create_rust_makefile('spikard_rb') do |config|
  config.profile = default_profile.to_sym
  # Use --locked to prevent Cargo from updating the committed lockfile
  # This avoids lockfile collision errors with vendored crates in CI
  config.extra_cargo_args = ['--locked']
end
