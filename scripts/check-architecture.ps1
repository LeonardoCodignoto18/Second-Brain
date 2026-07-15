$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "Project-Environment.ps1")

Write-Host "Checking architectural boundaries in $script:ProjectRoot"

$metadata = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$workspaceNames = @{}
foreach ($package in $metadata.packages) {
    $workspaceNames[$package.name] = $package
}

$allowed = @{
    "second-brain-contracts" = @()
    "second-brain-application" = @("second-brain-contracts")
    "second-brain-desktop" = @("second-brain-application", "second-brain-contracts")
}

foreach ($packageName in $allowed.Keys) {
    if (-not $workspaceNames.ContainsKey($packageName)) {
        throw "Required workspace package is missing: $packageName"
    }

    $actualWorkspaceDependencies = @(
        $workspaceNames[$packageName].dependencies |
            Where-Object { $workspaceNames.ContainsKey($_.name) } |
            ForEach-Object { $_.name }
    )

    foreach ($dependency in $actualWorkspaceDependencies) {
        if ($dependency -notin $allowed[$packageName]) {
            throw "Forbidden workspace dependency: $packageName -> $dependency"
        }
    }
}

$forbiddenContractDependencies = @("tauri", "rusqlite", "reqwest", "windows", "tokio")
$contract = $workspaceNames["second-brain-contracts"]
foreach ($dependency in $contract.dependencies.name) {
    if ($dependency -in $forbiddenContractDependencies) {
        throw "Contracts cannot depend on infrastructure/framework crate '$dependency'"
    }
}

$uiFiles = Get-ChildItem -Path (Join-Path $script:ProjectRoot "apps\desktop\src") -Recurse -File -Include *.ts,*.tsx
$forbiddenUiPatterns = @(
    "src-tauri",
    "crates/domains",
    "crates/infrastructure",
    "node:fs",
    "node:child_process"
)
foreach ($file in $uiFiles) {
    $contents = Get-Content -Raw -LiteralPath $file.FullName
    foreach ($pattern in $forbiddenUiPatterns) {
        if ($contents.Contains($pattern)) {
            throw "Forbidden UI dependency '$pattern' in $($file.FullName)"
        }
    }
}

Write-Host "Architecture checks passed."

