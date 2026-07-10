param(
    [Parameter(Mandatory = $true)]
    [string]$Name,

    [string]$Category = "sim",

    [string]$WorkingDirectory = (Get-Location).Path,

    [Parameter(Mandatory = $true, ValueFromRemainingArguments = $true)]
    [string[]]$Command
)

$ErrorActionPreference = "Stop"

$outputRoot = Join-Path $PSScriptRoot "sim-outputs"
$runRoot = Join-Path $outputRoot "runs"
New-Item -ItemType Directory -Force -Path $runRoot | Out-Null

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$safeName = ($Name -replace "[^A-Za-z0-9._-]+", "-").Trim("-")
if ([string]::IsNullOrWhiteSpace($safeName)) {
    $safeName = "sim-run"
}

$logPath = Join-Path $runRoot "$stamp-$safeName.log"
$manifestPath = Join-Path $outputRoot "manifest.tsv"
$commandLine = $Command -join " "

function Add-LogLine {
    param([string]$Line)
    $Line | Out-File -LiteralPath $logPath -Encoding utf8 -Append
}

$gitHead = ""
$gitStatus = ""
try {
    $gitHead = (git -C $PSScriptRoot rev-parse HEAD 2>$null)
    $gitStatus = (git -C $PSScriptRoot status --short 2>$null) -join " | "
} catch {
    $gitHead = "unavailable"
    $gitStatus = "unavailable"
}

Add-LogLine "# MM3E simulation run archive"
Add-LogLine "name: $Name"
Add-LogLine "category: $Category"
Add-LogLine "local_time: $(Get-Date -Format o)"
Add-LogLine "utc_time: $((Get-Date).ToUniversalTime().ToString('o'))"
Add-LogLine "machine: $env:COMPUTERNAME"
Add-LogLine "user: $env:USERNAME"
Add-LogLine "working_directory: $WorkingDirectory"
Add-LogLine "command: $commandLine"
Add-LogLine "git_head: $gitHead"
Add-LogLine "git_status_short: $gitStatus"
Add-LogLine "powershell: $($PSVersionTable.PSVersion)"
Add-LogLine ""
Add-LogLine "## Raw stdout"

Push-Location -LiteralPath $WorkingDirectory
try {
    $commandScript = Join-Path $runRoot "$stamp-$safeName.command.ps1"
    $stdoutPath = Join-Path $runRoot "$stamp-$safeName.stdout.txt"
    $stderrPath = Join-Path $runRoot "$stamp-$safeName.stderr.txt"
    @(
        '$ErrorActionPreference = "Continue"'
        $commandLine
        'if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }'
    ) | Out-File -LiteralPath $commandScript -Encoding utf8

    $quotedCommandScript = '"' + ($commandScript -replace '"', '\"') + '"'
    $proc = Start-Process `
        -FilePath "powershell.exe" `
        -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File $quotedCommandScript" `
        -WorkingDirectory $WorkingDirectory `
        -RedirectStandardOutput $stdoutPath `
        -RedirectStandardError $stderrPath `
        -WindowStyle Hidden `
        -Wait `
        -PassThru
    $exitCode = $proc.ExitCode

    if (Test-Path -LiteralPath $stdoutPath) {
        Get-Content -LiteralPath $stdoutPath | Tee-Object -FilePath $logPath -Append
    }
    Add-LogLine ""
    Add-LogLine "## Raw stderr"
    if (Test-Path -LiteralPath $stderrPath) {
        Get-Content -LiteralPath $stderrPath | Tee-Object -FilePath $logPath -Append
    }
} finally {
    Pop-Location
}

Add-LogLine ""
Add-LogLine "exit_code: $exitCode"

$sha256 = (Get-FileHash -LiteralPath $logPath -Algorithm SHA256).Hash
Add-LogLine "sha256_pre_manifest: $sha256"

if (!(Test-Path -LiteralPath $manifestPath)) {
    "timestamp`tcategory`tname`texit_code`tsha256`tpath`tcommand" |
        Out-File -LiteralPath $manifestPath -Encoding utf8
}

("$stamp`t$Category`t$Name`t$exitCode`t$sha256`t$logPath`t$commandLine") |
    Out-File -LiteralPath $manifestPath -Encoding utf8 -Append

Write-Host "Archived simulation output:"
Write-Host $logPath
Write-Host "SHA256: $sha256"

exit $exitCode
