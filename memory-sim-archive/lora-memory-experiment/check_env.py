import subprocess
result = subprocess.run(["pip", "list"], capture_output=True, text=True)
packages = result.stdout
has_torch = "torch" in packages
has_transformers = "transformers" in packages
has_peft = "peft" in packages
has_bitsandbytes = "bitsandbytes" in packages
has_trl = "trl" in packages

print(f"torch: {has_torch}")
print(f"transformers: {has_transformers}")
print(f"peft: {has_peft}")
print(f"bitsandbytes: {has_trl}")
print(f"trl: {has_trl}")

if has_torch:
    import torch
    print(f"PyTorch version: {torch.__version__}")
    print(f"CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        device_name = torch.cuda.get_device_name(0)
        vram = torch.cuda.get_device_properties(0).total_mem / 1e9
        print(f"GPU: {device_name}")
        print(f"VRAM: {vram:.1f} GB")