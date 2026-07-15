$ErrorActionPreference = "Stop"

$script:ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$isHostedCi = $env:CI -eq "true"
if (-not $isHostedCi -and $script:ProjectRoot -ne "D:\Projetos\Second Brain") {
    throw "Second Brain OS must run from D:\Projetos\Second Brain; found $script:ProjectRoot"
}

$env:CARGO_HOME = Join-Path $script:ProjectRoot ".tooling\cargo-home"
$env:RUSTUP_HOME = Join-Path $script:ProjectRoot ".tooling\rustup"
$env:CARGO_TARGET_DIR = Join-Path $script:ProjectRoot "target"
$localToolchainBin = Join-Path $env:RUSTUP_HOME "toolchains\1.96.1-x86_64-pc-windows-msvc\bin"
if (Test-Path -LiteralPath $localToolchainBin) {
    $env:PATH = $localToolchainBin + [IO.Path]::PathSeparator + $env:PATH
}
$env:PNPM_HOME = Join-Path $script:ProjectRoot ".tooling\pnpm-home"
$env:PNPM_STORE_DIR = Join-Path $script:ProjectRoot ".tooling\pnpm-home\store"
$env:NPM_CONFIG_CACHE = Join-Path $script:ProjectRoot ".tooling\npm-cache"
$env:XDG_CACHE_HOME = Join-Path $script:ProjectRoot ".tooling\xdg-cache"
$env:XDG_CONFIG_HOME = Join-Path $script:ProjectRoot ".tooling\xdg-config"
$env:XDG_DATA_HOME = Join-Path $script:ProjectRoot ".tooling\xdg-data"
$env:TEMP = Join-Path $script:ProjectRoot ".tooling\temp"
$env:TMP = $env:TEMP

@(
    $env:CARGO_HOME,
    $env:RUSTUP_HOME,
    $env:PNPM_HOME,
    $env:PNPM_STORE_DIR,
    $env:NPM_CONFIG_CACHE,
    $env:XDG_CACHE_HOME,
    $env:XDG_CONFIG_HOME,
    $env:XDG_DATA_HOME,
    $env:TEMP,
    (Join-Path $script:ProjectRoot ".tooling\tsbuild")
) | ForEach-Object { New-Item -ItemType Directory -Force -Path $_ | Out-Null }

Set-Location -LiteralPath $script:ProjectRoot

