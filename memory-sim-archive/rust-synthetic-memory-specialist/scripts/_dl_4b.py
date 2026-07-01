from huggingface_hub import snapshot_download
p = snapshot_download("Qwen/Qwen3.5-4B-Base",
    allow_patterns=["*.safetensors","*.json","*.txt","*.model","tokenizer*","merges.txt","vocab.json"])
print("DOWNLOADED_TO", p)
