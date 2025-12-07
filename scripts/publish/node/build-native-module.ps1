$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

# Map Rust target to napi platform directory name
$PlatformDir = switch ($env:TARGET) {
	"x86_64-apple-darwin"       { "darwin-x64" }
	"aarch64-apple-darwin"      { "darwin-arm64" }
	"x86_64-unknown-linux-gnu"  { "linux-x64-gnu" }
	"x86_64-pc-windows-msvc"    { "win32-x64-msvc" }
	default {
		Write-Error "Unknown target: $($env:TARGET)"
		exit 1
	}
}

$args = @('--release', '--target', $env:TARGET, '-o', "npm/$PlatformDir")
if ($env:USE_NAPI_CROSS -eq 'true') { $args += '--use-napi-cross' }
if ($env:USE_CROSS -eq 'true') { $args += '--use-cross' }

Write-Host "Building for target $($env:TARGET) -> npm/$PlatformDir"
pnpm exec napi build @args
