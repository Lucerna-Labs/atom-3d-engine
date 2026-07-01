import json

with open('F:/OPENCLAW-PROJECTS/lora-memory-experiment/colab/train_lora_fl.ipynb', 'r', encoding='utf-8') as f:
    nb = json.load(f)

num_cells = len(nb['cells'])
print(f'Notebook has {num_cells} cells')
for i, cell in enumerate(nb['cells']):
    ctype = cell['cell_type']
    source = ''.join(cell['source'])[:80]
    print(f'  Cell {i}: {ctype} - {source}...')