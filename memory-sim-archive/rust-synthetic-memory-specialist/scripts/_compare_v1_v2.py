import json, glob, os

def grab(tag):
    out = {}
    for d in glob.glob(f'rag_runs/general-corpus-campaign-*{tag}*'):
        rj = os.path.join(d, 'report.json')
        if not os.path.exists(rj):
            continue
        r = json.load(open(rj))
        m = r['model'].replace('qwen35-', '').replace('-raw', '')
        for s in r['summary']:
            out[(m, s['suite'], s['condition'])] = s['mean_passed']
    return out

v1 = grab('stageB-dk')
v2 = grab('stageB2-terse')
models = ['2b-base-q4km', '2b-base-q8', '4b-base-q6', '4b-base-q8']

def show(suite, title):
    print(title)
    hdr = ['model', 'no_mem', 'v1.corpus', 'v2.corpus', 'v1.combo', 'v2.combo']
    w = [16, 8, 11, 11, 10, 10]
    print(''.join(h.ljust(x) for h, x in zip(hdr, w)))
    for m in models:
        row = [
            m,
            v1.get((m, suite, 'no_memory'), '-'),
            v1.get((m, suite, 'general_memory'), '-'),
            v2.get((m, suite, 'general_memory'), '-'),
            v1.get((m, suite, 'combo'), '-'),
            v2.get((m, suite, 'combo'), '-'),
        ]
        print(''.join(str(c).ljust(x) for c, x in zip(row, w)))
    print()

show('holdout', 'HELD-OUT (transfer) /20 -- bare vs verbose-corpus(v1) vs terse-corpus(v2)')
show('dev', 'DEV (in-sample) /20')
