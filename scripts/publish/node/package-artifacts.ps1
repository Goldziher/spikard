$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

pnpm exec napi artifacts --output-dir ./artifacts
if (-Not (Test-Path npm)) {
  throw "npm artifact directory missing"
}

tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
