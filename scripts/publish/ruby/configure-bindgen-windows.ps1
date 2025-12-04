$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

if (-not $env:RI_DEVKIT) {
    throw "RI_DEVKIT environment variable is not set"
}

$msysPrefix = ridk exec bash -lc 'printf %s "$MSYSTEM_PREFIX"'
$msysPrefix = $msysPrefix.Trim()
$riDevkit = $env:RI_DEVKIT -replace '\\','/'
$sysroot = "$riDevkit$msysPrefix"
Add-Content -Path $env:GITHUB_ENV -Value "BINDGEN_EXTRA_CLANG_ARGS=--target=x86_64-pc-windows-gnu --sysroot=$sysroot"
