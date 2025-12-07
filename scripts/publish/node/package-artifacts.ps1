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
    Write-Host "::error::Unknown target: $($env:TARGET)"
    throw "Unknown target: $($env:TARGET)"
  }
}

# Verify npm directory was created by napi build --platform
if (-Not (Test-Path npm)) {
  Write-Host "::error::npm directory not found (napi build --platform should create it)"
  Write-Host "Current directory: $(Get-Location)"
  Get-ChildItem -Force
  throw "npm directory not found"
}

# Verify the platform-specific directory exists and contains a .node file
Write-Host "=== Verifying npm/$PlatformDir for target $($env:TARGET) ==="
$PlatformPath = "npm\$PlatformDir"
if (-Not (Test-Path $PlatformPath)) {
  Write-Host "::error::Platform directory $PlatformPath not found"
  Write-Host "Available directories:"
  Get-ChildItem -Path "npm" -Force
  throw "Platform directory not found"
}

$NodeBinaries = @(Get-ChildItem -Path $PlatformPath -Filter "*.node" -ErrorAction SilentlyContinue)
if ($NodeBinaries.Count -eq 0) {
  Write-Host "::error::No .node file found in $PlatformPath"
  Get-ChildItem -Path $PlatformPath -Force
  throw "No .node file found"
}

Write-Host "✓ Found $($NodeBinaries.Count) .node file(s) in $PlatformPath"
$NodeBinaries | ForEach-Object {
  Write-Host "  $($_.Name) ($([math]::Round($_.Length/1KB, 2)) KB)"
}

Write-Host "=== Creating tarball for $($env:TARGET) ==="
tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
Write-Host "✓ Created tarball: node-bindings-$($env:TARGET).tar.gz"
