$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")
Set-Location (Join-Path $RepoRoot "crates/spikard-node")

# Copy .node files from root into npm platform directories
$NodeFiles = Get-ChildItem -Path "." -Filter "*.node" -File
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

# napi artifacts organizes .node files from npm platform directories
# --output-dir . tells napi artifacts to look for .node files in current dir (default is ./artifacts)
pnpm exec napi artifacts --output-dir . --npm-dir ./npm
if (-Not (Test-Path npm)) {
  throw "npm artifact directory missing"
}

tar -czf "..\..\node-bindings-$($env:TARGET).tar.gz" -C . npm
