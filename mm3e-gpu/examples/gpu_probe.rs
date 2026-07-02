//! Probe: list every GPU adapter wgpu can see, then show which one the renderer opens.
//!
//! Selection: set `MM3E_GPU_ADAPTER` to a case-insensitive substring of the descriptor
//! (e.g. `arc`, `nvidia`, `5070`, `dx12`) to pick a card deliberately — how a heterogeneous
//! fleet validates the renderer on the GPU it means to, not whatever wgpu ranks first.
//!
//! Run: cargo run -p mm3e-gpu --example gpu_probe --release
//!      MM3E_GPU_ADAPTER=arc cargo run -p mm3e-gpu --example gpu_probe --release

fn main() {
    let all = mm3e_gpu::list_adapters();
    if all.is_empty() {
        eprintln!("no GPU adapters visible to wgpu");
        std::process::exit(1);
    }
    println!("available adapters:");
    for a in &all {
        println!("  {a}");
    }
    match mm3e_gpu::adapter_info() {
        Ok(info) => {
            let via = std::env::var("MM3E_GPU_ADAPTER").ok().filter(|v| !v.trim().is_empty());
            match via {
                Some(f) => println!("selected (MM3E_GPU_ADAPTER='{f}'): {info}"),
                None => println!("selected (default high-performance): {info}"),
            }
        }
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    }
}
