"""engine-mix-tpu.py -- JAX/TPU port of the engine-mix discovery search.

Same experiment as the torch versions (arc_engine_mix2.py on Intel Arc XPU, engine-mix-8h.py on
Kaggle P100 CUDA): evolve a population of marcher configs and score each on the ENGINE's real
field-eval currency (silhouette + depth error gated at <=0.012), across four new-domain operator
sources -- COGNITIVE (momentum, subitize/ANS), QUANTUM (stochastic-omega, phase-estimation
binary-refine), GLUE (a regime-blend adapter), SYMBOLS (a 2nd-order structural step).

Why JAX and not torch here: a TPU is a native-XLA device. JAX compiles the whole batched marcher to
one XLA program, runs the fixed-step inner loop as `lax.fori_loop` (no data-dependent break -- rays
are masked instead, which is what keeps XLA vectorized), and shards the population across all 8 TPU
cores with `pmap`. The operator MATH is a line-for-line port of the torch marcher so the two
implementations agree on the same scene/currency (guard against drift).

Precision note: a sphere-trace needs true float32 (epsilon ~6e-4). This kernel is almost entirely
elementwise (sqrt/min/max/select over the SDF) with no large matmuls, so float32 arrays run in
float32 on TPU; we still pin matmul precision to float32 defensively. We do NOT enable x64 (slow on
TPU and unnecessary).

Runs on Kaggle TPU (8 cores) but also on a single CPU/GPU device that JAX sees (pmap with 1 device),
so the logic is checkable off-TPU. Streams progress + every BEST config to the webhook relay so the
winning genome is never lost to the retention window (the lesson from the P100 run).
"""
import os, sys, subprocess, time, json, urllib.request
import numpy as np


# ---- TPU bootstrap: only self-heals a BROKEN env; never disturbs a working preinstall. --------
# Kaggle's TPU VM image usually ships JAX with the TPU already visible (import jax; jax.devices()
# -> 8 cores). If it does, we touch nothing. If JAX is missing or can't see a TPU, we install a
# matching jax[tpu]+libtpu and re-exec ONCE (a fresh process is the clean way to pick up new libtpu).
# The EMTPU_BOOT guard prevents an install/re-exec loop. Needs "enable_internet": true.
def _bootstrap_tpu():
    if os.environ.get("EMTPU_BOOT") == "1":
        return
    ok = False
    try:
        import jax
        ok = any(d.platform == "tpu" for d in jax.devices())
    except Exception:
        ok = False
    if not ok:
        subprocess.run([sys.executable, "-m", "pip", "install", "-q", "-U", "jax[tpu]",
                        "-f", "https://storage.googleapis.com/libtpu-releases/index.html"], check=False)
        os.environ["EMTPU_BOOT"] = "1"
        os.execv(sys.executable, [sys.executable] + sys.argv)


_bootstrap_tpu()

# ---- precision setup (before any jit) ----
import jax
jax.config.update("jax_default_matmul_precision", "float32")  # no-op unless the SDF ever uses matmul
import jax.numpy as jnp
from jax import lax

WEBHOOK = "https://webhook.site/ff06312d-a70d-42ec-870f-fa6acfc717c4"


def post(msg):
    try:
        req = urllib.request.Request(WEBHOOK, data=str(msg).encode("utf-8"),
                                     headers={"Content-Type": "text/plain"})
        urllib.request.urlopen(req, timeout=5)
    except Exception:
        pass


def parse_args():
    a = {"minutes": 450.0, "pop": 512, "res": (128, 72), "ncam": 12}
    v = sys.argv
    for i, x in enumerate(v):
        if x == "--minutes" and i + 1 < len(v): a["minutes"] = float(v[i + 1])
        if x == "--pop" and i + 1 < len(v): a["pop"] = int(v[i + 1])
        if x == "--res" and i + 1 < len(v):
            w, h = v[i + 1].split("x"); a["res"] = (int(w), int(h))
    return a


