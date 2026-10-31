#!/usr/bin/env bash
#MISE description="Upgrade zig dependencies in every build.zig.zon (via `zig fetch --save`)"
set -euo pipefail

if ! command -v zig >/dev/null 2>&1; then
  echo "zig not installed; skipping"
  exit 0
fi
find . -name "build.zig.zon" \
  -not -path "./target/*" \
  -not -path "*/.build/*" \
  -not -path "*/zig-cache/*" \
  -not -path "*/zig-out/*" \
  -not -path "*/test_apps/zig/zig-pkg/*" \
  -print | while read -r zon; do
  urls="$({ grep '\.url' "$zon" || true; } | cut -d '=' -f 2 | cut -d '#' -f 1 | tr -d '" ,')"
  [ -z "$urls" ] && continue
  dir="$(dirname "$zon")"
  echo "Updating zig deps in $dir"
  for url in $urls; do
    (cd "$dir" && zig fetch --save "$url")
  done
done
