import json, glob, os

SOURCES = [
    ('conf-bare', 'no_memory', 'bare'),
    ('conf-answeronly', 'general_memory', 'answer-only'),
    ('conf-sharp', 'general_memory', 'sharp'),
    ('conf-hybrid', 'general_memory', 'hybrid'),
    ('fm-full', 'general_memory', 'false.full(6)'),
    ('fm-compact', 'general_memory', 'false.compact(3)'),
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
    w = [16] + [15] * len(labels)
    print(''.join(h.ljust(x) for h, x in zip(['model'] + labels, w)))
    for m in MODELS:
        row = [m] + [data[l].get((m, suite), '-') for l in labels]
        print(''.join(str(c).ljust(x) for c, x in zip(row, w)))
    print()


show('holdout', 'HELD-OUT (transfer) /20 -- false-memory vs discipline nudges')
show('dev', 'DEV (in-sample) /20')
