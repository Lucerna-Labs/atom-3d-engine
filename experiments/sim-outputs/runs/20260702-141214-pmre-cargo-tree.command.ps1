$ErrorActionPreference = "Continue"
cargo tree
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
