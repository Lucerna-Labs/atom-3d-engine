# Scan for cookies, trackers, and telemetry

Write-Host "=== BROWSER COOKIES ===" -ForegroundColor Cyan

# Chrome cookies
$chromeCookies = "$env:LOCALAPPDATA\Google\Chrome\User Data\Default\Network\Cookies"
if (Test-Path $chromeCookies) {
    Write-Host "`nChrome cookie database found:" -ForegroundColor Yellow
    Write-Host "  $chromeCookies"
    Write-Host "  Size: $((Get-Item $chromeCookies).Length) bytes"
}

# Edge cookies
$edgeCookies = "$env:LOCALAPPDATA\Microsoft\Edge\User Data\Default\Network\Cookies"
if (Test-Path $edgeCookies) {
    Write-Host "`nEdge cookie database found:" -ForegroundColor Yellow
    Write-Host "  $edgeCookies"
    Write-Host "  Size: $((Get-Item $edgeCookies).Length) bytes"
}

# Firefox cookies
$firefoxProfiles = "$env:APPDATA\Mozilla\Firefox\Profiles"
if (Test-Path $firefoxProfiles) {
    $ffCookies = Get-ChildItem $firefoxProfiles -Recurse -Filter "cookies.sqlite" -ErrorAction SilentlyContinue
    if ($ffCookies) {
        Write-Host "`nFirefox cookie databases found:" -ForegroundColor Yellow
        foreach ($c in $ffCookies) {
            Write-Host "  $($c.FullName) ($($c.Length) bytes)"
        }
    }
}

# Brave cookies
$braveCookies = "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\User Data\Default\Network\Cookies"
if (Test-Path $braveCookies) {
    Write-Host "`nBrave cookie database found:" -ForegroundColor Yellow
    Write-Host "  $braveCookies"
}

Write-Host "`n=== WINDOWS TELEMETRY ===" -ForegroundColor Cyan

# Check telemetry services
$telemetryServices = Get-Service | Where-Object { $_.Name -match "diag|track|telem|report|dmwappush" } -ErrorAction SilentlyContinue
if ($telemetryServices) {
    Write-Host "`nTelemetry-related services:" -ForegroundColor Yellow
    foreach ($s in $telemetryServices) {
        Write-Host "  $($s.Name) - $($s.DisplayName) [$($s.Status)]"
    }
} else {
    Write-Host "`nNo telemetry services found running."
}

# Check scheduled tasks
$telemetryTasks = Get-ScheduledTask -ErrorAction SilentlyContinue | Where-Object { 
    $_.TaskName -match "telemetry|diag|CEIP|CustomerExp|Microsoft\\Windows\\Application Experience|Microsoft\\Windows\\Windows Error Reporting" 
}
if ($telemetryTasks) {
    Write-Host "`nTelemetry scheduled tasks:" -ForegroundColor Yellow
    foreach ($t in $telemetryTasks) {
        Write-Host "  $($t.TaskName) [$($t.State)]"
    }
}

# Check Windows data collection registry
Write-Host "`nWindows Data Collection Settings:" -ForegroundColor Yellow
$dataCollection = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection" -ErrorAction SilentlyContinue
if ($dataCollection) {
    Write-Host "  AllowTelemetry: $($dataCollection.AllowTelemetry)"
}

Write-Host "`n=== TRACKING APPS ===" -ForegroundColor Cyan

# Check for tracking/bloatware apps
$trackingApps = Get-AppxPackage | Where-Object { 
    $_.Name -match "xbox|bing|msn|skype|mail|people|maps|camera|photos|groove|solitaire|messaging|feedback|clipchamp|zune|oneplace|gethelp|getstarted|windowsmaps|yourphone|mixedreality|3dviewer|officeonenote|bingsports|bingnews|bingweather" 
} -ErrorAction SilentlyContinue
if ($trackingApps) {
    Write-Host "`nTracking/bloatware apps installed:" -ForegroundColor Yellow
    $trackingApps | ForEach-Object { Write-Host "  $($_.Name)" }
} else {
    Write-Host "`nNo common tracking apps found."
}

Write-Host "`n=== BACKGROUND APPS (can track) ===" -ForegroundColor Cyan
$bgApps = Get-ItemProperty "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications" -ErrorAction SilentlyContinue
if ($bgApps) {
    Write-Host "  GlobalUserDisabled: $($bgApps.GlobalUserDisabled)"
}

Write-Host "`n=== ADVERTISING ID ===" -ForegroundColor Cyan
$adId = Get-ItemProperty "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo" -ErrorAction SilentlyContinue
if ($adId) {
    Write-Host "  Enabled: $($adId.Enabled)"
}

Write-Host "`nScan complete." -ForegroundColor Green