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

# Use a git source (tag + immutable revision) rather than GitHub's auto-generated
# /archive/${tag}.tar.gz: those tarballs are NOT byte-stable — GitHub regenerates
# them, so a sha256 captured here can differ from what `brew` downloads minutes
# later during bottle building ("Formula reports different checksum"). Pinning the
# commit revision needs no sha256 and is immutable.
revision="${REVISION:-$(git rev-parse HEAD^{commit})}"
[[ -n "$revision" ]] || { echo "Could not resolve git revision for $tag" >&2; exit 1; }

write_formula() {
  cat >"$formula" <<EOF
# typed: false
# frozen_string_literal: true

class Spikard < Formula
  desc "Rust-centric multi-language HTTP framework with polyglot bindings"
  homepage "https://github.com/Goldziher/spikard"
  version "${version}"
  url "https://github.com/Goldziher/spikard.git",
      tag:      "${tag}",
      revision: "${revision}"
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
