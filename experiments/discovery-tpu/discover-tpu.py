"""discover-tpu.py -- a TRUE discovery engine for the marcher step function, on Kaggle TPU (JAX).

NOT a confirmation engine. The confirmation sims (arc_engine_mix*.py, engine-mix-tpu.py) evolve the
PARAMETERS of a fixed genome that already contains the known winners (subitize, over-relax, secant...)
-- so they can only re-find what was put in. This one is handed NOTHING. It synthesises PROGRAMS.

Each individual is a small linear-genetic-program: a sequence of L instructions (op, a, b, dst) over a
bank of R registers, plus a few evolved constants. Every march step the registers are seeded with raw
signals (d, d_prev, eps, sphere-overlap, approach, curvature, t, step-index, constants); the program
computes the step length from them; the marcher takes that step. The op-set is the ROOT ATOMS the
cross-domain primitive library is built from -- add/sub/mul/safe-div, min/max/order, abs/fold, sqrt,
sin, the crypto HASH atom, compare->indicator, conditional GLUE. So: atoms = primitives, the wiring
(operand/dst indices) = GLUE, the register discipline + the safe-distance invariant = SYMBOLS.

Why this discovers instead of confirms:
  * Subitize isn't a knob here -- it is ONE reachable program (gt(d, c*eps) -> mul) among an
    astronomical space. The search must INVENT it, beat it, or find something else.
  * A no-op program is exactly naive sphere-tracing (the output register seeds to d), so the search
    STARTS at the naive baseline and has to earn every gain.
  * Safety is enforced by the CURRENCY, not a baked-in guard: tunneling programs miss the silhouette/
    depth gate and die. The only priors are numerical (finite, non-negative, bounded step).
  * The winning program is DECODED to a human-readable formula, so a discovery is shippable like
    subitize was (implement in the real Rust marcher, wall-clock test) -- not a black box.

TPU via JAX: the population is sharded across the 8 cores (pmap); the march is a lax.fori_loop; the
per-step program is a small batched interpreter (take_along_axis gathers + one-hot writes), all float32.
Runs unchanged on a single CPU device (pmap over 1 device) so the LOGIC is smoke-tested off-TPU first.
"""
import os, sys, subprocess, time, urllib.request
import numpy as np


# ---- TPU bootstrap: fires ONLY on Kaggle, and only if a TPU isn't already visible. Never touches a
# local CPU smoke-test env (would wrongly pip-install jax[tpu]). --------------------------------------
def _bootstrap_tpu():
    on_kaggle = bool(os.environ.get("KAGGLE_KERNEL_RUN_TYPE") or os.environ.get("KAGGLE_URL_BASE")
                     or os.environ.get("KAGGLE_DATA_PROXY_TOKEN"))
    if not on_kaggle or os.environ.get("EMTPU_BOOT") == "1":
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

import jax
jax.config.update("jax_default_matmul_precision", "float32")  # no-op unless something uses matmul
import jax.numpy as jnp
from jax import lax

WEBHOOK = "https://webhook.site/ff06312d-a70d-42ec-870f-fa6acfc717c4"


def post(msg):
    if os.environ.get("DISCOVER_NO_WEBHOOK") == "1":
        return
    try:
        req = urllib.request.Request(WEBHOOK, data=str(msg).encode("utf-8"),
                                     headers={"Content-Type": "text/plain"})
        urllib.request.urlopen(req, timeout=5)
    except Exception:
        pass


def parse_args():
    a = {"minutes": 450.0, "pop": 256, "res": (80, 45), "ncam": 6, "steps": 128, "L": 10}
    v = sys.argv
    for i, x in enumerate(v):
        if x == "--minutes" and i + 1 < len(v): a["minutes"] = float(v[i + 1])
        if x == "--pop" and i + 1 < len(v): a["pop"] = int(v[i + 1])
        if x == "--ncam" and i + 1 < len(v): a["ncam"] = int(v[i + 1])
        if x == "--steps" and i + 1 < len(v): a["steps"] = int(v[i + 1])
        if x == "--L" and i + 1 < len(v): a["L"] = int(v[i + 1])
        if x == "--res" and i + 1 < len(v):
            w, h = v[i + 1].split("x"); a["res"] = (int(w), int(h))
    return a


# =====================================================================================
# Scene + rays (identical geometry to the other sims)
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
    return (np.concatenate(eyes, 0).astype(np.float32), np.concatenate(dirs, 0).astype(np.float32))


