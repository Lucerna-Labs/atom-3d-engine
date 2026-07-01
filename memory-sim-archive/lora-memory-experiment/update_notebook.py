import json

path = 'F:/OPENCLAW-PROJECTS/lora-memory-experiment/colab/train_lora_fl.ipynb'
with open(path, 'r', encoding='utf-8') as f:
    nb = json.load(f)

for cell in nb['cells']:
    if cell['cell_type'] == 'code':
        new_source = []
        for line in cell['source']:
            if 'final_corpus_fl.json' in line:
                line = line.replace('final_corpus_fl.json', 'training_corpus_fl.json')
                print('Updated line')
            new_source.append(line)
        cell['source'] = new_source

with open(path, 'w', encoding='utf-8') as f:
    json.dump(nb, f, indent=2, ensure_ascii=False)

print('Notebook updated')