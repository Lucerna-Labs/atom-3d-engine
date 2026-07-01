import json, glob, os

SOURCES = [
    ('stageB-dk', 'no_memory', 'bare'),
    ('v3-commononly', 'general_memory', 'disc(2line)'),
    ('d-answeronly', 'general_memory', 'answer-only'),
    ('d-separateonly', 'general_memory', 'separate-only'),
    ('d-sharp', 'general_memory', 'sharp-1line'),
    ('d-verify', 'general_memory', 'answer+verify'),
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
    w = [16] + [14] * len(labels)
    print(''.join(h.ljust(x) for h, x in zip(['model'] + labels, w)))
    for m in MODELS:
        row = [m] + [data[l].get((m, suite), '-') for l in labels]
        print(''.join(str(c).ljust(x) for c, x in zip(row, w)))
    print()


show('holdout', 'HELD-OUT (transfer) /20 -- discipline-nudge variants')
show('dev', 'DEV (in-sample) /20')