# =====================================================================================
# Op-set (root atoms) + interpreter. K ops; register bank of R; C evolved constants.
# =====================================================================================
R = 14          # registers: 10 signals + C constants (see seeding below)
C = 4           # evolved constants per program
K = 14          # number of ops
# arity table + human names, kept in lock-step with the cand stack in run_program and the decoder.
OP_NAME = ["add", "sub", "mul", "div", "min", "max", "abs", "neg", "sqrt", "sin", "hash", "gt", "glue", "half"]
OP_ARITY = [2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 1]
REG_NAME = ["d", "d_prev", "eps", "overlap", "approach", "curv", "1", "t*.1", "i*.02", "radius",
            "c0", "c1", "c2", "c3"]


def _hash(va, vb):
    s = jnp.sin(va * 12.9898 + vb * 78.233) * 43758.5453
    return jnp.abs(s - jnp.trunc(s))  # truncation-toward-zero frac (matches torch.frac; NOT jnp.floor)


def run_program(regs, OP, A, B, DST, L):
    # regs: (P, N, R); OP/A/B/DST: (P, L) int32. Unrolled over the static L instructions.
    P, N, _ = regs.shape

    def gather(idx):  # idx: (P,) register indices -> (P,N) values
        ix = jnp.broadcast_to(idx[:, None, None], (P, N, 1))
        return jnp.take_along_axis(regs, ix, axis=2)[:, :, 0]

    for l in range(L):
        va = gather(A[:, l])
        vb = gather(B[:, l])
        denom = jnp.where(jnp.abs(vb) < 1e-6, 1e-6, vb)
        cand = jnp.stack([
            va + vb, va - vb, va * vb, va / denom,
            jnp.minimum(va, vb), jnp.maximum(va, vb),
            jnp.abs(va), -va, jnp.sqrt(jnp.abs(va)), jnp.sin(va),
            _hash(va, vb), jnp.where(va > vb, 1.0, 0.0),
            jnp.where(va > 0.0, va, vb), va * 0.5,
        ], axis=-1)  # (P,N,K)
        opix = jnp.broadcast_to(OP[:, l][:, None, None], (P, N, 1))
        res = jnp.take_along_axis(cand, opix, axis=2)[:, :, 0]  # (P,N)
        oh = jax.nn.one_hot(DST[:, l], R, dtype=regs.dtype)     # (P,R)
        regs = regs * (1.0 - oh[:, None, :]) + res[:, :, None] * oh[:, None, :]
    return regs


# =====================================================================================
# The discovery marcher: at each step, seed registers, run the evolved program, take its step.
# Currency = field-evals (SDF calls) at held silhouette/depth quality. No guard: the gate is the guard.
# =====================================================================================
def march(genome, eye, dir, max_steps, L):
    OP, A, B, DST, CN = genome
    P = OP.shape[0]
    N = dir.shape[0]
    eyeb = eye[None]
    dirb = dir[None]
    INF = jnp.float32(jnp.inf)
    z = jnp.zeros((P, N), jnp.float32)
    ones = jnp.ones((P, N), jnp.float32)
    c = [CN[:, k][:, None] * ones for k in range(C)]  # each (P,N)

    t = z
    active = jnp.ones((P, N), bool)
    prev_radius = z
    last_step = z
    d_prev = jnp.full((P, N), INF)
    d_prev2 = jnp.full((P, N), INF)
    hit = jnp.zeros((P, N), bool)
    t_hit = z
    evals = jnp.zeros((P,), jnp.int32)

    def body(i, carry):
        (t, active, prev_radius, last_step, d_prev, d_prev2, hit, t_hit, evals) = carry
        pos = eyeb + t[..., None] * dirb
        d = sdf(pos)
        evals = evals + jnp.sum(active.astype(jnp.int32), axis=1)
        radius = jnp.abs(d)
        eps = 0.0006 * (1.0 + 0.5 * t)
        newhit = active & (d < eps)
        # finite-guarded signals (inf sentinels are discarded by jnp.where in forward mode)
        dpg = jnp.where(jnp.isfinite(d_prev), d_prev, d)
        approach = dpg - d
        fin2 = jnp.isfinite(d_prev) & jnp.isfinite(d_prev2)
        curv = jnp.where(fin2, d - 2.0 * d_prev + d_prev2, 0.0)
        overlap = radius + prev_radius - last_step
        inorm = jnp.float32(i) * 0.02
        regs = jnp.stack([d, dpg, eps, overlap, approach, curv, ones, t * 0.1, inorm * ones, radius,
                          c[0], c[1], c[2], c[3]], axis=-1)  # (P,N,R)
        regs = run_program(regs, OP, A, B, DST, L)
        proposed = regs[:, :, 0]  # output register
        step = jnp.where(jnp.isfinite(proposed), proposed, d)
        step = jnp.clip(step, 0.0, 20.0)  # numerical safety only; quality is enforced by the gate
        just = newhit & active
        t_hit = jnp.where(just, t, t_hit)
        hit = hit | just
        active = active & (~just)
        prev_radius = radius
        d_prev2 = d_prev
        d_prev = d
        last_step = step
        t = jnp.where(active, t + step, t)
        active = active & (t <= 30.0)
        return (t, active, prev_radius, last_step, d_prev, d_prev2, hit, t_hit, evals)

    carry = lax.fori_loop(0, max_steps, body,
                          (t, active, prev_radius, last_step, d_prev, d_prev2, hit, t_hit, evals))
    (t, active, prev_radius, last_step, d_prev, d_prev2, hit, t_hit, evals) = carry
    return hit, t_hit, evals


