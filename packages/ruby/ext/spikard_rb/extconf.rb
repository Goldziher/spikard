# frozen_string_literal: true

require 'mkmf'
require 'rb_sys/mkmf'

default_profile = ENV.fetch('CARGO_PROFILE', 'release')

create_rust_makefile('spikard_rb') do |config|
  config.profile = default_profile.to_sym
  # Only use --locked in development to prevent lockfile updates
  # Release builds need to update Cargo.lock after version bumps
  config.extra_cargo_args = ['--locked'] unless default_profile == 'release'
end
