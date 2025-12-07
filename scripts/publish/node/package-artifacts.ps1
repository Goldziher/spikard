$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

# napi build --platform already creates npm/platform-name/ directories with .node files
# We just need to verify they exist and package them

# Verify npm directory was created by napi build --platform
if (-Not (Test-Path npm)) {
  Write-Host "::error::npm directory not found (napi build --platform should create it)"
  Write-Host "Current directory: $(Get-Location)"
  Get-ChildItem -Force
  throw "npm directory not found"
}

# List and verify contents of npm directories
Write-Host "=== Verifying npm platform directories ==="
$Missing = $false
$PlatformCount = 0

Get-ChildItem -Path "npm" -Directory | ForEach-Object {
  $PlatformCount++
  Write-Host "Checking directory: $($_.FullName)"

  $NodeBinaries = Get-ChildItem -Path $_.FullName -Filter "*.node" -ErrorAction SilentlyContinue
  if (-Not $NodeBinaries) {
    Write-Host "::error::Missing .node file in $($_.FullName)"
    Get-ChildItem -Path $_.FullName -Force
    $Missing = $true
  } else {
    Write-Host "✓ Found $($NodeBinaries.Count) .node file(s) in $($_.FullName)"
    $NodeBinaries | ForEach-Object {
      Write-Host "  $($_.Name) ($([math]::Round($_.Length/1KB, 2)) KB)"
    }
  }
}

if ($PlatformCount -eq 0) {
  Write-Host "::error::No platform directories found in npm/"
  Get-ChildItem -Path "npm" -Force
  throw "No platform directories found"
}

if ($Missing) {
  throw "Some npm directories are missing .node files"
}

Write-Host "=== All platforms verified, creating tarball ==="
tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
Write-Host "✓ Created tarball: node-bindings-$($env:TARGET).tar.gz"
