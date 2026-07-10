$ErrorActionPreference = "Continue"
cargo run -p spiderweb-rf --example chaos --release
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
