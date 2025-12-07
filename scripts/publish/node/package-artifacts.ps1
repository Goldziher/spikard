$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

# List .node files in current directory before copying
Write-Host "=== .node files in current directory ==="
$NodeFiles = Get-ChildItem -Path "." -Filter "*.node" -File -ErrorAction SilentlyContinue
if ($NodeFiles) {
  $NodeFiles | ForEach-Object { Write-Host $_.FullName }
} else {
  Write-Host "No .node files in current directory"
}

# Copy .node files from root into npm platform directories
foreach ($NodeFile in $NodeFiles) {
  $Filename = $NodeFile.Name
  $Platform = $Filename -replace '^spikard-node\.', '' -replace '\.node$', ''
  $DestDir = Join-Path "npm" $Platform

  if (Test-Path $DestDir) {
    Write-Host "Copying $Filename to $DestDir\"
    Copy-Item -Path $NodeFile.FullName -Destination $DestDir -Force
  } else {
    Write-Host "::warning::Platform directory $DestDir not found for $Filename"
  }
}

# List contents of npm directories after copying
Write-Host "=== Contents of npm directories after copy ==="
Get-ChildItem -Path "npm" -Directory | ForEach-Object {
  Write-Host "Directory: $($_.FullName)"
  Get-ChildItem -Path $_.FullName
}

# napi artifacts organizes .node files from npm platform directories
# --output-dir . tells napi artifacts to look for .node files in current dir (default is ./artifacts)
pnpm exec napi artifacts --output-dir . --npm-dir ./npm

# List contents of npm directories after napi artifacts
Write-Host "=== Contents of npm directories after napi artifacts ==="
Get-ChildItem -Path "npm" -Directory | ForEach-Object {
  Write-Host "Directory: $($_.FullName)"
  Get-ChildItem -Path $_.FullName
}

# Verify npm directory was created
if (-Not (Test-Path npm)) {
  throw "npm artifact directory missing"
}

# Verify .node files exist in npm directories
$Missing = $false
Get-ChildItem -Path "npm" -Directory | ForEach-Object {
  $NodeBinaries = Get-ChildItem -Path $_.FullName -Filter "*.node" -ErrorAction SilentlyContinue
  if (-Not $NodeBinaries) {
    Write-Host "::error::Missing .node file in $($_.FullName) after packaging"
    $Missing = $true
  }
}

if ($Missing) {
  throw "Some npm directories are missing .node files before tarball creation"
}

tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
