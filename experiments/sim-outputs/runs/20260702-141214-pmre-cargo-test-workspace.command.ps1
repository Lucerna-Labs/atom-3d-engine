$ErrorActionPreference = "Continue"
cargo test --workspace
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
