"""
LoRA Memory Experiment: Forensic + Logic Training Script
Run on Google Colab T4 GPU

This script:
1. Installs dependencies
2. Authenticates with Hugging Face
3. Loads Llama 3.2 1B
4. Configures LoRA
5. Trains on episodic memory corpus
6. Saves the adapter
7. Runs evaluation
"""

import json
import os
import sys

# ============================================================
# STEP 1: Install dependencies
# ============================================================
print("=" * 70)
print("STEP 1: Installing dependencies...")
print("=" * 70)

os.system("pip install -q transformers datasets accelerate peft bitsandbytes trl")

import torch
print(f"PyTorch: {torch.__version__}")
print(f"CUDA available: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    print(f"GPU: {torch.cuda.get_device_name(0)}")
    vram = torch.cuda.get_device_properties(0).total_memory / 1e9
    print(f"VRAM: {vram:.1f} GB")

# ============================================================
# STEP 2: Configuration
# ============================================================
print("\n" + "=" * 70)
print("STEP 2: Configuration")
print("=" * 70)

MODEL_ID = "meta-llama/Llama-3.2-1B"
OUTPUT_DIR = "./lora-memory-forensic-logic"

LORA_R = 16
LORA_ALPHA = 32
LORA_DROPOUT = 0.05
TARGET_MODULES = ["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"]

BATCH_SIZE = 4
GRADIENT_ACCUMULATION = 4
LEARNING_RATE = 2e-4
NUM_EPOCHS = 3
WARMUP_RATIO = 0.1
MAX_SEQ_LENGTH = 1024
LOGGING_STEPS = 5
SAVE_STEPS = 25

USE_4BIT = True
BNB_4BIT_DTYPE = "nf4"

print(f"Model: {MODEL_ID}")
print(f"LoRA rank: {LORA_R}, alpha: {LORA_ALPHA}")
print(f"Epochs: {NUM_EPOCHS}, LR: {LEARNING_RATE}")
print(f"Batch size: {BATCH_SIZE}, Grad accum: {GRADIENT_ACCUMULATION}")
print(f"4-bit quantization: {USE_4BIT}")

# ============================================================
# STEP 3: Authentication
# ============================================================
print("\n" + "=" * 70)
print("STEP 3: Hugging Face Authentication")
print("=" * 70)

from huggingface_hub import login

# Try Colab secrets first, then environment variable, then interactive
token = None
try:
    from google.colab import userdata
    token = userdata.get('HF_TOKEN')
    print("Authenticated via Colab secrets")
except:
    pass

if not token:
    hf_token_env = os.environ.get('HF_TOKEN') or os.environ.get('HUGGING_FACE_HUB_TOKEN')
    if hf_token_env:
        token = hf_token_env
        print("Authenticated via environment variable")
    else:
        token = input("Enter your Hugging Face token: ")

login(token=token)
print("Logged in to Hugging Face")

# ============================================================
# STEP 4: Load model and tokenizer
# ============================================================
print("\n" + "=" * 70)
print("STEP 4: Loading model and tokenizer")
print("=" * 70)

from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig
from peft import LoraConfig, get_peft_model, prepare_model_for_kbit_training

BNB_4BIT_COMPUTE_DTYPE = torch.bfloat16

if USE_4BIT:
    bnb_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type=BNB_4BIT_DTYPE,
        bnb_4bit_compute_dtype=BNB_4BIT_COMPUTE_DTYPE,
        bnb_4bit_use_double_quant=True,
    )
else:
    bnb_config = None

print("Loading tokenizer...")
tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)
if tokenizer.pad_token is None:
    tokenizer.pad_token = tokenizer.eos_token
    tokenizer.pad_token_id = tokenizer.eos_token_id

print("Loading model...")
model = AutoModelForCausalLM.from_pretrained(
    MODEL_ID,
    quantization_config=bnb_config,
    device_map="auto",
    torch_dtype=torch.bfloat16 if USE_4BIT else torch.float32,
)

if USE_4BIT:
    model = prepare_model_for_kbit_training(model)

print(f"Model loaded: {MODEL_ID}")
print(f"Parameters: {model.num_parameters():,}")

# ============================================================
# STEP 5: Configure LoRA
# ============================================================
print("\n" + "=" * 70)
print("STEP 5: Configuring LoRA")
print("=" * 70)

