#!/usr/bin/env bash
set -euo pipefail
if ! command -v spikard >/dev/null 2>&1; then
  printf '%s\n' 'Install the MCP-enabled CLI: cargo install spikard-cli --version 0.17.2 --locked' >&2
  exit 1
fi
if ! spikard mcp --help >/dev/null 2>&1; then
  printf '%s\n' 'The installed spikard CLI lacks MCP support; rebuild with the default mcp feature.' >&2
  exit 1
fi
exec spikard "$@"
