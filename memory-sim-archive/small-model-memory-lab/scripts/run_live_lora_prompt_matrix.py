from __future__ import annotations

import csv
import inspect
import json
import shutil
import tempfile
import time
from pathlib import Path

import torch
from peft import LoraConfig, PeftModel
from transformers import AutoModelForCausalLM, AutoTokenizer

from run_live_memory_prompt_benchmark import MEMORY_PACKS, TASKS, build_prompt, score_output


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "live-lora-prompt-matrix"
MODEL_ID = "Qwen/Qwen3-1.7B-Base"
ADAPTER_DIR = ROOT / "loras" / "structural-primitive-lora-v0-2"


def sanitize_adapter_config(adapter_dir: Path) -> Path:
    """Make a temporary adapter dir compatible with the installed PEFT version."""
    temp_dir = Path(tempfile.mkdtemp(prefix="structural_lora_peft_"))
    for source in adapter_dir.iterdir():
        if source.is_file() and source.name != "adapter_config.json":
            shutil.copy2(source, temp_dir / source.name)

    config = json.loads((adapter_dir / "adapter_config.json").read_text(encoding="utf-8"))
    allowed = set(inspect.signature(LoraConfig.__init__).parameters)
    allowed.discard("self")
    clean = {key: value for key, value in config.items() if key in allowed}
    (temp_dir / "adapter_config.json").write_text(json.dumps(clean, indent=2), encoding="utf-8")

    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    (REPORT_DIR / "sanitized_adapter_config.json").write_text(json.dumps(clean, indent=2), encoding="utf-8")
    (REPORT_DIR / "adapter_config_removed_keys.json").write_text(
        json.dumps(sorted(set(config) - set(clean)), indent=2),
        encoding="utf-8",
    )
    return temp_dir


def load_base_model() -> tuple[object, object]:
    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True, local_files_only=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    dtype = torch.float16 if torch.cuda.is_available() else torch.float32
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        trust_remote_code=True,
        local_files_only=True,
        dtype=dtype,
        device_map="auto",
    )
    model.eval()
    return model, tokenizer


def load_lora_model() -> tuple[object, object]:
    base_model, tokenizer = load_base_model()
    adapter_temp_dir = sanitize_adapter_config(ADAPTER_DIR)
    model = PeftModel.from_pretrained(base_model, str(adapter_temp_dir), local_files_only=True)
    model.eval()
    return model, tokenizer


def generate(model: object, tokenizer: object, prompt: str) -> str:
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    with torch.no_grad():
        generated = model.generate(
            **inputs,
            max_new_tokens=220,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
        )
    return tokenizer.decode(generated[0][inputs["input_ids"].shape[-1] :], skip_special_tokens=True).strip()


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    fields: list[str] = []
    for row in rows:
        for key in row:
            if key not in fields:
                fields.append(key)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    groups: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in rows:
        groups.setdefault((str(row["model_setup"]), str(row["condition"])), []).append(row)

    summary = []
    for (model_setup, condition), group in sorted(groups.items()):
        count = len(group)
        summary.append(
            {
                "model_setup": model_setup,
                "condition": condition,
                "tasks": count,
                "pass_rate": round(sum(1 for row in group if row["passed"]) / count, 4),
                "clean_pass_rate": round(sum(1 for row in group if row["clean_passed"]) / count, 4),
                "mean_score": round(sum(float(row["score"]) for row in group) / count, 4),
                "echo_rate": round(sum(1 for row in group if row["echo"]) / count, 4),
                "second_task_rate": round(sum(1 for row in group if row["second_task"]) / count, 4),
                "avg_words": round(sum(int(row["word_count"]) for row in group) / count, 2),
            }
        )
    return summary


def run_setup(model_setup: str, model: object, tokenizer: object) -> tuple[list[dict[str, object]], list[dict[str, object]]]:
    rows: list[dict[str, object]] = []
    outputs: list[dict[str, object]] = []
    for condition, memory_context in MEMORY_PACKS.items():
        for task in TASKS:
            prompt = build_prompt(task, memory_context)
            start = time.time()
            output = generate(model, tokenizer, prompt)
            elapsed = time.time() - start
            scored = score_output(task, condition, output)
            row = {
                "model_id": MODEL_ID,
                "adapter": str(ADAPTER_DIR) if model_setup == "base_plus_structural_lora" else "",
                "model_setup": model_setup,
                "condition": condition,
                "task_id": task.task_id,
                "family": task.family,
                "elapsed_sec": round(elapsed, 3),
                **scored,
            }
            rows.append(row)
            outputs.append(
                {
                    **row,
                    "prompt": prompt,
                    "output": output,
                }
            )
            print(
                f"{model_setup:<26} | {condition:<18} | {task.task_id:<24} "
                f"score={row['score']:.2f} pass={row['passed']} clean={row['clean_passed']}"
            )
    return rows, outputs


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    rows: list[dict[str, object]] = []
    outputs: list[dict[str, object]] = []

    print(f"Loading base model: {MODEL_ID}")
    base_model, tokenizer = load_base_model()
    setup_rows, setup_outputs = run_setup("base_no_lora", base_model, tokenizer)
    rows.extend(setup_rows)
    outputs.extend(setup_outputs)

    del base_model
    if torch.cuda.is_available():
        torch.cuda.empty_cache()

    print(f"Loading LoRA adapter: {ADAPTER_DIR}")
    lora_model, tokenizer = load_lora_model()
    setup_rows, setup_outputs = run_setup("base_plus_structural_lora", lora_model, tokenizer)
    rows.extend(setup_rows)
    outputs.extend(setup_outputs)

    summary = summarize(rows)
    write_csv(REPORT_DIR / "trial_rows.csv", rows)
    write_csv(REPORT_DIR / "summary.csv", summary)
    (REPORT_DIR / "outputs.json").write_text(json.dumps(outputs, indent=2), encoding="utf-8")
    (REPORT_DIR / "report.json").write_text(
        json.dumps(
            {
                "description": "Live Transformers benchmark for Qwen/Qwen3-1.7B-Base with and without the structural primitive LoRA.",
                "model_id": MODEL_ID,
                "adapter_dir": str(ADAPTER_DIR),
                "conditions": list(MEMORY_PACKS),
                "tasks": [task.__dict__ for task in TASKS],
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    print()
    print("Summary")
    for row in summary:
        print(
            f"{row['model_setup']:<26} | {row['condition']:<18} "
            f"pass={row['pass_rate']:.3f} clean={row['clean_pass_rate']:.3f} "
            f"score={row['mean_score']:.3f} echo={row['echo_rate']:.3f} second={row['second_task_rate']:.3f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
