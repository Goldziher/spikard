#!/usr/bin/env bash
#MISE description="Check alef.toml's alef_version matches the alef dependency in every Cargo manifest"
set -euo pipefail

# ~keep Two independent sources of the alef version with nothing tying them together.
# They have drifted twice: v0.60.2 pinned an alef release that was never cut (404ing
# every CI job), and the same commit swapped the Cargo dep to a local filesystem path
# that broke `cargo metadata` for the whole workspace on any clean checkout.
pin=$(sed -n '/^\[workspace\]/,/^\[/{ s/^alef_version *= *"\(.*\)"/\1/p; }' alef.toml | head -1)
if [ -z "$pin" ]; then
  echo "alef:check-pin: no alef_version in alef.toml" >&2
  exit 1
fi
rc=0
for m in crates/spikard-alef/Cargo.toml crates/spikard-alef-ext/Cargo.toml crates/spikard-e2e-http/Cargo.toml; do
  dep=$(sed -n 's/^alef = "\(.*\)"$/\1/p' "$m" | head -1)
  if [ -z "$dep" ]; then
    echo "alef:check-pin: $m does not declare a plain versioned \`alef = \"<version>\"\` dependency" >&2
    echo "  found: $(grep -n '^alef =' "$m" || echo '<no alef dependency>')" >&2
    rc=1
  elif [ "$dep" != "$pin" ]; then
    echo "alef:check-pin: $m has alef = \"$dep\" but alef.toml pins $pin" >&2
    rc=1
  fi
done
if [ "$rc" -eq 0 ]; then
  echo "alef:check-pin: all manifests agree on alef $pin"
fi
exit "$rc"
