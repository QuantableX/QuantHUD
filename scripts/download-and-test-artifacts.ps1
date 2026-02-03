# QuantHUD - Download and Test Artifacts from GitHub Actions
# Requires: GitHub CLI (gh) - Install: winget install GitHub.cli

param(
    [string]$RunId = "latest"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "QuantHUD Artifact Downloader & Tester" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if gh CLI is installed
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: GitHub CLI (gh) is not installed!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Install with:" -ForegroundColor Yellow
    Write-Host "  winget install GitHub.cli" -ForegroundColor White
    Write-Host ""
    Write-Host "Or download from: https://cli.github.com/" -ForegroundColor White
    exit 1
}

# Create temp directory
$tempDir = Join-Path $env:TEMP "quanthud-artifacts"
if (Test-Path $tempDir) {
    Remove-Item $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

Write-Host "Downloading artifacts to: $tempDir" -ForegroundColor Green
Write-Host ""

# Download artifacts
Write-Host "Fetching latest workflow run..." -ForegroundColor Yellow
cd $PSScriptRoot\..

if ($RunId -eq "latest") {
    gh run download --dir $tempDir
} else {
    gh run download $RunId --dir $tempDir
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to download artifacts!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Testing Artifacts" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Test Windows artifact
$windowsArtifact = Get-ChildItem -Path $tempDir -Filter "QuantHUD-Windows" -Directory -ErrorAction SilentlyContinue
if ($windowsArtifact) {
    Write-Host "[Windows]" -ForegroundColor Green
    $exeFiles = Get-ChildItem -Path $windowsArtifact.FullName -Filter "*.exe" -Recurse
    foreach ($exe in $exeFiles) {
        Write-Host "  Found: $($exe.Name)" -ForegroundColor White
        Write-Host "  Size:  $([math]::Round($exe.Length / 1MB, 2)) MB" -ForegroundColor Gray
        Write-Host "  Path:  $($exe.FullName)" -ForegroundColor Gray
    }
    Write-Host ""
} else {
    Write-Host "[Windows] NOT FOUND" -ForegroundColor Red
    Write-Host ""
}

# Test macOS artifact
$macosArtifact = Get-ChildItem -Path $tempDir -Filter "QuantHUD-macOS" -Directory -ErrorAction SilentlyContinue
if ($macosArtifact) {
    Write-Host "[macOS]" -ForegroundColor Green
    $dmgFiles = Get-ChildItem -Path $macosArtifact.FullName -Filter "*.dmg" -Recurse
    foreach ($dmg in $dmgFiles) {
        Write-Host "  Found: $($dmg.Name)" -ForegroundColor White
        Write-Host "  Size:  $([math]::Round($dmg.Length / 1MB, 2)) MB" -ForegroundColor Gray
        Write-Host "  Path:  $($dmg.FullName)" -ForegroundColor Gray
    }
    Write-Host ""
} else {
    Write-Host "[macOS] NOT FOUND" -ForegroundColor Red
    Write-Host ""
}

# Test Linux artifact
$linuxArtifact = Get-ChildItem -Path $tempDir -Filter "QuantHUD-Linux" -Directory -ErrorAction SilentlyContinue
if ($linuxArtifact) {
    Write-Host "[Linux]" -ForegroundColor Green
    $debFiles = Get-ChildItem -Path $linuxArtifact.FullName -Filter "*.deb" -Recurse
    $appImageFiles = Get-ChildItem -Path $linuxArtifact.FullName -Filter "*.AppImage" -Recurse
    
    foreach ($deb in $debFiles) {
        Write-Host "  Found: $($deb.Name)" -ForegroundColor White
        Write-Host "  Size:  $([math]::Round($deb.Length / 1MB, 2)) MB" -ForegroundColor Gray
    }
    foreach ($appImage in $appImageFiles) {
        Write-Host "  Found: $($appImage.Name)" -ForegroundColor White
        Write-Host "  Size:  $([math]::Round($appImage.Length / 1MB, 2)) MB" -ForegroundColor Gray
    }
    Write-Host ""
} else {
    Write-Host "[Linux] NOT FOUND" -ForegroundColor Red
    Write-Host ""
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Artifacts location: $tempDir" -ForegroundColor Yellow
Write-Host ""
Write-Host "To test Windows installer:" -ForegroundColor White
if ($windowsArtifact -and $exeFiles) {
    $setupExe = $exeFiles | Where-Object { $_.Name -like "*setup.exe" } | Select-Object -First 1
    if ($setupExe) {
        Write-Host "  $($setupExe.FullName)" -ForegroundColor Gray
    }
}
Write-Host ""
Write-Host "To open artifacts folder:" -ForegroundColor White
Write-Host "  explorer `"$tempDir`"" -ForegroundColor Gray
Write-Host ""

# Ask if user wants to open folder
$response = Read-Host "Open artifacts folder? (y/n)"
if ($response -eq "y" -or $response -eq "Y") {
    explorer $tempDir
}

