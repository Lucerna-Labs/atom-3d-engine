"""Mix new-domain MECHANISMS into the engine marcher and measure them against the engine's real
currency (field-evals at held image quality). Not a single-domain search — an expanded operator
genome where three NEW candidate operators, each abstracted (via the 8 root atoms) from a mechanism
in the freshly-added domains, compete alongside the shipped operators on the real tensor marcher:

  momentum_beta   <- predictive-coding (cognitive-primitives) / momentum (optimization):
                     extrapolate the distance trend, step further when the field is shrinking smoothly.
  stoch_omega     <- quantum-walk / Monte-Carlo (quantum + statistics): jitter the over-relaxation
                     factor per ray per step (deterministic hash), a stochastic enhanced sphere-trace.
  subitize_k      <- the Approximate Number System's "instantly recognize it's far" (numerical
                     cognition): an extra empty-space leap when the distance is clearly large.

Shipped operators also in the genome (the baseline they must beat): omega (over-relax), lod_k (LOD),
secant (root refine), fair_q (fair-queue crawl), normal_mode (tetra/fwd3/dual — dual now shipped).

Every config is measured on the REAL marcher: field-eval count + silhouette/depth image error vs a
high-step omega=1 reference. The image-error budget (0.012) rejects anything that tunnels or misses,
so a "win" is a real field-eval cut at held correctness. The report tracks the best config overall
AND the best config that actually USES a new operator — the honest answer to "do they help?".

Arc XPU only (never CUDA — the RTX stays free). Run in the Arc venv.
"""
import torch, numpy as np, time, sys

def pick_device():
    try:
        if hasattr(torch, 'xpu') and torch.xpu.is_available():
            return torch.device('xpu'), torch.xpu.get_device_name(0)
    except Exception:
        pass
    return torch.device('cpu'), 'cpu'  # never cuda

def parse_args():
    a = {'minutes': 60.0, 'pop': 256, 'res': (96, 54)}
    v = sys.argv
    for i, x in enumerate(v):
        if x == '--minutes' and i + 1 < len(v): a['minutes'] = float(v[i + 1])
        if x == '--pop' and i + 1 < len(v): a['pop'] = int(v[i + 1])
        if x == '--res' and i + 1 < len(v):
            w, h = v[i + 1].split('x'); a['res'] = (int(w), int(h))
    return a

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

def march(eye, dir, om, lod, sec, fq, mom, stoch, subk, nmode, max_steps=160):
    """Expanded tensor marcher. om,lod,sec,mom,stoch,subk float [P]; fq,nmode int [P]."""
    P, N, dev = om.shape[0], dir.shape[0], eye.device
    z = lambda *s: torch.zeros(*s).to(dev)
    t = z(P, N)
    active = torch.ones((P, N), dtype=torch.bool).to(dev)
    prev_radius = z(P, N)
    step_len = z(P, N)
    omega = om[:, None].expand(P, N).clone()
    lodc, sthr, fqc = lod[:, None], sec[:, None], fq[:, None]
    momc, stochc, subkc = mom[:, None], stoch[:, None], subk[:, None]
    crawl = z(P, N)
    d_prev = torch.full((P, N), float('inf')).to(dev)
    t_prev = z(P, N)
    hit = torch.zeros((P, N), dtype=torch.bool).to(dev)
    t_hit = z(P, N)
    evals = torch.zeros(P, dtype=torch.int64).to(dev)
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
        # stochastic over-relaxation (quantum-walk): deterministic per-(ray,step) jitter of omega.
        jitter = torch.frac(torch.sin(t * 12.9898 + float(i) * 78.233) * 43758.5453).abs()
        omega_eff = (omega + stochc * (jitter - 0.5) * 2.0).clamp(1.0, 2.0)
        step_b = d * omega_eff
        # momentum / predictive-coding: add a fraction of the approach rate (how fast d shrinks).
        approach = torch.clamp(torch.where(torch.isfinite(d_prev), d_prev - d, torch.zeros_like(d)), min=0.0)
        step_b = step_b + momc * approach
        # subitize (ANS): an extra leap when the distance is clearly far.
        far = d > (eps * 6.0)
        step_b = torch.where(far, step_b * (1.0 + subkc), step_b)
        # secant near-surface refinement (numerical-opt).
        use_sec = (sthr > 0) & (~overshoot) & (d < sthr) & torch.isfinite(d_prev)
        dt = t - t_prev
        dd = d - d_prev
        sec_ok = use_sec & (dt > 1e-6) & (dd < -1e-6)
        sec_step = torch.minimum(torch.maximum(torch.clamp(-d * dt / torch.where(dd == 0, torch.full_like(dd, -1e-9), dd), min=0), d), d * 4.0)
        step_b = torch.where(sec_ok, sec_step, step_b)
        # fair-queue crawl (queueing).
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

def evaluate(eye, dir, ref_hit, ref_t, base, g, max_steps=160):
    om, lod, sec, fq, mom, stoch, subk, nm = g
    hit, t_hit, ev = march(eye, dir, om, lod, sec, fq, mom, stoch, subk, nm, max_steps)
    sil = (hit != ref_hit[None]).float().mean(dim=1)
    both = hit & ref_hit[None]
    depth = (torch.where(both, (t_hit - ref_t[None]).abs() / (ref_t[None] + 1e-3), torch.zeros_like(t_hit)).sum(dim=1)
             / both.sum(dim=1).clamp(min=1))
    return (ev - base).float() / base * 100.0, sil + 0.3 * depth

