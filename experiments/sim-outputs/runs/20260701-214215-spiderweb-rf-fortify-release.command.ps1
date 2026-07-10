$ErrorActionPreference = "Continue"
cargo run -p spiderweb-rf --example fortify --release
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
