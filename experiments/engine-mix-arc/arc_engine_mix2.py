"""Engine-mix v2 (2h Arc): broaden the operator genome across FOUR new-domain sources and measure
each against the engine's real field-eval currency, with a per-domain ablation.

  COGNITIVE:  subitize  (ANS "leap when clearly far")            -- proven real winner already
              momentum  (predictive-coding, 1st-order extrapolate)
  QUANTUM:    stoch     (quantum-walk, jitter omega per step)
              brefine   (phase-estimation-style damped near-surface convergence)
  GLUE:       glue      (an ADAPTER: smooth the subitize far/near regime switch instead of a hard
                         gate -- glue "binds" the two stepping regimes; may reduce subitize's edge error)
  SYMBOLS:    sym2      (a 2nd-order STRUCTURAL/invariant step from the curvature of d along the ray --
                         the ray-SDF is rank-1, sym2 uses its 2nd difference)

Honest note: cognitive/quantum map cleanly to marcher mechanisms; glue/symbols are meta-layers, so
those two are exploratory -- the ablation shows whether they earn their place or are dead weight.

Every config measured on the real tensor marcher: field-evals + silhouette/depth error gate (<=0.012).
Arc XPU only (never CUDA -- RTX stays free).
"""
import torch, numpy as np, time, sys

def pick_device():
    try:
        if hasattr(torch, 'xpu') and torch.xpu.is_available():
            return torch.device('xpu'), torch.xpu.get_device_name(0)
    except Exception:
        pass
    return torch.device('cpu'), 'cpu'

def parse_args():
    a = {'minutes': 120.0, 'pop': 256, 'res': (96, 54)}
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

def march(eye, dir, g, max_steps=160):
    om, lod, sec, fq, mom, stoch, subk, brefine, glue, sym2, nmode = g
    P, N, dev = om.shape[0], dir.shape[0], eye.device
    z = lambda *s: torch.zeros(*s).to(dev)
    t = z(P, N)
    active = torch.ones((P, N), dtype=torch.bool).to(dev)
    prev_radius = z(P, N)
    step_len = z(P, N)
    omega = om[:, None].expand(P, N).clone()
    lodc, sthr, fqc = lod[:, None], sec[:, None], fq[:, None]
    momc, stochc, subkc = mom[:, None], stoch[:, None], subk[:, None]
    brefc, gluec, sym2c = brefine[:, None], glue[:, None], sym2[:, None]
    crawl = z(P, N)
    d_prev = torch.full((P, N), float('inf')).to(dev)
    d_prev2 = torch.full((P, N), float('inf')).to(dev)
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
        # stochastic omega (quantum-walk)
        jitter = torch.frac(torch.sin(t * 12.9898 + float(i) * 78.233) * 43758.5453).abs()
        omega_eff = (omega + stochc * (jitter - 0.5) * 2.0).clamp(1.0, 2.0)
        step_b = d * omega_eff
        # momentum / predictive-coding (cognitive, 1st-order)
        approach = torch.clamp(torch.where(torch.isfinite(d_prev), d_prev - d, torch.zeros_like(d)), min=0.0)
        step_b = step_b + momc * approach
        # symbol 2nd-order structural step: curvature of d along the ray (rank-1 field's 2nd diff)
        fin2 = torch.isfinite(d_prev) & torch.isfinite(d_prev2)
        accel = torch.where(fin2, d - 2.0 * d_prev + d_prev2, torch.zeros_like(d))
        step_b = step_b + sym2c * accel
        # subitize (ANS) with GLUE-smoothed far/near regime transition
        far_edge = eps * 6.0
        width = gluec * far_edge + 1e-6
        far_weight = torch.clamp((d - far_edge) / width, 0.0, 1.0)
        step_b = step_b * (1.0 + subkc * far_weight)
        # binary-refine (quantum phase-estimation): damp the step very near the surface
        near2 = d < 0.05
        step_b = torch.where(near2 & (brefc > 0.0), step_b * (1.0 - 0.5 * brefc), step_b)
        # secant (numerical-opt) overrides near the surface
        use_sec = (sthr > 0) & (~overshoot) & (d < sthr) & torch.isfinite(d_prev)
        dt = t - t_prev
        dd = d - d_prev
        sec_ok = use_sec & (dt > 1e-6) & (dd < -1e-6)
        sec_step = torch.minimum(torch.maximum(torch.clamp(-d * dt / torch.where(dd == 0, torch.full_like(dd, -1e-9), dd), min=0), d), d * 4.0)
        step_b = torch.where(sec_ok, sec_step, step_b)
        # fair-queue (queueing)
        near = (fqc > 0) & (~overshoot) & (~newhit) & (d < eps * 6.0)
        crawl = torch.where(near, crawl + 1, torch.zeros_like(crawl))
        newhit = newhit | (active & near & (crawl >= fqc.float()))
        step_len = torch.where(overshoot, step_len * (1.0 - omega), step_b)
        omega = torch.where(overshoot, torch.ones_like(omega), omega)
        just = newhit & active
        t_hit = torch.where(just, t, t_hit)
        hit = hit | just
        active = active & (~just)
        prev_radius = radius
        d_prev2 = d_prev
        d_prev = d
        t_prev = t
        t = torch.where(active, t + step_len, t)
        active = active & (t <= 30.0)
    nh = hit.sum(dim=1)
    ncost = torch.where(nmode == 2, torch.zeros_like(om), torch.where(nmode == 1, torch.full_like(om, 3.0), torch.full_like(om, 4.0))).to(torch.int64)
    total = evals + nh * ncost + nh * SHADOW
    return hit, t_hit, total

