$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "Project-Environment.ps1")

Write-Host "Working directory: $script:ProjectRoot"
pnpm dev