def march_plain(eye, dir, max_steps):
    # naive sphere trace (step = d), the honest baseline the discovery must beat.
    N = dir.shape[0]
    eyeb = eye[None]
    dirb = dir[None]
    t = jnp.zeros((1, N), jnp.float32)
    active = jnp.ones((1, N), bool)
    hit = jnp.zeros((1, N), bool)
    t_hit = jnp.zeros((1, N), jnp.float32)
    evals = jnp.zeros((1,), jnp.int32)

    def body(i, carry):
        t, active, hit, t_hit, evals = carry
        d = sdf(eyeb + t[..., None] * dirb)
        evals = evals + jnp.sum(active.astype(jnp.int32), axis=1)
        eps = 0.0006 * (1.0 + 0.5 * t)
        just = active & (d < eps)
        t_hit = jnp.where(just, t, t_hit)
        hit = hit | just
        active = active & (~just)
        t = jnp.where(active, t + d, t)
        active = active & (t <= 30.0)
        return t, active, hit, t_hit, evals

    return lax.fori_loop(0, max_steps, body, (t, active, hit, t_hit, evals))


# =====================================================================================
# Host-side decoder: turn the winning integer genome into a readable step formula.
# =====================================================================================
def decode(op, a, b, dst, cn, L):
    reg = list(REG_NAME[:10]) + [f"{cn[k]:+.3f}" for k in range(C)]
    for l in range(L):
        o, ai, bi, di = int(op[l]), int(a[l]), int(b[l]), int(dst[l])
        va, vb = reg[ai], reg[bi]
        nm = OP_NAME[o]
        if OP_ARITY[o] == 1:
            e = f"({va})*.5" if nm == "half" else (f"-({va})" if nm == "neg" else f"{nm}({va})")
        elif nm == "add":
            e = f"({va}+{vb})"
        elif nm == "sub":
            e = f"({va}-{vb})"
        elif nm == "mul":
            e = f"({va}*{vb})"
        elif nm == "div":
            e = f"({va}/{vb})"
        elif nm == "gt":
            e = f"({va}>{vb})"
        elif nm == "glue":
            e = f"({va}>0?{va}:{vb})"
        else:
            e = f"{nm}({va},{vb})"
        reg[di] = e
    return reg[0]


