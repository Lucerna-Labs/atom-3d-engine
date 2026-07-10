$ErrorActionPreference = "Continue"
cargo run -p mm3e-orchestrator --example tpu_signals_real --release
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
