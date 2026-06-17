param(
  [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"

$projectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$releaseDir = Join-Path $projectRoot "release"
$installerPath = Join-Path $releaseDir "duscan-v$Version-x86_64-pc-windows-msvc-setup.exe"
$checksumPath = "$installerPath.sha256"
$scriptPath = Join-Path $projectRoot "installer\duscan.nsi"
$exePath = Join-Path $projectRoot "target\release\duscan.exe"

New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

cargo build --release

if (-not (Test-Path -LiteralPath $exePath)) {
  throw "Release binary not found: $exePath"
}

$makensisCandidates = @(
  (Join-Path $env:LOCALAPPDATA "tauri\NSIS\makensis.exe"),
  (Join-Path $env:LOCALAPPDATA "tauri\NSIS\Bin\makensis.exe")
)

$makensisCommand = Get-Command makensis.exe -ErrorAction SilentlyContinue
if ($makensisCommand) {
  $makensis = $makensisCommand.Source
} else {
  $makensis = $makensisCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
}

if (-not $makensis) {
  throw "makensis.exe not found. Install NSIS or build Downlink once so Tauri downloads NSIS."
}

& $makensis `
  "/DVERSION=$Version" `
  "/DOUTFILE=$installerPath" `
  "/DEXE_FILE=$exePath" `
  "/DREADME_FILE=$(Join-Path $projectRoot "README.md")" `
  "/DADD_PATH_SCRIPT=$(Join-Path $projectRoot "installer\add-to-user-path.ps1")" `
  "/DREMOVE_PATH_SCRIPT=$(Join-Path $projectRoot "installer\remove-from-user-path.ps1")" `
  $scriptPath

$hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $installerPath).Hash.ToLowerInvariant()
Set-Content -LiteralPath $checksumPath -Value "$hash  $(Split-Path -Leaf $installerPath)" -Encoding ASCII

Write-Host "Installer created at $installerPath"
Write-Host "Checksum created at $checksumPath"
