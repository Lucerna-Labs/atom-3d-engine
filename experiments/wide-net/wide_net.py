"""Wide-net cross-domain discovery (v2) — best bridge PER FIELD-PAIR, strong-only stream.

v1 was a firehose: it deduped by full domain-path (~21M possible), so it streamed 1.1M low-meaning
bridges and grew an unbounded list. v2 dedups by the unordered endpoint FIELD-PAIR (≤~2,300) and keeps
only the best chain connecting each pair — bounded memory, and a streamed feed of only the STRONG
bridges (score ≥ stream threshold). Same gating: SYMBOL (shared root atoms) × GLUE (explicit
cross-domain wiring link) × CURRENCY (lands on a real-wall primitive) × domain-spread.

Device: Intel Arc XPU preferred, CPU fallback. NEVER CUDA — the RTX stays free.
"""
import json, time, sys, re
import numpy as np
import torch

CATALOG = r"C:\Projects\wide-net\catalog.json"
OUT = r"C:\Users\jgali\AppData\Local\Temp\claude\C--Projects-3D-Primitve-math-engine\d8c40a60-0d8a-4164-ba60-e024faf340a5\scratchpad\discoveries.txt"
ATOMS = ['scan', 'hash', 'fold', 'project', 'scale', 'compare', 'combine', 'order']
STOP = set('and the of a an in to for with theory based system systems processing general advanced'.split())

def pick_device():
    try:
        if hasattr(torch, 'xpu') and torch.xpu.is_available():
            return torch.device('xpu'), torch.xpu.get_device_name(0)
    except Exception:
        pass
    return torch.device('cpu'), 'cpu'  # never cuda — RTX stays free

def parse_args():
    a = {'minutes': 90.0, 'batch': 200000, 'thresh': 1.6, 'stream': 3.0}
    v = sys.argv
    for i, x in enumerate(v):
        if x == '--minutes' and i + 1 < len(v): a['minutes'] = float(v[i + 1])
        if x == '--batch' and i + 1 < len(v): a['batch'] = int(v[i + 1])
        if x == '--thresh' and i + 1 < len(v): a['thresh'] = float(v[i + 1])
        if x == '--stream' and i + 1 < len(v): a['stream'] = float(v[i + 1])
    return a

