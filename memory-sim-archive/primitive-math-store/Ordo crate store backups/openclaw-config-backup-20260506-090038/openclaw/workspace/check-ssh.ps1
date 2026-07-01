Get-ChildItem -Path "$env:USERPROFILE\.ssh" -File | ForEach-Object {
    Write-Host "--- $($_.Name) ---"
    Get-Content $_.FullName | Select-Object -First 3
}