# =====================================================================================
# Scene SDF (identical geometry to arc_engine_mix2.py, expressed in jnp)
# =====================================================================================
def sdf(p):
    x, y, z = p[..., 0], p[..., 1], p[..., 2]
    d = y
    d = jnp.minimum(d, jnp.sqrt((x + 1.1) ** 2 + (y - 1.0) ** 2 + z ** 2) - 1.0)
    qx = jnp.abs(x - 1.2) - 0.6
    qy = jnp.abs(y - 0.9) - 0.6
    qz = jnp.abs(z - 0.3) - 0.6
    outside = jnp.sqrt(jnp.maximum(qx, 0.0) ** 2 + jnp.maximum(qy, 0.0) ** 2 + jnp.maximum(qz, 0.0) ** 2)
    inside = jnp.minimum(jnp.maximum(qx, jnp.maximum(qy, qz)), 0.0)
    d = jnp.minimum(d, outside + inside - 0.12)
    cx, cy, cz = x - 0.1, y - 0.6, z - 1.8
    t0 = jnp.sqrt(cx ** 2 + cz ** 2) - 0.7
    d = jnp.minimum(d, jnp.sqrt(t0 ** 2 + cy ** 2) - 0.25)
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
    return (np.concatenate(eyes, 0).astype(np.float32),
            np.concatenate(dirs, 0).astype(np.float32))


SHADOW = 24.0
MAX_STEPS = 160
REF_STEPS = 320
GENES = ["om", "lod", "sec", "fq", "mom", "stoch", "subk", "brefine", "glue", "sym2", "nm"]