if __name__ == "__main__":
    args = parse_args()
    L = args["L"]
    devs = jax.devices()
    ncore = len(devs)
    plat = devs[0].platform
    w, h = args["res"]
    ncam = args["ncam"]
    STEPS = args["steps"]
    REF_STEPS = STEPS * 2

    eye_np, dir_np = make_rays(ncam, w, h)
    eye = jnp.asarray(eye_np)
    dir = jnp.asarray(dir_np)
    N = dir.shape[0]
    P = max(ncore, (args["pop"] // ncore) * ncore)
    ELITE = max(8, P // 16)
    TOL = 0.012

    # baseline + ground-truth reference from the naive sphere trace.
    # march_plain returns the fori_loop carry: (t, active, hit, t_hit, evals).
    _, _, _, _, base_ev = jax.jit(lambda: march_plain(eye, dir, STEPS))()
    _, _, ref_hit, ref_t, _ = jax.jit(lambda: march_plain(eye, dir, REF_STEPS))()
    ref_hit = ref_hit[0]
    ref_t = ref_t[0]
    base = int(np.asarray(base_ev)[0])

    def evaluate(genome, eye, dir, ref_hit, ref_t):
        hit, t_hit, evals = march(genome, eye, dir, STEPS, L)
        sil = jnp.mean((hit != ref_hit[None]).astype(jnp.float32), axis=1)
        both = hit & ref_hit[None]
        depth = (jnp.sum(jnp.where(both, jnp.abs(t_hit - ref_t[None]) / (ref_t[None] + 1e-3), 0.0), axis=1)
                 / jnp.maximum(jnp.sum(both.astype(jnp.float32), axis=1), 1.0))
        redux = (evals.astype(jnp.float32) - base) / base * 100.0
        return redux, sil + 0.3 * depth

    peval = jax.pmap(evaluate, in_axes=(0, None, None, None, None))

    def eval_pop(g):
        chunk = tuple(jnp.asarray(x.reshape((ncore, P // ncore) + x.shape[1:])) for x in g)
        redux, err = peval(chunk, eye, dir, ref_hit, ref_t)
        return np.asarray(redux).reshape(P), np.asarray(err).reshape(P)

    rng = np.random.default_rng(20260701)
    OP = rng.integers(0, K, (P, L)).astype(np.int32)
    A = rng.integers(0, R, (P, L)).astype(np.int32)
    B = rng.integers(0, R, (P, L)).astype(np.int32)
    DST = rng.integers(0, R, (P, L)).astype(np.int32)
    CN = (rng.standard_normal((P, C)) * 1.5).astype(np.float32)

    def mut_int(arr, hi, p, parents):
        child = arr[parents].copy()
        mask = rng.random(child.shape) < p
        child[mask] = rng.integers(0, hi, int(mask.sum()))
        return child.astype(np.int32)

    hdr = (f"=== DISCOVER-TPU START | {ncore}x {plat.upper()} | {N} rays ({ncam} cams {w}x{h}) | "
           f"pop={P} L={L} R={R} ops={K} | steps={STEPS} | budget {args['minutes']:.0f} min | "
           f"naive-baseline {base} field-evals ===")
    print(hdr, flush=True)
    post(hdr)
    post("discover-tpu: linear-GP program synthesis of the marcher STEP function over root atoms "
         "(add/sub/mul/div/min/max/abs/neg/sqrt/sin/HASH/gt/GLUE/half). Nothing pre-loaded; naive "
         "sphere-trace is a no-op program; safety = the currency gate. Discovers, does not confirm.")

    best = None  # (redux, err, genome-tuple-of-1)
    start = time.time()
    budget = args["minutes"] * 60.0
    gen = 0
    last = 0.0
    while time.time() - start < budget:
        redux, err = eval_pop((OP, A, B, DST, CN))
        ok = err <= TOL
        score = np.where(ok, redux, 1e6)
        order = np.argsort(score)
        bi = int(order[0])
        if ok[bi] and (best is None or redux[bi] < best[0]):
            g1 = (OP[bi].copy(), A[bi].copy(), B[bi].copy(), DST[bi].copy(), CN[bi].copy())
            best = (float(redux[bi]), float(err[bi]), g1)
            formula = decode(*g1, L)
            line = f"[{time.time()-start:6.0f}s g{gen}] DISCOVERED {redux[bi]:+.1f}% (err {err[bi]:.4f})  step = clip({formula}, 0, 20)"
            print(line, flush=True)
            post(line)
        elite = order[:ELITE]
        parents = elite[rng.integers(0, ELITE, P)]
        OP = mut_int(OP, K, 0.18, parents)
        A = mut_int(A, R, 0.14, parents)
        B = mut_int(B, R, 0.14, parents)
        DST = mut_int(DST, R, 0.14, parents)
        CN = (CN[parents] + 0.15 * rng.standard_normal((P, C))).astype(np.float32)
        # fresh random immigrants keep the search from collapsing onto one lineage (discovery > exploit)
        imm = rng.random(P) < 0.06
        if imm.any():
            m = int(imm.sum())
            OP[imm] = rng.integers(0, K, (m, L)); A[imm] = rng.integers(0, R, (m, L))
            B[imm] = rng.integers(0, R, (m, L)); DST[imm] = rng.integers(0, R, (m, L))
            CN[imm] = (rng.standard_normal((m, C)) * 1.5).astype(np.float32)
        gen += 1
        el = time.time() - start
        if el - last >= 120.0:
            last = el
            cps = P * gen / el
            b = best[0] if best else 0.0
            hb = f"[{el:6.0f}s] gen={gen} ({cps:.0f} prog/s) | best {b:+.1f}%"
            print(hb, flush=True)
            post(hb)

    print(f"\n=== DONE after {time.time()-start:.0f}s, {gen} gens, {P*gen} programs evaluated ===", flush=True)
    if best:
        formula = decode(*best[2], L)
        summary = (f"BEST DISCOVERED PROGRAM: {best[0]:+.1f}% field-evals vs naive (err {best[1]:.4f})\n"
                   f"  step = clip( {formula} , 0, 20 )\n"
                   f"  (naive sphere-trace = the identity 'd'; over-relax ~= 'd*1.4'; subitize ~= "
                   f"'d*(1 + c*(d>k*eps))'. Compare the discovered form to these.)")
        print(summary, flush=True)
        post(summary)
        post("=== KERNEL DONE ===")
    else:
        print("no program held image quality.", flush=True)
        post("discover-tpu: no program held image quality.")