def main():
    args = parse_args()
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass
    dev, name = pick_device()
    ncam, (w, h) = 12, args['res']
    eye_np, dir_np = make_rays(ncam, w, h)
    eye = torch.tensor(eye_np, dtype=torch.float32).to(dev)
    dir = torch.tensor(dir_np, dtype=torch.float32).to(dev)
    N = dir.shape[0]
    zc = lambda v: torch.tensor(v, dtype=torch.float32).to(dev)
    zi = lambda v: torch.tensor(v, dtype=torch.int64).to(dev)
    rnd = lambda *s: torch.rand(*s).to(dev)
    rndn = lambda *s: torch.randn(*s).to(dev)
    rndi = lambda hi, *s: torch.randint(0, hi, s).to(dev)

    # reference (exact) + stock baseline: omega=1, no operators, tetra normal.
    ref_hit, ref_t, _ = march(eye, dir, zc([1.0]), zc([0.0]), zc([0.0]), zi([0]), zc([0.0]), zc([0.0]), zc([0.0]), zi([0]), 320)
    ref_hit, ref_t = ref_hit[0], ref_t[0]
    _, _, base_ev = march(eye, dir, zc([1.0]), zc([0.0]), zc([0.0]), zi([0]), zc([0.0]), zc([0.0]), zc([0.0]), zi([0]))
    base = int(base_ev[0])

    P, ELITE, TOL = args['pop'], max(8, args['pop'] // 16), 0.012
    # genome
    om = (1.0 + 0.8 * rnd(P)).clamp(1.0, 1.8)
    lod = 0.012 * rnd(P)
    sec = torch.where(rnd(P) < 0.6, 0.02 + 0.13 * rnd(P), torch.zeros(P).to(dev))
    fq = rndi(5, P) * 4
    mom = torch.where(rnd(P) < 0.5, 0.6 * rnd(P), torch.zeros(P).to(dev))
    stoch = torch.where(rnd(P) < 0.5, 0.3 * rnd(P), torch.zeros(P).to(dev))
    subk = torch.where(rnd(P) < 0.5, 3.0 * rnd(P), torch.zeros(P).to(dev))
    nm = rndi(3, P)
    modes = {0: 'tetra', 1: 'fwd3', 2: 'dual'}

    def label(t):
        r, o, l, s, f, mo, st, su, n = t
        return (f"{r:+.1f}% | omega={o:.2f} lod={l:.4f} sec={s:.3f} fq={f} normal={modes[n]} | "
                f"NEW: mom={mo:.3f} stoch={st:.3f} subitize={su:.3f}")

    print(f"=== ARC ENGINE-MIX START | device={name} | {N} rays ({ncam} cams {w}x{h}) | pop={P} | "
          f"budget {args['minutes']:.0f} min | stock baseline {base} field-evals ===", flush=True)
    print("Genome = shipped ops (omega/lod/secant/fairq/normal) + NEW domain-inspired ops "
          "(mom=predictive-coding, stoch=quantum-walk, subitize=ANS). A 'win' = real field-eval cut "
          "at image-err <= 0.012 on the actual marcher.", flush=True)

    best = None            # overall
    best_new = None        # best config that actually uses a new op (mom/stoch/subitize > 0.02)
    start = time.time()
    budget = args['minutes'] * 60.0
    gen = 0
    last = 0.0
    while time.time() - start < budget:
        redux, err = evaluate(eye, dir, ref_hit, ref_t, base, (om, lod, sec, fq, mom, stoch, subk, nm))
        ok = err <= TOL
        score = torch.where(ok, redux, torch.full_like(redux, 1e6))
        order = torch.argsort(score.detach().to('cpu')).to(dev)
        bi = int(order[0].item())
        if bool(ok[bi]):
            cand = (float(redux[bi]), float(om[bi]), float(lod[bi]), float(sec[bi]), int(fq[bi]),
                    float(mom[bi]), float(stoch[bi]), float(subk[bi]), int(nm[bi]))
            if best is None or cand[0] < best[0]:
                best = cand
                print(f"[{time.time()-start:6.0f}s g{gen}] BEST {label(cand)}", flush=True)
            uses_new = cand[5] > 0.02 or cand[6] > 0.02 or cand[7] > 0.02
            if uses_new and (best_new is None or cand[0] < best_new[0]):
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
        if el - last >= 60.0:
            last = el
            cps = P * gen / el
            b = best[0] if best else 0.0
            print(f"[{el:6.0f}s] .. gen={gen} ({cps:.0f} cfg/s) | best {b:+.1f}%", flush=True)

    print(f"\n=== DONE after {time.time()-start:.0f}s, {gen} gens, {P*gen} configs ===", flush=True)
    print(f"BEST OVERALL:        {label(best)}" if best else "no valid config", flush=True)
    if best_new:
        print(f"BEST USING A NEW OP: {label(best_new)}", flush=True)
        gap = best_new[0] - best[0]
        verdict = ("the new domain-inspired ops did NOT help — best-with-new is worse than best-without"
                   if gap > 0.3 else
                   "a new domain-inspired op is competitive with / part of the best config")
        print(f"VERDICT: {verdict} (gap {gap:+.1f}% vs overall best).", flush=True)
    else:
        print("BEST USING A NEW OP: none held image quality — the new ops did not survive the error gate.", flush=True)

if __name__ == "__main__":
    main()
