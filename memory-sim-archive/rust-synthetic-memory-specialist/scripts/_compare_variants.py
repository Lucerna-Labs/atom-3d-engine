import json, glob, os

# map a run-dir tag substring + condition -> a column label
SOURCES = [
    ('stageB-dk', 'no_memory', 'bare'),
    ('stageB-dk', 'general_memory', 'v1.verbose'),
    ('stageB2-terse', 'general_memory', 'v2.terse'),
    ('v3-methodsonly', 'general_memory', 'v3.methods'),
    ('v3-commononly', 'general_memory', 'v3.common'),
]
MODELS = ['2b-base-q4km', '2b-base-q8', '4b-base-q6', '4b-base-q8']


def grab(tag, cond):
    out = {}
    for d in glob.glob(f'rag_runs/general-corpus-campaign-*{tag}*'):
        rj = os.path.join(d, 'report.json')
        if not os.path.exists(rj):
            continue
        r = json.load(open(rj))
        m = r['model'].replace('qwen35-', '').replace('-raw', '')
        for s in r['summary']:
            if s['condition'] == cond:
                out[(m, s['suite'])] = s['mean_passed']
    return out


data = {label: grab(tag, cond) for tag, cond, label in SOURCES}
labels = [s[2] for s in SOURCES]


def show(suite, title):
    print(title)
    w = [16] + [12] * len(labels)
    print(''.join(h.ljust(x) for h, x in zip(['model'] + labels, w)))
    for m in MODELS:
        row = [m] + [data[l].get((m, suite), '-') for l in labels]
        print(''.join(str(c).ljust(x) for c, x in zip(row, w)))
    print()


show('holdout', 'HELD-OUT (transfer) /20 -- corpus leanness curve (general_memory, routed)')
show('dev', 'DEV (in-sample) /20')
