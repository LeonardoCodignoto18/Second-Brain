$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "Project-Environment.ps1")

function Invoke-Checked {
    param([scriptblock]$Command)
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "External command failed with exit code ${LASTEXITCODE}: $Command"
    }
}

Write-Host "Working directory: $script:ProjectRoot"

& (Join-Path $PSScriptRoot "check-architecture.ps1")
Invoke-Checked { cargo fmt --all -- --check }
Invoke-Checked { cargo clippy --workspace --all-targets --all-features --locked -- -D warnings }
Invoke-Checked { cargo test --workspace --all-targets --all-features --locked }
Invoke-Checked { pnpm format:check }
Invoke-Checked { pnpm check }
Invoke-Checked { pnpm test }
Invoke-Checked { pnpm build }