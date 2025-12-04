param(
    [Parameter(Mandatory = $true)]
    [string]$RubyPrefix
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$includeRoot = "$RubyPrefix\include\ruby-3.2.0"
$compat = "${env:GITHUB_WORKSPACE}\packages\ruby\ext\spikard_rb\native\include\msvc_compat"
$includeRoot = $includeRoot -replace '\\','/'
$compatForward = $compat -replace '\\','/'
$extraInclude = "C:/msys64/ucrt64/include"
$extra = "-I$includeRoot -I$compatForward -I$extraInclude -fms-extensions -fstack-protector-strong -fno-omit-frame-pointer -fno-fast-math"
Add-Content -Path $env:GITHUB_ENV -Value "BINDGEN_EXTRA_CLANG_ARGS=$extra"
Add-Content -Path $env:GITHUB_ENV -Value "BINDGEN_EXTRA_CLANG_ARGS_x86_64-pc-windows-msvc=$extra"
Add-Content -Path $env:GITHUB_ENV -Value "BINDGEN_EXTRA_CLANG_ARGS_x86_64_pc_windows_msvc=$extra"