def main():
    args = parse_args()
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
        sys.stderr.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass
    dev, name = pick_device()
    cat = json.load(open(CATALOG, encoding='utf-8'))
    for c in cat:
        c['name'] = (c.get('name') or '').split(' (cross-domain')[0].strip()
    N = len(cat)
    domains = sorted(set(c['domain'] for c in cat))
    dom_id = {d: i for i, d in enumerate(domains)}
    D = len(domains)

    atom_mat = np.zeros((N, 8), dtype=np.float32)
    dom_index_np = np.zeros(N, dtype=np.int64)
    real_wall_np = np.zeros(N, dtype=np.float32)
    for i, c in enumerate(cat):
        t = (c.get('atom') or '').lower()
        for k, atom in enumerate(ATOMS):
            if atom in t:
                atom_mat[i, k] = 1.0
        dom_index_np[i] = dom_id[c['domain']]
        real_wall_np[i] = 1.0 if c.get('real_wall') == 'yes' else 0.0

    dom_kw = {d: [w for w in re.split(r'[-_ ]', d.lower()) if len(w) > 2 and w not in STOP] for d in domains}
    wiring_dom = np.zeros((N, D), dtype=np.float32)
    for i, c in enumerate(cat):
        w = (c.get('wiring') or '').lower()
        if not w:
            continue
        for d in domains:
            for kw in dom_kw[d]:
                if kw in w:
                    wiring_dom[i, dom_id[d]] = 1.0
                    break

    atom_mat_t = torch.tensor(atom_mat, device=dev)
    dom_index_t = torch.tensor(dom_index_np, device=dev)
    real_wall_t = torch.tensor(real_wall_np, device=dev)
    wiring_dom_t = torch.tensor(wiring_dom, device=dev)

    print(f"=== WIDE NET v2 START | device={name} | {N} primitives / {D} domains | "
          f"stream>={args['stream']} | budget {args['minutes']:.0f} min ===", flush=True)

    best_pair = {}   # frozenset{src_dom, dst_dom} -> (score, domain_path_str, names)
    M = args['batch']
    THRESH = args['thresh']
    STREAM = args['stream']
    start = time.time()
    budget = args['minutes'] * 60.0
    chains_total = 0
    last_hb = 0.0
    streamed = 0
    lens = [2, 3, 4]
    li = 0

    while time.time() - start < budget:
        L = lens[li % 3]; li += 1
        idx = torch.randint(0, N, (M, L), device=dev)
        doms = dom_index_t[idx]
        valid = torch.ones(M, dtype=torch.bool, device=dev)
        has_wire = torch.zeros(M, dtype=torch.bool, device=dev)
        score = torch.zeros(M, device=dev)
        for p in range(L - 1):
            a, b = idx[:, p], idx[:, p + 1]
            ov = (atom_mat_t[a] * atom_mat_t[b]).sum(-1)
            wl = wiring_dom_t[a].gather(1, doms[:, p + 1:p + 2]).squeeze(1)
            valid &= (ov > 0) | (wl > 0.5)
            has_wire |= (wl > 0.5)
            score += 0.6 * wl + 0.2 * (ov > 0).float() + 0.1 * torch.clamp(ov, max=3) / 3.0
        score = score / max(L - 1, 1)
        nov = torch.ones(M, device=dev)
        for p in range(1, L):
            isnew = torch.ones(M, dtype=torch.bool, device=dev)
            for q in range(p):
                isnew &= (doms[:, p] != doms[:, q])
            nov += isnew.float()
        term_rw = real_wall_t[idx[:, -1]]
        final = score * nov * (1.0 + 0.5 * term_rw)
        keep = valid & has_wire & (nov >= 2) & (final > THRESH)
        chains_total += M

        nz = keep.nonzero(as_tuple=False).squeeze(1)
        if nz.numel() > 0:
            take = nz[:2048]
            sc = final[take].detach().to('cpu').numpy()
            chains = idx[take].detach().to('cpu').numpy()
            order = np.argsort(-sc)
            for r in order:
                ch = chains[r]; s = float(sc[r])
                dompath = [int(dom_index_np[j]) for j in ch]
                if dompath[0] == dompath[-1] or len(set(dompath)) < 2:
                    continue
                pair = frozenset((dompath[0], dompath[-1]))
                prev = best_pair.get(pair)
                if prev is not None and s <= prev[0] + 0.1:
                    continue  # only a genuine improvement for this field-pair
                names = " -> ".join(cat[j]['name'] for j in ch)
                dpath = " -> ".join(domains[d] for d in dompath)
                best_pair[pair] = (s, dpath, names)
                if s >= STREAM:
                    streamed += 1
                    el = time.time() - start
                    print(f"[{el:6.0f}s] BRIDGE {s:.2f} | {dpath} | {names}", flush=True)

        el = time.time() - start
        if el - last_hb >= 60.0:
            last_hb = el
            cps = chains_total / el
            best = max((v[0] for v in best_pair.values()), default=0.0)
            print(f"[{el:6.0f}s] .. {chains_total/1e6:.0f}M chains ({cps/1e6:.1f}M/s) | "
                  f"field-pairs={len(best_pair)} | streamed strong={streamed} | best={best:.2f}", flush=True)

    ranked = sorted(best_pair.values(), key=lambda x: -x[0])
    with open(OUT, 'w', encoding='utf-8') as f:
        f.write(f"# wide-net v2 — best bridge per field-pair: {len(ranked)} pairs, "
                f"{chains_total/1e6:.0f}M chains gated on {name}\n\n")
        for s, dpath, names in ranked:
            f.write(f"{s:.2f}\t{dpath}\t{names}\n")
    print(f"\n=== DONE after {time.time()-start:.0f}s | {chains_total/1e6:.0f}M chains | "
          f"{len(ranked)} field-pair bridges -> {OUT} ===", flush=True)
    print("Top 20 field-pair bridges:", flush=True)
    for s, dpath, names in ranked[:20]:
        print(f"  {s:.2f} | {dpath} | {names}", flush=True)

if __name__ == "__main__":
    main()
