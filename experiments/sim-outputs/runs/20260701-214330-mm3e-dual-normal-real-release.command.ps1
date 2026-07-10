$ErrorActionPreference = "Continue"
cargo run -p mm3e-orchestrator --example dual_normal_real --release
if ($null -ne $global:LASTEXITCODE) { exit $global:LASTEXITCODE }
