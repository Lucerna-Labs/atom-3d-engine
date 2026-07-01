# engine-mix-8h on Kaggle GPU: larger version of the Arc engine-mix search. Tests new-domain
# mechanisms (predictive-coding momentum, quantum-walk stochastic omega, ANS subitize) as real
# marcher operators against the engine's measured field-eval currency, at HIGHER resolution than
# the Arc run (128x72 vs 96x54 — checks whether the low-res -35% signal survives more fidelity),
# a bigger population, ~7.5h, plus an end-of-run per-operator ABLATION. Streams to a webhook every
# 5 min. On Kaggle the GPU IS the compute target (cuda) — the "RTX stays free" rule is about the
# local machine, not Kaggle.
import os, sys, subprocess, time, urllib.request

INBOX = "https://webhook.site/ff06312d-a70d-42ec-870f-fa6acfc717c4"
BUDGET_MIN = 450  # 7.5h — headroom under Kaggle's session ceiling for the final post + ablation

def post(text):
    try:
        req = urllib.request.Request(INBOX, data=text.encode("utf-8", "replace"),
                                     headers={"Content-Type": "text/plain"}, method="POST")
        urllib.request.urlopen(req, timeout=30).read()
    except Exception as e:
        print("[relay] post failed:", e, flush=True)

# Kaggle's API-provisioned GPU is often a Tesla P100 (sm_60) whose preinstalled torch lacks Pascal
# support. Detect it and reinstall a cu121 wheel that includes sm_60 BEFORE importing torch.
def gpu_name():
    try:
        return subprocess.run(["nvidia-smi", "--query-gpu=name", "--format=csv,noheader"],
                              capture_output=True, text=True, timeout=30).stdout.strip()
    except Exception:
        return ""

_g = gpu_name()
print("provisioned GPU:", _g or "(none)", flush=True)
post("engine-mix-8h: kernel started on GPU=" + (_g or "cpu"))
if "P100" in _g:
    print("P100 detected -> reinstalling cu121 torch...", flush=True)
    subprocess.run([sys.executable, "-m", "pip", "install", "-q", "--force-reinstall", "--no-deps",
                    "torch", "--index-url", "https://download.pytorch.org/whl/cu121"], check=False)

import torch
import numpy as np

def pick_device():
    if torch.cuda.is_available():
        try:
            _ = (torch.zeros(8, device="cuda") + 1).sum().item()
            return torch.device("cuda"), torch.cuda.get_device_name(0)
        except Exception as e:
            print("cuda unusable, CPU fallback:", e, flush=True)
    return torch.device("cpu"), "cpu"

def sdf(p):
    x, y, z = p[..., 0], p[..., 1], p[..., 2]
    d = y
    d = torch.minimum(d, torch.sqrt((x + 1.1) ** 2 + (y - 1.0) ** 2 + z ** 2) - 1.0)
    qx = torch.abs(x - 1.2) - 0.6
    qy = torch.abs(y - 0.9) - 0.6
    qz = torch.abs(z - 0.3) - 0.6
    outside = torch.sqrt(torch.clamp(qx, min=0) ** 2 + torch.clamp(qy, min=0) ** 2 + torch.clamp(qz, min=0) ** 2)
    inside = torch.clamp(torch.maximum(qx, torch.maximum(qy, qz)), max=0.0)
    d = torch.minimum(d, outside + inside - 0.12)
    cx, cy, cz = x - 0.1, y - 0.6, z - 1.8
    t0 = torch.sqrt(cx ** 2 + cz ** 2) - 0.7
    d = torch.minimum(d, torch.sqrt(t0 ** 2 + cy ** 2) - 0.25)
    return d

