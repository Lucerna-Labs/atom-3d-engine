# Fable 3D Engine Handoff

This is a scoped copy of Jesse's MM3E 3D primitive math renderer from:

C:\Projects\3D Primitve math engine

Included:

- mm3e-kit
- mm3e-orchestrator
- mm3e-gpu
- Cargo workspace files
- core architecture/docs/license/config
- sample scene.mm3e
- renderer report

Excluded intentionally:

- experiments/
- sim-output archives
- memory-sim-archive/
- primitive-math-store / large primitive library
- target/ build outputs
- root generated BMP/PNG renders
- .git and .claude metadata
- kernel Os-spiderwebBus side project

Suggested first checks:

```powershell
cargo test --workspace
cargo run -p mm3e-orchestrator --example spheres --release
cargo run -p mm3e-gpu --example gpu_probe --release
```

Dependency boundary:

- mm3e-kit and mm3e-orchestrator are the core CPU engine.
- mm3e-gpu is the optional GPU/WGSL backend.