lora_config = LoraConfig(
    r=LORA_R,
    lora_alpha=LORA_ALPHA,
    lora_dropout=LORA_DROPOUT,
    target_modules=TARGET_MODULES,
    bias="none",
    task_type="CAUSAL_LM",
)

model = get_peft_model(model, lora_config)
model.print_trainable_parameters()

# ============================================================
# STEP 6: Load and format training data
# ============================================================
print("\n" + "=" * 70)
print("STEP 6: Loading training data")
print("=" * 70)

# The corpus is embedded directly in this script as a fallback
# but we also try to load from file
CORPUS_PATH = "./training_corpus_fl.json"

# Check if file exists (it will if uploaded to Colab)
if os.path.exists(CORPUS_PATH):
    with open(CORPUS_PATH, 'r', encoding='utf-8') as f:
        corpus = json.load(f)
    print(f"Loaded corpus from file: {len(corpus)} memories")
else:
    print("ERROR: No corpus file found. Please upload training_corpus_fl.json to Colab.")
    print("The file should be at: " + os.path.abspath(CORPUS_PATH))
    sys.exit(1)

# System prompt
SYSTEM_PROMPT = """You are an experienced analytical thinker who has encountered and navigated complex situations involving deception, logical reasoning, and behavioral analysis throughout your life. You respond based on lived experience, not textbook rules. When you detect deception, analyze statements, assess threats, or validate reasoning, you do so because you have been in those situations before and recognized the patterns."""

def format_memory_as_training(memory):
    domain = memory["domain"]
    title = memory["title"]
    body = memory["body"]
    sensory = memory.get("sensory_anchor", "")
    emotion = memory.get("emotional_signature", "")
    
    if domain == "forensic":
        prompt = f"In reading people and situations, I recall: {title.lower()}. What did I notice?"
    else:
        prompt = f"In reasoning through logical problems, I recall: {title.lower()}. What did I notice?"
    
    response_parts = [body]
    if sensory:
        response_parts.append(f"What I remember most: {sensory}.")
    if emotion:
        response_parts.append(f"How it felt: {emotion}.")
    
    response = " ".join(response_parts)
    
    messages = [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": prompt},
        {"role": "assistant", "content": response},
    ]
    
    return {"messages": messages}

training_data = [format_memory_as_training(m) for m in corpus]

import random
random.seed(42)
random.shuffle(training_data)

split_idx = int(len(training_data) * 0.9)
train_data = training_data[:split_idx]
eval_data = training_data[split_idx:]

print(f"Training examples: {len(train_data)}")
print(f"Eval examples: {len(eval_data)}")
print(f"\nSample training example:")
sample = train_data[0]
for msg in sample["messages"]:
    content_preview = msg['content'][:150]
    print(f"  [{msg['role']}]: {content_preview}...")

# ============================================================
# STEP 7: Tokenize
# ============================================================
print("\n" + "=" * 70)
print("STEP 7: Tokenizing dataset")
print("=" * 70)

from datasets import Dataset

def tokenize_chat(example):
    text = tokenizer.apply_chat_template(
        example["messages"],
        tokenize=False,
        add_generation_prompt=False,
    )
    tokenized = tokenizer(
        text,
        truncation=True,
        max_length=MAX_SEQ_LENGTH,
        padding=False,
    )
    tokenized["labels"] = tokenized["input_ids"].copy()
    return tokenized

train_dataset = Dataset.from_list(train_data)
eval_dataset = Dataset.from_list(eval_data)

train_dataset = train_dataset.map(tokenize_chat, remove_columns=["messages"])
eval_dataset = eval_dataset.map(tokenize_chat, remove_columns=["messages"])

print(f"Tokenized train dataset: {len(train_dataset)} examples")
print(f"Tokenized eval dataset: {len(eval_dataset)} examples")

# ============================================================
# STEP 8: TRAIN
# ============================================================
print("\n" + "=" * 70)
print("STEP 8: Training LoRA adapter")
print("=" * 70)

from transformers import TrainingArguments
from trl import SFTTrainer

