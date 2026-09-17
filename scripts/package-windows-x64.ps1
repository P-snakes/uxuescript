[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$cargoTomlPath = Join-Path $root "src-tauri\Cargo.toml"
$cargoToml = Get-Content -Path $cargoTomlPath -Raw
$versionMatch = [regex]::Match($cargoToml, '(?m)^version\s*=\s*"([^"]+)"')

if (-not $versionMatch.Success) {
    throw "Cannot find package.version in $cargoTomlPath"
}

$version = $versionMatch.Groups[1].Value
$exePath = Join-Path $root "src-tauri\target\release\uxuescript.exe"
$artifactPath = Join-Path (Split-Path -Parent $exePath) "uxs-$version-win-x64.exe"

if (-not (Test-Path -LiteralPath $exePath -PathType Leaf)) {
    throw "Release executable not found: $exePath"
}

Move-Item -LiteralPath $exePath -Destination $artifactPath -Force
Write-Output "Created $artifactPath"
