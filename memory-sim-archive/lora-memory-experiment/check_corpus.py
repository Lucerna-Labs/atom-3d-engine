import json

with open("F:/OPENCLAW-PROJECTS/lora-memory-experiment/corpus/final_fl/final_corpus_fl.json", "r", encoding="utf-8") as f:
    d = json.load(f)

print(f"Memories: {len(d)}")
print(f"Sample ID: {d[0]['id']}")
print(f"Sample domain: {d[0]['domain']}")
print(f"Sample title: {d[0]['title']}")
print(f"Sample behaviors: {d[0]['behaviors_encoded']}")
print(f"Sample body (first 200 chars): {d[0]['body'][:200]}")