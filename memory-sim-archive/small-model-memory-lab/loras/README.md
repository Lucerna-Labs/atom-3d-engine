# LoRA Adapters

This folder contains the LoRA adapters from the public small-model memory experiments.

Both adapters target:

```text
Qwen/Qwen3-1.7B-Base
```

They are PEFT adapters, not full model weights. Load the base model first, then attach the adapter with `PeftModel.from_pretrained`.

## Available Adapters

| Folder | Purpose | Public result |
|---|---|---:|
| `rust-cyber-memory-lora` | cyber/source-boundary synthetic-memory LoRA for prompt-injection behavior | `7/12` strict, `12/12` behavioral on the cyber eval |
| `structural-primitive-lora-v0-2` | cleaned structural primitive LoRA for small-model task stability | `6/6` on the structural smoke eval |

## Example Usage

```python
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PeftModel

base_id = "Qwen/Qwen3-1.7B-Base"
adapter_dir = "/path/to/loras/structural-primitive-lora-v0-2"

tokenizer = AutoTokenizer.from_pretrained(adapter_dir, trust_remote_code=True)
base = AutoModelForCausalLM.from_pretrained(
    base_id,
    device_map="auto",
    torch_dtype="auto",
    trust_remote_code=True,
)
model = PeftModel.from_pretrained(base, adapter_dir)
model.eval()

prompt = "User: Four printers make 480 pages in 6 minutes. How long for three printers to make 720 pages?\nAssistant:"
inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
output = model.generate(**inputs, max_new_tokens=160, do_sample=False)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## Caveats

- These are research adapters from narrow evals, not production models.
- The structural adapter still shows some repetition even after the v0.2 cleanup.
- The cyber adapter improves behavioral prompt-injection handling on the tested probes, but it is not a complete security system.
- The adapters were trained and evaluated in Kaggle/PyTorch, so outputs may differ from Ollama, LM Studio, llama.cpp, or other runtimes.
