# Sign the debug/release Tauri executable with the local dev certificate.
# Usage: powershell -ExecutionPolicy Bypass -File scripts/sign-dev.ps1 [debug|release]

param(
    [string]$Profile = "debug"
)

$exePath = "src-tauri\target\$Profile\quanthud.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "ERROR: $exePath not found. Build first." -ForegroundColor Red
    exit 1
}

$cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert |
    Where-Object { $_.Subject -eq 'CN=QuantHUD Dev' } |
    Select-Object -First 1

if (-not $cert) {
    Write-Host "No 'QuantHUD Dev' code-signing certificate found. Creating one..." -ForegroundColor Yellow
    $cert = New-SelfSignedCertificate `
        -Type CodeSigningCert `
        -Subject 'CN=QuantHUD Dev' `
        -CertStoreLocation Cert:\CurrentUser\My `
        -NotAfter (Get-Date).AddYears(5)

    # Trust it
    $store = New-Object System.Security.Cryptography.X509Certificates.X509Store('Root','CurrentUser')
    $store.Open('ReadWrite')
    $store.Add($cert)
    $store.Close()
    Write-Host "Certificate created and trusted." -ForegroundColor Green
}

Write-Host "Signing $exePath ..." -ForegroundColor Cyan
Set-AuthenticodeSignature -FilePath $exePath -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
Write-Host "Done." -ForegroundColor Green