def make_rays(ncam, w, h):
    def n(v):
        return v / max(np.linalg.norm(v), 1e-9)
    eyes, dirs = [], []
    center = np.array([0.1, 0.7, 0.4])
    for i in range(ncam):
        theta = i * 2.3999632
        dist = 4.6 + (i % 7) * 0.35
        height = 1.0 + (i % 5) * 0.45
        eye = np.array([center[0] + np.sin(theta) * dist, height, center[2] + np.cos(theta) * dist])
        fwd = n(center - eye)
        right = n(np.cross(fwd, np.array([0.0, 1.0, 0.0])))
        up = np.cross(right, fwd)
        aspect = w / h
        xs = (np.arange(w) + 0.5) / w * 2 - 1
        ys = 1 - (np.arange(h) + 0.5) / h * 2
        uu, vv = np.meshgrid(xs, ys)
        dd = right * (uu * aspect * 0.6)[..., None] + up * (vv * 0.6)[..., None] + fwd
        dd = dd / np.maximum(np.linalg.norm(dd, axis=-1, keepdims=True), 1e-9)
        dirs.append(dd.reshape(-1, 3))
        eyes.append(np.broadcast_to(eye, dd.reshape(-1, 3).shape))
    return np.concatenate(eyes, 0), np.concatenate(dirs, 0)

SHADOW = 24

def march(eye, dir, om, lod, sec, fq, mom, stoch, subk, nmode, max_steps=180):
    P, N, dev = om.shape[0], dir.shape[0], eye.device
    z = lambda *s: torch.zeros(*s, device=dev)
    t = z(P, N)
    active = torch.ones((P, N), dtype=torch.bool, device=dev)
    prev_radius = z(P, N)
    step_len = z(P, N)
    omega = om[:, None].expand(P, N).clone()
    lodc, sthr, fqc = lod[:, None], sec[:, None], fq[:, None]
    momc, stochc, subkc = mom[:, None], stoch[:, None], subk[:, None]
    crawl = z(P, N)
    d_prev = torch.full((P, N), float('inf'), device=dev)
    t_prev = z(P, N)
    hit = torch.zeros((P, N), dtype=torch.bool, device=dev)
    t_hit = z(P, N)
    evals = torch.zeros(P, dtype=torch.int64, device=dev)
    eyeb, dirb = eye[None], dir[None]
    for i in range(max_steps):
        if not bool(active.any()):
            break
        pos = eyeb + t[..., None] * dirb
        d = sdf(pos)
        evals += active.sum(dim=1)
        radius = torch.abs(d)
        eps = 0.0006 * (1.0 + 0.5 * t) + lodc * t
        overshoot = (omega > 1.0) & (radius + prev_radius < step_len) & active
        newhit = active & (~overshoot) & (d < eps)
        jitter = torch.frac(torch.sin(t * 12.9898 + float(i) * 78.233) * 43758.5453).abs()
        omega_eff = (omega + stochc * (jitter - 0.5) * 2.0).clamp(1.0, 2.0)
        step_b = d * omega_eff
        approach = torch.clamp(torch.where(torch.isfinite(d_prev), d_prev - d, torch.zeros_like(d)), min=0.0)
        step_b = step_b + momc * approach
        far = d > (eps * 6.0)
        step_b = torch.where(far, step_b * (1.0 + subkc), step_b)
        use_sec = (sthr > 0) & (~overshoot) & (d < sthr) & torch.isfinite(d_prev)
        dt = t - t_prev
        dd = d - d_prev
        sec_ok = use_sec & (dt > 1e-6) & (dd < -1e-6)
        sec_step = torch.minimum(torch.maximum(torch.clamp(-d * dt / torch.where(dd == 0, torch.full_like(dd, -1e-9), dd), min=0), d), d * 4.0)
        step_b = torch.where(sec_ok, sec_step, step_b)
        near = (fqc > 0) & (~overshoot) & (~newhit) & (d < eps * 6.0)
        crawl = torch.where(near, crawl + 1, torch.zeros_like(crawl))
        newhit = newhit | (active & near & (crawl >= fqc.float()))
        step_len = torch.where(overshoot, step_len * (1.0 - omega), step_b)
        omega = torch.where(overshoot, torch.ones_like(omega), omega)
        just = newhit & active
        t_hit = torch.where(just, t, t_hit)
        hit = hit | just
        active = active & (~just)
        prev_radius, d_prev, t_prev = radius, d, t
        t = torch.where(active, t + step_len, t)
        active = active & (t <= 30.0)
    nh = hit.sum(dim=1)
    ncost = torch.where(nmode == 2, torch.zeros_like(om), torch.where(nmode == 1, torch.full_like(om, 3.0), torch.full_like(om, 4.0))).to(torch.int64)
    total = evals + nh * ncost + nh * SHADOW
    return hit, t_hit, total

