# package.ps1
#
# Packages relay for Windows into a clean .zip release bundle.
# Usage in PowerShell:
#   .\package.ps1

$ErrorActionPreference = "Stop"

$Version = "0.1.0"
$DistDir = "dist\relay-$Version"
$ZipFile = "relay-$Version-windows-x64.zip"

Write-Host "Building release binaries..." -ForegroundColor Cyan
cargo build --release

Write-Host "Preparing distribution folder..." -ForegroundColor Cyan
if (Test-Path "dist") { Remove-Item -Recurse -Force "dist" }
if (Test-Path $ZipFile) { Remove-Item -Force $ZipFile }

New-Item -ItemType Directory -Path "$DistDir\bin" -Force | Out-Null

Copy-Item "target\release\relay-server.exe" "$DistDir\bin\"
Copy-Item "target\release\relay-client.exe" "$DistDir\bin\"
Copy-Item "mdfiles\README.md" "$DistDir\"
Copy-Item "install.ps1" "$DistDir\"

Write-Host "Creating zip package: $ZipFile" -ForegroundColor Cyan
Compress-Archive -Path $DistDir -DestinationPath $ZipFile -Force

Remove-Item -Recurse -Force "dist"

Write-Host "Package ready: $ZipFile" -ForegroundColor Green