training_args = TrainingArguments(
    output_dir=OUTPUT_DIR,
    num_train_epochs=NUM_EPOCHS,
    per_device_train_batch_size=BATCH_SIZE,
    per_device_eval_batch_size=BATCH_SIZE,
    gradient_accumulation_steps=GRADIENT_ACCUMULATION,
    learning_rate=LEARNING_RATE,
    lr_scheduler_type="cosine",
    warmup_ratio=WARMUP_RATIO,
    logging_steps=LOGGING_STEPS,
    save_steps=SAVE_STEPS,
    eval_strategy="steps",
    eval_steps=SAVE_STEPS,
    bf16=True,
    gradient_checkpointing=True,
    gradient_checkpointing_kwargs={"use_reentrant": False},
    report_to="none",
    remove_unused_columns=False,
    dataloader_pin_memory=False,
)

trainer = SFTTrainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset,
    eval_dataset=eval_dataset,
    processing_class=tokenizer,
    max_seq_length=MAX_SEQ_LENGTH,
)

trainable = sum(p.numel() for p in model.parameters() if p.requires_grad)
total_steps = (len(train_dataset) // (BATCH_SIZE * GRADIENT_ACCUMULATION)) * NUM_EPOCHS
print(f"Trainable params: {trainable:,}")
print(f"Estimated total steps: ~{total_steps}")
print(f"\nStarting training...")

train_result = trainer.train()

print(f"\nTraining complete!")
print(f"  Final loss: {train_result.training_loss:.4f}")
print(f"  Total steps: {train_result.global_step}")

# ============================================================
# STEP 9: Save adapter
# ============================================================
print("\n" + "=" * 70)
print("STEP 9: Saving LoRA adapter")
print("=" * 70)

adapter_path = os.path.join(OUTPUT_DIR, "final_adapter")
model.save_pretrained(adapter_path)
tokenizer.save_pretrained(adapter_path)

print(f"LoRA adapter saved to: {adapter_path}")

# ============================================================
# STEP 10: Quick evaluation
# ============================================================
print("\n" + "=" * 70)
print("STEP 10: Quick evaluation")
print("=" * 70)

EVAL_PROMPTS = [
    "Someone gives a very detailed, chronological account of an incident. Everything checks out factually. But something feels wrong about it. What should I look for?",
    "In a statement about an alleged robbery, the person says 'I was walking home and then this guy came out of nowhere and took my wallet.' What's missing from this statement that would be in a genuine account?",
    "If it rains, the streets are wet. The streets are wet. Does that mean it rained?",
    "A suspect's alibi is logically consistent and emotionally compelling. Every detail checks out. But the level of detail seems slightly too perfect, and there are no spontaneous corrections. What's happening here?",
    "The argument is: 'This new drug should be approved because it passed clinical trials.' What assumption is being made?",
]

# Load base model for comparison
print("Loading base model for comparison...")
base_model = AutoModelForCausalLM.from_pretrained(
    MODEL_ID,
    quantization_config=bnb_config if USE_4BIT else None,
    device_map="auto",
    torch_dtype=torch.bfloat16 if USE_4BIT else torch.float32,
)
base_tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)

def generate_response(model_obj, tok, prompt, max_new_tokens=256):
    messages = [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": prompt},
    ]
    text = tok.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tok(text, return_tensors="pt").to(model_obj.device)
    
    with torch.no_grad():
        outputs = model_obj.generate(
            **inputs,
            max_new_tokens=max_new_tokens,
            temperature=0.7,
            top_p=0.9,
            do_sample=True,
        )
    
    response = tok.decode(outputs[0][inputs["input_ids"].shape[1]:], skip_special_tokens=True)
    return response

print("\n" + "=" * 70)
print("EVALUATION: Base vs Fine-tuned")
print("=" * 70)

results = []
for i, prompt in enumerate(EVAL_PROMPTS):
    print(f"\n--- Prompt {i+1} ---")
    print(f"Q: {prompt[:100]}...")
    
    base_response = generate_response(base_model, base_tokenizer, prompt)
    ft_response = generate_response(model, tokenizer, prompt)
    
    print(f"\nBASE: {base_response[:200]}...")
    print(f"\nFINE-TUNED: {ft_response[:200]}...")

# Save results
eval_results_path = os.path.join(OUTPUT_DIR, "evaluation_results.json")
with open(eval_results_path, 'w') as f:
    json.dump(results, f, indent=2)

print(f"\n\nEvaluation results saved to: {eval_results_path}")
print("\n" + "=" * 70)
print("DONE! LoRA adapter trained and evaluated.")
print("=" * 70)
print(f"\nAdapter saved at: {adapter_path}")
print("To download, zip the directory and use files.download()")