def evaluate(eye, dir, ref_hit, ref_t, base, g, max_steps=160):
    hit, t_hit, ev = march(eye, dir, g, max_steps)
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
    off = lambda: torch.zeros(1).to(dev)

    def gtuple(P):
        return None
    stock = (zc([1.0]), zc([0.0]), zc([0.0]), zi([0]), zc([0.0]), zc([0.0]), zc([0.0]), zc([0.0]), zc([0.0]), zc([0.0]), zi([0]))
    ref_hit, ref_t, _ = march(eye, dir, stock, 320)
    ref_hit, ref_t = ref_hit[0], ref_t[0]
    _, _, base_ev = march(eye, dir, stock)
    base = int(base_ev[0])

    P, ELITE, TOL = args['pop'], max(8, args['pop'] // 16), 0.012
    om = (1.0 + 0.8 * rnd(P)).clamp(1.0, 1.8)
    lod = 0.012 * rnd(P)
    sec = torch.where(rnd(P) < 0.6, 0.02 + 0.13 * rnd(P), torch.zeros(P).to(dev))
    fq = rndi(5, P) * 4
    mom = torch.where(rnd(P) < 0.4, 0.6 * rnd(P), torch.zeros(P).to(dev))
    stoch = torch.where(rnd(P) < 0.4, 0.3 * rnd(P), torch.zeros(P).to(dev))
    subk = torch.where(rnd(P) < 0.6, 3.0 * rnd(P), torch.zeros(P).to(dev))
    brefine = torch.where(rnd(P) < 0.4, rnd(P), torch.zeros(P).to(dev))
    glue = torch.where(rnd(P) < 0.4, 2.0 * rnd(P), torch.zeros(P).to(dev))
    sym2 = torch.where(rnd(P) < 0.4, 0.5 * rnd(P), torch.zeros(P).to(dev))
    nm = rndi(3, P)
    modes = {0: 'tetra', 1: 'fwd3', 2: 'dual'}

    def genome():
        return (om, lod, sec, fq, mom, stoch, subk, brefine, glue, sym2, nm)

    def label(c):
        r, o, l, s, f, mo, st, su, br, gl, sy, n = c
        return (f"{r:+.1f}% | omega={o:.2f} lod={l:.4f} sec={s:.3f} fq={f} normal={modes[n]}\n"
                f"    COG mom={mo:.3f} subitize={su:.3f} | QUANT stoch={st:.3f} brefine={br:.3f} | "
                f"GLUE glue={gl:.3f} | SYM sym2={sy:.3f}")

    print(f"=== ARC ENGINE-MIX v2 START | device={name} | {N} rays ({ncam} cams {w}x{h}) | pop={P} | "
          f"budget {args['minutes']:.0f} min | stock baseline {base} field-evals ===", flush=True)
    print("Genome spans COGNITIVE (momentum, subitize) + QUANTUM (stoch, brefine) + GLUE (regime blend) "
          "+ SYMBOLS (2nd-order structure), all measured on the real marcher at err<=0.012.", flush=True)

    best = None
    start = time.time()
    budget = args['minutes'] * 60.0
    gen = 0
    last = 0.0
    while time.time() - start < budget:
        redux, err = evaluate(eye, dir, ref_hit, ref_t, base, genome())
        ok = err <= TOL
        score = torch.where(ok, redux, torch.full_like(redux, 1e6))
        order = torch.argsort(score.detach().to('cpu')).to(dev)
        bi = int(order[0].item())
        if bool(ok[bi]):
            cand = (float(redux[bi]), float(om[bi]), float(lod[bi]), float(sec[bi]), int(fq[bi]),
                    float(mom[bi]), float(stoch[bi]), float(subk[bi]), float(brefine[bi]),
                    float(glue[bi]), float(sym2[bi]), int(nm[bi]))
            if best is None or cand[0] < best[0]:
                best = cand
                print(f"[{time.time()-start:6.0f}s g{gen}] BEST {label(cand)}", flush=True)
        elite = order[:ELITE]
        pick = elite[rndi(ELITE, P)]
        om = (om[pick] + 0.10 * rndn(P)).clamp(1.0, 1.8)
        lod = (lod[pick] + 0.0015 * rndn(P)).clamp(0.0, 0.014)
        sec = (sec[pick] + 0.02 * rndn(P)).clamp(0.0, 0.15)
        fq = torch.where(rnd(P) < 0.15, rndi(5, P) * 4, fq[pick])
        mom = (mom[pick] + 0.05 * rndn(P)).clamp(0.0, 0.8)
        stoch = (stoch[pick] + 0.03 * rndn(P)).clamp(0.0, 0.5)
        subk = (subk[pick] + 0.25 * rndn(P)).clamp(0.0, 4.0)
        brefine = (brefine[pick] + 0.08 * rndn(P)).clamp(0.0, 1.0)
        glue = (glue[pick] + 0.15 * rndn(P)).clamp(0.0, 3.0)
        sym2 = (sym2[pick] + 0.04 * rndn(P)).clamp(0.0, 0.6)
        nm = torch.where(rnd(P) < 0.15, rndi(3, P), nm[pick])
        gen += 1
        el = time.time() - start
        if el - last >= 120.0:
            last = el
            cps = P * gen / el
            b = best[0] if best else 0.0
            print(f"[{el:6.0f}s] .. gen={gen} ({cps:.0f} cfg/s) | best {b:+.1f}%", flush=True)

    # ---- per-domain ablation on the best config ----
    def eval_one(c):
        g = (zc([c[1]]), zc([c[2]]), zc([c[3]]), zi([c[4]]), zc([c[5]]), zc([c[6]]), zc([c[7]]),
             zc([c[8]]), zc([c[9]]), zc([c[10]]), zi([c[11]]))
        r, e = evaluate(eye, dir, ref_hit, ref_t, base, g)
        return float(r[0]), float(e[0])

    print(f"\n=== DONE after {time.time()-start:.0f}s, {gen} gens, {P*gen} configs ===", flush=True)
    if best:
        print(f"BEST OVERALL: {label(best)}", flush=True)
        full_r, _ = eval_one(best)
        print(f"\nPer-domain ablation (zero each operator on the best config, remeasure):", flush=True)
        for idx, dom, opn in [(7, 'COGNITIVE', 'subitize'), (5, 'COGNITIVE', 'momentum'),
                              (6, 'QUANTUM', 'stoch-omega'), (8, 'QUANTUM', 'binary-refine'),
                              (9, 'GLUE', 'regime-blend'), (10, 'SYMBOLS', '2nd-order')]:
            ab = list(best); ab[idx] = 0.0
            r0, e0 = eval_one(ab)
            print(f"  {dom:<10} {opn:<14}: {r0:+.1f}% (err {e0:.4f}) -> worth {full_r - r0:+.1f}%", flush=True)
    else:
        print("no valid config held image quality.", flush=True)

if __name__ == "__main__":
    main()
