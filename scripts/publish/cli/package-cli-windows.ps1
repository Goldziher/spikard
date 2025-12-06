$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

param(
    [Parameter(Mandatory = $true)]
    [string]$Target
)

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot ".." ".." "..")

$stage = Join-Path $RepoRoot "spikard-cli-$Target"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item (Join-Path $RepoRoot "target/$Target/release/spikard.exe") $stage
if (Test-Path "$RepoRoot/LICENSE") { Copy-Item "$RepoRoot/LICENSE" $stage }
if (Test-Path "$RepoRoot/README.md") { Copy-Item "$RepoRoot/README.md" $stage }
Compress-Archive -Path "$stage/*" -DestinationPath "$stage.zip" -Force
Remove-Item -Recurse -Force $stage