def evaluate(eye, dir, ref_hit, ref_t, base, g, max_steps=180):
    om, lod, sec, fq, mom, stoch, subk, nm = g
    hit, t_hit, ev = march(eye, dir, om, lod, sec, fq, mom, stoch, subk, nm, max_steps)
    sil = (hit != ref_hit[None]).float().mean(dim=1)
    both = hit & ref_hit[None]
    depth = (torch.where(both, (t_hit - ref_t[None]).abs() / (ref_t[None] + 1e-3), torch.zeros_like(t_hit)).sum(dim=1)
             / both.sum(dim=1).clamp(min=1))
    return (ev - base).float() / base * 100.0, sil + 0.3 * depth

def main():
    dev, name = pick_device()
    ncam, w, h = 12, 128, 72
    eye_np, dir_np = make_rays(ncam, w, h)
    eye = torch.tensor(eye_np, dtype=torch.float32, device=dev)
    dir = torch.tensor(dir_np, dtype=torch.float32, device=dev)
    N = dir.shape[0]
    zc = lambda v: torch.tensor(v, dtype=torch.float32, device=dev)
    zi = lambda v: torch.tensor(v, dtype=torch.int64, device=dev)
    rnd = lambda *s: torch.rand(*s, device=dev)
    rndn = lambda *s: torch.randn(*s, device=dev)
    rndi = lambda hi, *s: torch.randint(0, hi, s, device=dev)

    ref_hit, ref_t, _ = march(eye, dir, zc([1.0]), zc([0.0]), zc([0.0]), zi([0]), zc([0.0]), zc([0.0]), zc([0.0]), zi([0]), 360)
    ref_hit, ref_t = ref_hit[0], ref_t[0]
    _, _, base_ev = march(eye, dir, zc([1.0]), zc([0.0]), zc([0.0]), zi([0]), zc([0.0]), zc([0.0]), zc([0.0]), zi([0]))
    base = int(base_ev[0])

    P, ELITE, TOL = 512, 32, 0.012
    om = (1.0 + 0.8 * rnd(P)).clamp(1.0, 1.8)
    lod = 0.012 * rnd(P)
    sec = torch.where(rnd(P) < 0.6, 0.02 + 0.13 * rnd(P), torch.zeros(P, device=dev))
    fq = rndi(5, P) * 4
    mom = torch.where(rnd(P) < 0.5, 0.6 * rnd(P), torch.zeros(P, device=dev))
    stoch = torch.where(rnd(P) < 0.5, 0.3 * rnd(P), torch.zeros(P, device=dev))
    subk = torch.where(rnd(P) < 0.5, 3.0 * rnd(P), torch.zeros(P, device=dev))
    nm = rndi(3, P)
    modes = {0: 'tetra', 1: 'fwd3', 2: 'dual'}

    def label(t):
        r, o, l, s, f, mo, st, su, n = t
        return (f"{r:+.1f}% | omega={o:.2f} lod={l:.4f} sec={s:.3f} fq={f} normal={modes[n]} | "
                f"NEW mom={mo:.3f} stoch={st:.3f} subitize={su:.3f}")

    hdr = (f"=== ENGINE-MIX-8H START | device={name} | {N} rays ({ncam} cams {w}x{h}) | pop={P} | "
           f"budget {BUDGET_MIN} min | stock baseline {base} field-evals ===")
    print(hdr, flush=True); post(hdr)

    best = None
    best_new = None
    start = time.time()
    budget = BUDGET_MIN * 60.0
    gen = 0
    last = 0.0
    while time.time() - start < budget:
        redux, err = evaluate(eye, dir, ref_hit, ref_t, base, (om, lod, sec, fq, mom, stoch, subk, nm))
        ok = err <= TOL
        score = torch.where(ok, redux, torch.full_like(redux, 1e6))
        order = torch.argsort(score)
        bi = int(order[0].item())
        if bool(ok[bi]):
            cand = (float(redux[bi]), float(om[bi]), float(lod[bi]), float(sec[bi]), int(fq[bi]),
                    float(mom[bi]), float(stoch[bi]), float(subk[bi]), int(nm[bi]))
            if best is None or cand[0] < best[0]:
                best = cand
                line = f"[{time.time()-start:6.0f}s g{gen}] BEST {label(cand)}"
                print(line, flush=True)
            if (cand[5] > 0.02 or cand[6] > 0.02 or cand[7] > 0.02) and (best_new is None or cand[0] < best_new[0]):
                best_new = cand
                print(f"[{time.time()-start:6.0f}s g{gen}] BEST-WITH-NEW-OP {label(cand)}", flush=True)
        elite = order[:ELITE]
        pick = elite[rndi(ELITE, P)]
        om = (om[pick] + 0.10 * rndn(P)).clamp(1.0, 1.8)
        lod = (lod[pick] + 0.0015 * rndn(P)).clamp(0.0, 0.014)
        sec = (sec[pick] + 0.02 * rndn(P)).clamp(0.0, 0.15)
        fq = torch.where(rnd(P) < 0.15, rndi(5, P) * 4, fq[pick])
        mom = (mom[pick] + 0.05 * rndn(P)).clamp(0.0, 0.8)
        stoch = (stoch[pick] + 0.03 * rndn(P)).clamp(0.0, 0.5)
        subk = (subk[pick] + 0.25 * rndn(P)).clamp(0.0, 4.0)
        nm = torch.where(rnd(P) < 0.15, rndi(3, P), nm[pick])
        gen += 1
        el = time.time() - start
        if el - last >= 300.0:
            last = el
            cps = P * gen / el
            b = best[0] if best else 0.0
            msg = f"[{el:6.0f}s] gen={gen} ({cps:.0f} cfg/s) | best {b:+.1f}%"
            print(msg, flush=True); post(msg)

    # ---- ablation: how much does each NEW operator contribute to the best config? ----
    def eval_one(cfg):
        g = (zc([cfg[1]]), zc([cfg[2]]), zc([cfg[3]]), zi([cfg[4]]), zc([cfg[5]]), zc([cfg[6]]), zc([cfg[7]]), zi([cfg[8]]))
        r, e = evaluate(eye, dir, ref_hit, ref_t, base, g)
        return float(r[0]), float(e[0])
    lines = [f"\n=== DONE after {time.time()-start:.0f}s, {gen} gens, {P*gen} configs ==="]
    if best:
        lines.append(f"BEST OVERALL: {label(best)}")
        full_r, full_e = eval_one(best)
        # zero each new op in turn (indices 5=mom, 6=stoch, 7=subitize) and remeasure.
        for idx, opname in [(5, 'momentum'), (6, 'stoch-omega'), (7, 'subitize')]:
            ab = list(best); ab[idx] = 0.0
            r0, e0 = eval_one(ab)
            lines.append(f"  ablate {opname:<12}: {r0:+.1f}% (err {e0:.4f})  vs full {full_r:+.1f}% -> "
                         f"operator worth {full_r - r0:+.1f}%")
    if best_new:
        lines.append(f"BEST USING A NEW OP: {label(best_new)}")
    fin = "\n".join(lines)
    print(fin, flush=True)
    post(fin + "\n=== KERNEL DONE ===")

if __name__ == "__main__":
    main()
