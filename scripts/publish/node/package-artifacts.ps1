$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

# --output-dir . tells napi artifacts to look for .node files in current dir (default is ./artifacts)
pnpm exec napi artifacts --output-dir . --npm-dir ./npm
if (-Not (Test-Path npm)) {
  throw "npm artifact directory missing"
}

tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
