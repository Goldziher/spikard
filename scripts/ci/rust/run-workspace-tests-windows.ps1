param(
    [switch]$ExcludeRuby
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$args = @("--workspace")
if ($ExcludeRuby) {
    $args += @("--exclude", "spikard-rb")
}

cargo test @args
