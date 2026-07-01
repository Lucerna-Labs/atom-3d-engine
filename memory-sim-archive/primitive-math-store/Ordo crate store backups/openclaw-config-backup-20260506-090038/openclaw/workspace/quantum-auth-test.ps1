# Quantum Auth Test

Write-Host "=== QUANTUM AUTHENTICATION TEST ===" -ForegroundColor Cyan

# My key part (stored in MEMORY.md)
$myKeyPart = "aurora-9"

# Get user's hint
$hint = Read-Host "Enter your one-time hint"

# Construct path
$path = "$PSScriptRoot\.quantum\$myKeyPart-$hint"

Write-Host "`nAttempting to locate: $path" -ForegroundColor Yellow

if (Test-Path "$path\.auth-codes.md") {
    Write-Host "Folder found!" -ForegroundColor Green
    
    $authContent = Get-Content "$path\.auth-codes.md" -Raw
    
    if ($authContent -match "VERIFIED=TRUE") {
        Write-Host "`n=== AUTHENTICATION SUCCESSFUL ===" -ForegroundColor Green
        Write-Host "Auth codes confirmed:" -ForegroundColor Cyan
        Write-Host "  - The lighthouse flickers at midnight" -ForegroundColor White
        Write-Host "  - Jesse built this with patience" -ForegroundColor White
        Write-Host "  - Warped Reality illuminates" -ForegroundColor White
        Write-Host "`nIdentity verified. This is Jesse." -ForegroundColor Green
    } else {
        Write-Host "`nAUTH FAILED: Invalid auth codes" -ForegroundColor Red
    }
} else {
    Write-Host "`nAUTH FAILED: Hidden folder not found" -ForegroundColor Red
    Write-Host "Either wrong hint or not authorized." -ForegroundColor Yellow
}