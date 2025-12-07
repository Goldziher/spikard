$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

$args = @('--platform', '--release', '--target', $env:TARGET, '-o', '.')
if ($env:USE_NAPI_CROSS -eq 'true') { $args += '--use-napi-cross' }
if ($env:USE_CROSS -eq 'true') { $args += '--use-cross' }

pnpm exec napi build @args
