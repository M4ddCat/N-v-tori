$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptDir

Write-Host "Building RSL linter..." -ForegroundColor Cyan

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust not found!" -ForegroundColor Red
    exit 1
}

cargo build --release

# Убрали эмодзи, чтобы не ломать кодировку
Write-Host "Success!" -ForegroundColor Green