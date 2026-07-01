import json
from pathlib import Path

corpus_path = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/corpus/final/final_corpus.json")
with open(corpus_path, 'r', encoding='utf-8') as f:
    d = json.load(f)

print(f"Memories: {len(d)}")
print(f"First: {d[0]['id']} - {d[0]['domain']}")
print(f"Last: {d[-1]['id']} - {d[-1]['domain']}")

domains = set(m['domain'] for m in d)
print(f"Domains: {domains}")

behaviors = set()
for m in d:
    behaviors.update(m['behaviors_encoded'])
print(f"Unique behaviors: {len(behaviors)}")

lb = sum(1 for m in d if m['load_bearing'])
print(f"Load-bearing: {lb}")

echo = sum(1 for m in d if m['echoes_from'] or m['echoes_to'])
print(f"Echo-connected: {echo}")

# Check validation
val_path = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/corpus/final/final_validation.json")
with open(val_path, 'r', encoding='utf-8') as f:
    val = json.load(f)
print(f"\nValidation: {val['validated']}")
print(f"Combined score: {val['combined_score']:.3f}")