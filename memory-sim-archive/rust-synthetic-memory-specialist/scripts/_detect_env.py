import importlib, sys
print("EXE:", sys.executable)
print("PY :", sys.version.split()[0])
for p in ['torch', 'transformers', 'transformer_lens', 'accelerate', 'safetensors', 'huggingface_hub', 'nnsight']:
    try:
        m = importlib.import_module(p)
        print("  {:18} {}".format(p, getattr(m, '__version__', '?')))
    except Exception as e:
        print("  {:18} MISSING ({})".format(p, type(e).__name__))
try:
    import torch
    print("  cuda_available:", torch.cuda.is_available())
    print("  torch_cuda_ver:", torch.version.cuda)
    if torch.cuda.is_available():
        print("  device:", torch.cuda.get_device_name(0))
        print("  capability:", torch.cuda.get_device_capability(0))
        print("  arch_list:", torch.cuda.get_arch_list())
except Exception as e:
    print("  torch check failed:", repr(e))
