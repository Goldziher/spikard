#!/usr/bin/env bash
set -euo pipefail

# Update spikard Homebrew formula with the source archive URL and SHA256 for a new release.
#
# Usage:
#   TAG=v0.16.0 VERSION=0.16.0 \
#   TAP_DIR=/path/to/homebrew-tap \
#   ./update-homebrew-formula.sh

tag="${TAG:?TAG is required (e.g. v0.16.0)}"
version="${VERSION:?VERSION is required (e.g. 0.16.0)}"
tap_dir="${TAP_DIR:?TAP_DIR is required (path to homebrew-tap checkout)}"

formula="${tap_dir}/Formula/spikard.rb"

[[ -f "$formula" ]] || {
  echo "Missing $formula" >&2
  exit 1
}

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

# Source archive SHA: GitHub auto-generates the source tarball at /archive/${tag}.tar.gz.
# Download via curl since `gh release download` only fetches uploaded assets.
download_source_sha() {
  local url="https://github.com/Goldziher/spikard/archive/${tag}.tar.gz"
  echo "Downloading source archive from $url..." >&2
  curl -fsSL "$url" -o "$work_dir/source.tar.gz"
  shasum -a 256 "$work_dir/source.tar.gz" | awk '{print $1}'
}

source_sha=$(download_source_sha)

write_formula() {
  cat >"$formula" <<EOF
# typed: false
# frozen_string_literal: true

class Spikard < Formula
  desc "Rust-centric multi-language HTTP framework with polyglot bindings"
  homepage "https://github.com/Goldziher/spikard"
  version "${version}"
  url "https://github.com/Goldziher/spikard/archive/v#{version}.tar.gz"
  sha256 "${source_sha}"
  license "MIT"

  depends_on "pkg-config" => :build
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "crates/spikard-cli")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/spikard --version")
  end
end
EOF
}

write_formula

echo "Updated formula: $formula"