# =====================================================================================
# The batched marcher -- a line-for-line jnp port of the torch v2 marcher.
# params: dict of (Pc,) float32 arrays (one chunk of the population).
# eye, dir: (N, 3) float32.  Returns hit (Pc,N) bool, t_hit (Pc,N), total_evals (Pc,).
# =====================================================================================
def march(params, eye, dir, max_steps):
    om = params["om"][:, None]
    lodc = params["lod"][:, None]
    secc = params["sec"][:, None]
    fqc = params["fq"][:, None]
    momc = params["mom"][:, None]
    stochc = params["stoch"][:, None]
    subkc = params["subk"][:, None]
    brefc = params["brefine"][:, None]
    gluec = params["glue"][:, None]
    sym2c = params["sym2"][:, None]
    nm = params["nm"][:, None]
    P = om.shape[0]
    N = dir.shape[0]
    eyeb = eye[None]          # (1,N,3)
    dirb = dir[None]          # (1,N,3)
    INF = jnp.float32(jnp.inf)
    z = jnp.zeros((P, N), jnp.float32)

    t = z
    active = jnp.ones((P, N), bool)
    omega = jnp.broadcast_to(om, (P, N)).astype(jnp.float32)
    prev_radius = z
    step_len = z
    d_prev = jnp.full((P, N), INF)
    d_prev2 = jnp.full((P, N), INF)
    t_prev = z
    crawl = z
    hit = jnp.zeros((P, N), bool)
    t_hit = z
    evals = jnp.zeros((P,), jnp.int32)  # max ~N*steps ~= 18M per config, well within int32

    def body(i, carry):
        (t, active, omega, prev_radius, step_len, d_prev, d_prev2, t_prev, crawl, hit, t_hit, evals) = carry
        pos = eyeb + t[..., None] * dirb
        d = sdf(pos)
        evals = evals + jnp.sum(active.astype(evals.dtype), axis=1)
        radius = jnp.abs(d)
        eps = 0.0006 * (1.0 + 0.5 * t) + lodc * t
        overshoot = (omega > 1.0) & (radius + prev_radius < step_len) & active
        newhit = active & (~overshoot) & (d < eps)
        # stochastic omega (quantum-walk)
        ang = t * 12.9898 + i.astype(jnp.float32) * 78.233
        s = jnp.sin(ang) * 43758.5453
        jitter = jnp.abs(s - jnp.trunc(s))  # torch.frac truncates toward zero (NOT floor) — parity
        omega_eff = jnp.clip(omega + stochc * (jitter - 0.5) * 2.0, 1.0, 2.0)
        step_b = d * omega_eff
        # momentum / predictive-coding (cognitive)
        approach = jnp.maximum(jnp.where(jnp.isfinite(d_prev), d_prev - d, 0.0), 0.0)
        step_b = step_b + momc * approach
        # symbols: 2nd-order structural step (curvature of d along the ray)
        fin2 = jnp.isfinite(d_prev) & jnp.isfinite(d_prev2)
        accel = jnp.where(fin2, d - 2.0 * d_prev + d_prev2, 0.0)
        step_b = step_b + sym2c * accel
        # subitize (ANS) with glue-smoothed far/near regime transition
        far_edge = eps * 6.0
        width = gluec * far_edge + 1e-6
        far_weight = jnp.clip((d - far_edge) / width, 0.0, 1.0)
        step_b = step_b * (1.0 + subkc * far_weight)
        # binary-refine (quantum phase-estimation): damp very near the surface
        near2 = d < 0.05
        step_b = jnp.where(near2 & (brefc > 0.0), step_b * (1.0 - 0.5 * brefc), step_b)
        # secant (numerical-opt) overrides near the surface
        use_sec = (secc > 0.0) & (~overshoot) & (d < secc) & jnp.isfinite(d_prev)
        dt = t - t_prev
        dd = d - d_prev
        sec_ok = use_sec & (dt > 1e-6) & (dd < -1e-6)
        dd_safe = jnp.where(dd == 0.0, -1e-9, dd)
        sec_step = jnp.minimum(jnp.maximum(jnp.maximum(-d * dt / dd_safe, 0.0), d), d * 4.0)
        step_b = jnp.where(sec_ok, sec_step, step_b)
        # fair-queue (queueing)
        near = (fqc > 0.0) & (~overshoot) & (~newhit) & (d < eps * 6.0)
        crawl = jnp.where(near, crawl + 1.0, 0.0)
        newhit = newhit | (active & near & (crawl >= fqc))
        step_len = jnp.where(overshoot, step_len * (1.0 - omega), step_b)
        omega = jnp.where(overshoot, 1.0, omega)
        just = newhit & active
        t_hit = jnp.where(just, t, t_hit)
        hit = hit | just
        active = active & (~just)
        prev_radius = radius
        d_prev2 = d_prev
        d_prev = d
        t_prev = t
        t = jnp.where(active, t + step_len, t)
        active = active & (t <= 30.0)
        return (t, active, omega, prev_radius, step_len, d_prev, d_prev2, t_prev, crawl, hit, t_hit, evals)

    carry0 = (t, active, omega, prev_radius, step_len, d_prev, d_prev2, t_prev, crawl, hit, t_hit, evals)
    carry = lax.fori_loop(0, max_steps, body, carry0)
    (t, active, omega, prev_radius, step_len, d_prev, d_prev2, t_prev, crawl, hit, t_hit, evals) = carry
    nh = jnp.sum(hit.astype(evals.dtype), axis=1)
    ncost = jnp.where(nm[:, 0] > 1.5, 0, jnp.where(nm[:, 0] > 0.5, 3, 4)).astype(evals.dtype)
    total = evals + nh * ncost + nh * jnp.int32(SHADOW)
    return hit, t_hit, total


