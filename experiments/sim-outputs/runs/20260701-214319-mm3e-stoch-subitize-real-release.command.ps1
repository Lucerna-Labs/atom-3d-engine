$ErrorActionPreference = "Continue"
cargo run -p mm3e-orchestrator --example stoch_subitize_real --release
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
