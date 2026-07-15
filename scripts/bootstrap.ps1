$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "Project-Environment.ps1")

Write-Host "Working directory: $script:ProjectRoot"

foreach ($command in @("cargo", "rustc", "node", "pnpm")) {
    if (-not (Get-Command $command -ErrorAction SilentlyContinue)) {
        throw "Required command '$command' was not found. See docs/engineering/development-environment.md."
    }
}

rustc --version
cargo --version
node --version
pnpm --version

pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw "pnpm install failed" }
cargo fetch --locked
if ($LASTEXITCODE -ne 0) { throw "cargo fetch failed" }

Write-Host "Bootstrap completed. Run scripts/check.ps1."

