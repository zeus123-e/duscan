param(
  [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"

$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$releaseDir = Join-Path $projectRoot "release"
$installerPath = Join-Path $releaseDir "duscan-v$Version-x86_64-pc-windows-msvc.msi"
$checksumPath = "$installerPath.sha256"
$wxsPath = Join-Path $projectRoot "installer\duscan.wxs"
$wixObjPath = Join-Path $releaseDir "duscan.wixobj"
$wixPdbPath = Join-Path $releaseDir "duscan-v$Version-x86_64-pc-windows-msvc.wixpdb"
$exePath = Join-Path $projectRoot "target\release\duscan.exe"
$readmePath = Join-Path $projectRoot "README.md"
$licensePath = Join-Path $projectRoot "installer\license.rtf"

New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

cargo build --release

if (-not (Test-Path -LiteralPath $exePath)) {
  throw "Release binary not found: $exePath"
}

$wixRoot = Join-Path $env:LOCALAPPDATA "tauri\WixTools314"
$candle = Join-Path $wixRoot "candle.exe"
$light = Join-Path $wixRoot "light.exe"

if (-not (Test-Path -LiteralPath $candle) -or -not (Test-Path -LiteralPath $light)) {
  throw "WiX tools not found at $wixRoot. Build a Tauri MSI once or install WiX Toolset 3."
}

foreach ($path in @($installerPath, $checksumPath, $wixObjPath, $wixPdbPath)) {
  if (Test-Path -LiteralPath $path) {
    Remove-Item -LiteralPath $path -Force
  }
}

& $candle `
  -nologo `
  "-dVersion=$Version" `
  "-dExeFile=$exePath" `
  "-dReadmeFile=$readmePath" `
  "-dLicenseFile=$licensePath" `
  -out $wixObjPath `
  $wxsPath

if ($LASTEXITCODE -ne 0) {
  throw "candle.exe failed with exit code $LASTEXITCODE"
}

& $light `
  -nologo `
  -ext (Join-Path $wixRoot "WixUIExtension.dll") `
  -sval `
  -pdbout $wixPdbPath `
  -out $installerPath `
  $wixObjPath

if ($LASTEXITCODE -ne 0) {
  throw "light.exe failed with exit code $LASTEXITCODE"
}

foreach ($path in @($wixObjPath, $wixPdbPath)) {
  if (Test-Path -LiteralPath $path) {
    Remove-Item -LiteralPath $path -Force
  }
}

$hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $installerPath).Hash.ToLowerInvariant()
Set-Content -LiteralPath $checksumPath -Value "$hash  $(Split-Path -Leaf $installerPath)" -Encoding ASCII

Write-Host "MSI created at $installerPath"
Write-Host "Checksum created at $checksumPath"
