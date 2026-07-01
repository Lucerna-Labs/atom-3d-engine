from huggingface_hub import HfApi
api = HfApi()
print("=== Qwen 3.5 base/small models on HF ===")
try:
    for m in api.list_models(author="Qwen", search="Qwen3.5", limit=60):
        mid = m.id
        if any(k in mid for k in ["2B", "4B", "1.7B", "0.6B", "Base", "base"]):
            print(" ", mid)
except Exception as e:
    print("lookup failed:", repr(e))