if __name__ == "__main__":
    args = parse_args()
    devs = jax.devices()
    ncore = len(devs)
    plat = devs[0].platform
    w, h = args["res"]
    ncam = args["ncam"]

    eye_np, dir_np = make_rays(ncam, w, h)
    eye = jnp.asarray(eye_np)
    dir = jnp.asarray(dir_np)
    N = dir.shape[0]

    # population rounded to a multiple of the core count for pmap
    P = max(ncore, (args["pop"] // ncore) * ncore)
    ELITE = max(8, P // 16)
    TOL = 0.012

    # ---- ground-truth reference: stock config, high step count, one device ----
    stock = {g: jnp.asarray(np.array([v], np.float32)) for g, v in
             zip(GENES, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])}
    ref_hit, ref_t, base_ev = jax.jit(lambda pr: march(pr, eye, dir, REF_STEPS))(stock)
    ref_hit = ref_hit[0]
    ref_t = ref_t[0]
    _, _, base_ev = jax.jit(lambda pr: march(pr, eye, dir, MAX_STEPS))(stock)
    base = int(np.asarray(base_ev)[0])

    def evaluate(params, eye, dir, ref_hit, ref_t):
        hit, t_hit, total = march(params, eye, dir, MAX_STEPS)
        sil = jnp.mean((hit != ref_hit[None]).astype(jnp.float32), axis=1)
        both = hit & ref_hit[None]
        depth = (jnp.sum(jnp.where(both, jnp.abs(t_hit - ref_t[None]) / (ref_t[None] + 1e-3), 0.0), axis=1)
                 / jnp.maximum(jnp.sum(both.astype(jnp.float32), axis=1), 1.0))
        redux = (total.astype(jnp.float32) - base) / base * 100.0
        return redux, sil + 0.3 * depth

    # pmap shards the population axis across all cores (in_axes: params sharded, rays/ref replicated).
    # This kernel is embarrassingly parallel over configs with zero cross-ray communication, so pmap
    # is performance-equivalent to the newer jit+NamedSharding path here and is the more conservative,
    # battle-tested API for an untested-locally deploy. (Modern alt: jax.make_mesh((ncore,),('pop',))
    # + NamedSharding(mesh, P_('pop',None)) + jax.jit.)
    peval = jax.pmap(evaluate, in_axes=(0, None, None, None, None))

    def eval_pop(pdict):
        # reshape each (P,) gene to (ncore, Pc) for pmap
        chunk = {g: jnp.asarray(pdict[g].reshape(ncore, P // ncore)) for g in GENES}
        redux, err = peval(chunk, eye, dir, ref_hit, ref_t)
        return np.asarray(redux).reshape(P), np.asarray(err).reshape(P)

    def eval_one(cfg):
        pd = {g: jnp.asarray(np.array([cfg[g]], np.float32).reshape(1, 1)) for g in GENES}
        redux, err = peval(pd, eye, dir, ref_hit, ref_t)
        return float(np.asarray(redux).reshape(-1)[0]), float(np.asarray(err).reshape(-1)[0])

    rng = np.random.default_rng(1234)
    pop = {
        "om": np.clip(1.0 + 0.8 * rng.random(P), 1.0, 1.8),
        "lod": 0.012 * rng.random(P),
        "sec": np.where(rng.random(P) < 0.6, 0.02 + 0.13 * rng.random(P), 0.0),
        "fq": (rng.integers(0, 5, P) * 4).astype(np.float32),
        "mom": np.where(rng.random(P) < 0.4, 0.6 * rng.random(P), 0.0),
        "stoch": np.where(rng.random(P) < 0.4, 0.3 * rng.random(P), 0.0),
        "subk": np.where(rng.random(P) < 0.6, 3.0 * rng.random(P), 0.0),
        "brefine": np.where(rng.random(P) < 0.4, rng.random(P), 0.0),
        "glue": np.where(rng.random(P) < 0.4, 2.0 * rng.random(P), 0.0),
        "sym2": np.where(rng.random(P) < 0.4, 0.5 * rng.random(P), 0.0),
        "nm": rng.integers(0, 3, P).astype(np.float32),
    }
    pop = {g: pop[g].astype(np.float32) for g in GENES}
    modes = {0: "tetra", 1: "fwd3", 2: "dual"}

    def label(cfg, r):
        return (f"{r:+.1f}% | omega={cfg['om']:.2f} lod={cfg['lod']:.4f} sec={cfg['sec']:.3f} "
                f"fq={int(cfg['fq'])} normal={modes[int(round(cfg['nm']))]} | "
                f"COG mom={cfg['mom']:.3f} subitize={cfg['subk']:.3f} | "
                f"QUANT stoch={cfg['stoch']:.3f} brefine={cfg['brefine']:.3f} | "
                f"GLUE glue={cfg['glue']:.3f} | SYM sym2={cfg['sym2']:.3f}")

    hdr = (f"=== ENGINE-MIX-TPU START | {ncore}x {plat.upper()} | {N} rays ({ncam} cams {w}x{h}) | "
           f"pop={P} | budget {args['minutes']:.0f} min | stock baseline {base} field-evals ===")
    print(hdr, flush=True)
    post(hdr)
    post("engine-mix-tpu: COGNITIVE(momentum,subitize) + QUANTUM(stoch,brefine) + GLUE(regime-blend) "
         "+ SYMBOLS(2nd-order), scored on the real marcher currency at err<=0.012.")

    best = None
    start = time.time()
    budget = args["minutes"] * 60.0
    gen = 0
    last = 0.0
    while time.time() - start < budget:
        redux, err = eval_pop(pop)
        ok = err <= TOL
        score = np.where(ok, redux, 1e6)
        order = np.argsort(score)
        bi = int(order[0])
        if ok[bi]:
            cfg = {g: float(pop[g][bi]) for g in GENES}
            if best is None or redux[bi] < best[0]:
                best = (float(redux[bi]), cfg)
                line = f"[{time.time()-start:6.0f}s g{gen}] BEST {label(cfg, redux[bi])}"
                print(line, flush=True)
                post(line)
        elite = order[:ELITE]
        parents = elite[rng.integers(0, ELITE, P)]

        def mut(g, scale, lo, hi):
            return np.clip(pop[g][parents] + scale * rng.standard_normal(P), lo, hi).astype(np.float32)

        newpop = {
            "om": mut("om", 0.10, 1.0, 1.8),
            "lod": mut("lod", 0.0015, 0.0, 0.014),
            "sec": mut("sec", 0.02, 0.0, 0.15),
            "fq": np.where(rng.random(P) < 0.15, (rng.integers(0, 5, P) * 4), pop["fq"][parents]).astype(np.float32),
            "mom": mut("mom", 0.05, 0.0, 0.8),
            "stoch": mut("stoch", 0.03, 0.0, 0.5),
            "subk": mut("subk", 0.25, 0.0, 4.0),
            "brefine": mut("brefine", 0.08, 0.0, 1.0),
            "glue": mut("glue", 0.15, 0.0, 3.0),
            "sym2": mut("sym2", 0.04, 0.0, 0.6),
            "nm": np.where(rng.random(P) < 0.15, rng.integers(0, 3, P), pop["nm"][parents]).astype(np.float32),
        }
        pop = newpop
        gen += 1
        el = time.time() - start
        if el - last >= 120.0:
            last = el
            cps = P * gen / el
            b = best[0] if best else 0.0
            hb = f"[{el:6.0f}s] gen={gen} ({cps:.0f} cfg/s) | best {b:+.1f}%"
            print(hb, flush=True)
            post(hb)

    # ---- per-domain ablation on the best config ----
    print(f"\n=== DONE after {time.time()-start:.0f}s, {gen} gens, {P*gen} configs ===", flush=True)
    if best:
        full_r, best_cfg = best[0], best[1]
        head = f"BEST OVERALL: {label(best_cfg, full_r)}"
        print(head, flush=True)
        post(head)
        abl = ["Per-domain ablation (zero each operator on the best config, remeasure):"]
        for gene, dom, opn in [("subk", "COGNITIVE", "subitize"), ("mom", "COGNITIVE", "momentum"),
                               ("stoch", "QUANTUM", "stoch-omega"), ("brefine", "QUANTUM", "binary-refine"),
                               ("glue", "GLUE", "regime-blend"), ("sym2", "SYMBOLS", "2nd-order")]:
            ab = dict(best_cfg); ab[gene] = 0.0
            r0, e0 = eval_one(ab)
            abl.append(f"  {dom:<10} {opn:<14}: {r0:+.1f}% (err {e0:.4f}) -> worth {r0 - full_r:+.1f}% (positive = helped)")
        text = "\n".join(abl)
        print(text, flush=True)
        post(text)
        post("=== KERNEL DONE ===")
    else:
        print("no valid config held image quality.", flush=True)
        post("engine-mix-tpu: no valid config held image quality.")
