//! Probe: print the GPU adapter wgpu selects on this machine.
//! Run: cargo run -p mm3e-gpu --example gpu_probe --release

fn main() {
    match mm3e_gpu::adapter_info() {
        Ok(info) => println!("GPU adapter: {info}"),
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    }
}
