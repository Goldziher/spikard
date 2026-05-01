import Config

# Force the spikard NIF to compile locally instead of downloading a precompiled
# binary from GitHub Releases. The release artifact for the current dev version
# (0.14.0) doesn't exist yet — building locally avoids the 404.
config :rustler_precompiled, :force_build, spikard: true
