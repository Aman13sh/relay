# install.ps1
#
# One-shot installer for relay on Windows (PowerShell).
# Installs relay-server.exe and relay-client.exe into user's local bin.
#
# Usage:
#   .\install.ps1

$ErrorActionPreference = "Stop"

$InstallDir = "$env:LOCALAPPDATA\Programs\relay\bin"

Write-Host "Installing relay to: $InstallDir" -ForegroundColor Cyan

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

if (Test-Path "bin\relay-server.exe") {
    Copy-Item "bin\relay-server.exe" "$InstallDir\" -Force
    Copy-Item "bin\relay-client.exe" "$InstallDir\" -Force
} elseif (Test-Path "target\release\relay-server.exe") {
    Copy-Item "target\release\relay-server.exe" "$InstallDir\" -Force
    Copy-Item "target\release\relay-client.exe" "$InstallDir\" -Force
} else {
    Write-Error "Could not find relay-server.exe and relay-client.exe. Run cargo build --release first."
    exit 1
}

# Add to user PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to user PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
}

Write-Host "Installed successfully!" -ForegroundColor Green
Write-Host "You can now run:" -ForegroundColor Green
Write-Host "  relay-server"
Write-Host "  relay-client"
