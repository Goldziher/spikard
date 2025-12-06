param(
    [string]$ArtifactDir = "cli-artifact"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

& "$ArtifactDir/spikard.exe" --version
& "$ArtifactDir/spikard.exe" --help
