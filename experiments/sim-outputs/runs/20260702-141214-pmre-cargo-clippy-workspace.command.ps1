$ErrorActionPreference = "Continue"
cargo clippy --workspace --all-targets -- -D warnings
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
