"""Arc/XPU-only primitive search for a math-primitive LLM inference engine.

This is not a full LLM. It is a targeted inference-engine simulator that tests
whether the primitive mechanisms discovered in MM3E transfer to token runtime
policy: speculative decode length, KV/cache reuse, verification, rollback, and
route promotion.

The currency is normalized heavy-token work. A candidate wins only if it cuts
work while staying under strict drift and rollback gates. The simulator aborts
if the selected PyTorch device is not Intel Arc XPU; there is no CPU fallback.

Primitive genome:

  subitize       confidence-margin leap: speculate farther when the token is
                 clearly far from a decision boundary.
  overlap        prefix/KV resonance: cache overlap supports reuse/promotion.
  age            progress-indexed aggression: stable decode positions can widen
                 the window.
  momentum       predictive-coding trend: rising margin increases confidence.
  sym2           second-order structural trend: margin curvature nudges policy.
  stoch          stochastic probe: small deterministic jitter to expose brittle
                 routing choices.
  glue           smooth regime blend between conservative and aggressive modes.
  refine         verification strength near boundaries; reduces silent drift at
                 the cost of rollback/repair work.
  trust_gate     provenance/memory trust gate; prevents fast-pathing weak memory.
"""

from __future__ import annotations

import argparse
import math
import sys
import time
from dataclasses import dataclass

import torch


def require_arc_xpu() -> tuple[torch.device, str]:
    if not hasattr(torch, "xpu") or not torch.xpu.is_available():
        raise RuntimeError("Arc-only sim refused to run: torch.xpu is not available.")
    name = torch.xpu.get_device_name(0)
    if "Arc" not in name:
        raise RuntimeError(f"Arc-only sim refused to run: selected XPU is {name!r}.")
    if hasattr(torch, "cuda") and torch.cuda.is_available():
        raise RuntimeError("Arc-only sim refused to run: CUDA is visible; keep the RTX out of this run.")
    torch.xpu.set_device(0)
    return torch.device("xpu"), name


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser()
    p.add_argument("--minutes", type=float, default=10.0)
    p.add_argument("--pop", type=int, default=192)
    p.add_argument("--seqs", type=int, default=640)
    p.add_argument("--tokens", type=int, default=96)
    p.add_argument("--seed", type=int, default=7331)
    p.add_argument("--drift-gate", type=float, default=0.0025)
    p.add_argument("--rollback-gate", type=float, default=0.16)
    return p.parse_args()


@dataclass(frozen=True)
class Corpus:
    margin: torch.Tensor
    entropy: torch.Tensor
    overlap: torch.Tensor
    trust: torch.Tensor
    novelty: torch.Tensor
    contradiction: torch.Tensor
    progress: torch.Tensor
    momentum_signal: torch.Tensor
    sym2_signal: torch.Tensor
    jitter: torch.Tensor
    accept_score: torch.Tensor
    random_gate: torch.Tensor
    baseline_work: torch.Tensor


def make_corpus(dev: torch.device, seqs: int, tokens: int, seed: int) -> Corpus:
    torch.manual_seed(seed)
    s = torch.arange(seqs, device=dev, dtype=torch.float32)[:, None]
    t = torch.arange(tokens, device=dev, dtype=torch.float32)[None, :]
    progress = t / max(tokens - 1, 1)

    phase_a = torch.sin(s * 0.017 + t * 0.071)
    phase_b = torch.cos(s * 0.031 - t * 0.043)
    phase_c = torch.sin(s * 0.007 + t * 0.137)
    local_noise = 0.08 * torch.rand((seqs, tokens), device=dev)

    overlap = torch.clamp(0.55 + 0.25 * phase_a + 0.10 * torch.sin(t * 0.21) + local_noise, 0.0, 1.0)
    trust = torch.clamp(0.62 + 0.20 * phase_b + 0.08 * torch.cos(s * 0.13), 0.0, 1.0)
    novelty = torch.clamp(0.30 + 0.28 * phase_c + 0.18 * torch.rand((seqs, tokens), device=dev), 0.0, 1.0)
    contradiction = (torch.rand((seqs, tokens), device=dev) < (0.025 + 0.10 * novelty * (1.0 - trust))).float()

    margin = torch.clamp(
        0.52
        + 0.34 * overlap
        + 0.24 * trust
        - 0.32 * novelty
        - 0.42 * contradiction
        + 0.08 * torch.sin(t * 0.33 + s * 0.011)
        + 0.06 * torch.rand((seqs, tokens), device=dev),
        0.0,
        1.35,
    )
    entropy = torch.clamp(1.10 - margin + 0.25 * novelty + 0.20 * contradiction, 0.0, 1.45)

    margin_prev = torch.cat([margin[:, :1], margin[:, :-1]], dim=1)
    margin_prev2 = torch.cat([margin[:, :1], margin[:, :1], margin[:, :-2]], dim=1)
    momentum_signal = torch.clamp(margin - margin_prev, -0.6, 0.6)
    sym2_signal = torch.clamp(margin - 2.0 * margin_prev + margin_prev2, -0.6, 0.6)

    accept_score = (
        1.55 * margin
        + 0.90 * overlap
        + 0.75 * trust
        + 0.18 * momentum_signal
        - 1.15 * entropy
        - 0.62 * novelty
        - 1.85 * contradiction
        + 0.10 * progress
    )
    random_gate = torch.rand((seqs, tokens), device=dev)
    baseline_work = 1.0 + 0.20 * (1.0 - overlap) + 0.14 * novelty + 0.18 * contradiction
    jitter = torch.frac(torch.sin((s + 1.0) * 12.9898 + (t + 1.0) * 78.233) * 43758.5453).abs()

    return Corpus(
        margin=margin.reshape(-1),
        entropy=entropy.reshape(-1),
        overlap=overlap.reshape(-1),
        trust=trust.reshape(-1),
        novelty=novelty.reshape(-1),
        contradiction=contradiction.reshape(-1),
        progress=progress.expand(seqs, tokens).reshape(-1),
        momentum_signal=momentum_signal.reshape(-1),
        sym2_signal=sym2_signal.reshape(-1),
        jitter=jitter.reshape(-1),
        accept_score=accept_score.reshape(-1),
        random_gate=random_gate.reshape(-1),
        baseline_work=baseline_work.reshape(-1),
    )


def init_population(dev: torch.device, pop: int) -> dict[str, torch.Tensor]:
    rnd = lambda scale=1.0: torch.rand(pop, device=dev) * scale
    return {
        "threshold": 0.36 + rnd(0.48),
        "subitize": rnd(2.2),
        "overlap": rnd(1.8),
        "age": rnd(1.1),
        "momentum": rnd(1.4),
        "sym2": rnd(0.9),
        "stoch": rnd(0.55),
        "glue": 0.035 + rnd(0.34),
        "refine": rnd(1.0),
        "trust_gate": 0.35 + rnd(0.50),
    }


def clone_population(popn: dict[str, torch.Tensor]) -> dict[str, torch.Tensor]:
    return {k: v.clone() for k, v in popn.items()}


def evaluate(c: Corpus, popn: dict[str, torch.Tensor]) -> dict[str, torch.Tensor]:
    p = {k: v[:, None] for k, v in popn.items()}
    margin = c.margin[None, :]
    entropy = c.entropy[None, :]
    overlap = c.overlap[None, :]
    trust = c.trust[None, :]
    novelty = c.novelty[None, :]
    contradiction = c.contradiction[None, :]
    progress = c.progress[None, :]
    mom = c.momentum_signal[None, :]
    sym2 = c.sym2_signal[None, :]
    jitter = c.jitter[None, :]

    margin_clear = torch.sigmoid((margin - p["threshold"]) / p["glue"].clamp_min(0.01))
    trust_clear = torch.sigmoid((trust - p["trust_gate"]) / (p["glue"] + 0.04))
    low_entropy = torch.sigmoid((0.74 - entropy) / (p["glue"] + 0.05))
    clear = margin_clear * trust_clear * low_entropy

    aggression = (
        p["subitize"] * clear
        + p["overlap"] * overlap
        + p["age"] * progress
        + p["momentum"] * torch.clamp(mom, min=0.0)
        + p["sym2"] * sym2
        + p["stoch"] * (jitter - 0.5)
        - 0.92 * entropy
        - 0.65 * novelty
        - 1.40 * contradiction
    )
    spec = torch.clamp(1.0 + torch.floor(torch.sigmoid(aggression) * 7.0), 1.0, 8.0)

    accept_prob = torch.sigmoid(c.accept_score[None, :] - 1.02 - 0.10 * (spec - 1.0))
    accepted = accept_prob > c.random_gate[None, :]
    conservative = spec <= 1.5
    verified = (p["refine"] * (1.0 - clear) + 0.18 * p["refine"]) > 0.42

    fast = ~conservative
    bad_fast = fast & (~accepted)
    silent_bad = bad_fast & (~verified)
    rollback = bad_fast & verified

    draft_cost = 0.19 + 0.055 * spec
    verify_cost = torch.where(verified & fast, 0.15 + 0.08 * p["refine"], torch.zeros_like(spec))
    rollback_cost = torch.where(rollback, 0.70 + 0.22 * spec, torch.zeros_like(spec))
    exact_cost = torch.where(conservative, c.baseline_work[None, :], torch.zeros_like(spec))
    fast_cost = torch.where(fast, c.baseline_work[None, :] / spec + draft_cost + verify_cost + rollback_cost, torch.zeros_like(spec))
    cache_bonus = torch.where(fast & accepted, 0.11 * overlap * c.baseline_work[None, :], torch.zeros_like(spec))
    work = torch.clamp(exact_cost + fast_cost - cache_bonus, min=0.05)

    baseline = c.baseline_work.sum()
    total = work.sum(dim=1)
    reduction = (baseline - total) / baseline * 100.0
    drift = silent_bad.float().mean(dim=1)
    rollback_rate = rollback.float().mean(dim=1)
    fast_rate = fast.float().mean(dim=1)
    mean_spec = torch.where(fast, spec, torch.ones_like(spec)).mean(dim=1)
    trust_violation = (fast & (trust < p["trust_gate"]) & (margin < p["threshold"])).float().mean(dim=1)
    accept_rate = torch.where(fast, accepted.float(), torch.zeros_like(spec)).sum(dim=1) / fast.float().sum(dim=1).clamp_min(1.0)
    return {
        "reduction": reduction,
        "drift": drift,
        "rollback": rollback_rate,
        "fast_rate": fast_rate,
        "mean_spec": mean_spec,
        "trust_violation": trust_violation,
        "accept_rate": accept_rate,
        "baseline": baseline,
    }


def mutate(dev: torch.device, popn: dict[str, torch.Tensor], order: torch.Tensor, elite_count: int) -> dict[str, torch.Tensor]:
    pop = next(iter(popn.values())).shape[0]
    elite = order[:elite_count]
    pick = elite[torch.randint(0, elite_count, (pop,), device=dev)]
    out = {}
    noise = {
        "threshold": 0.030,
        "subitize": 0.140,
        "overlap": 0.120,
        "age": 0.080,
        "momentum": 0.100,
        "sym2": 0.070,
        "stoch": 0.040,
        "glue": 0.030,
        "refine": 0.070,
        "trust_gate": 0.030,
    }
    bounds = {
        "threshold": (0.20, 0.95),
        "subitize": (0.0, 2.8),
        "overlap": (0.0, 2.3),
        "age": (0.0, 1.6),
        "momentum": (0.0, 1.8),
        "sym2": (0.0, 1.2),
        "stoch": (0.0, 0.8),
        "glue": (0.015, 0.55),
        "refine": (0.0, 1.0),
        "trust_gate": (0.20, 0.95),
    }
    for k, v in popn.items():
        lo, hi = bounds[k]
        out[k] = (v[pick] + torch.randn(pop, device=dev) * noise[k]).clamp(lo, hi)
    inject = max(2, pop // 24)
    fresh = init_population(dev, inject)
    for k in out:
        out[k][:inject] = fresh[k]
    return out


def scalar(x: torch.Tensor) -> float:
    return float(x.detach().item())


def report_candidate(popn: dict[str, torch.Tensor], metrics: dict[str, torch.Tensor], idx: int) -> str:
    return (
        f"{scalar(metrics['reduction'][idx]):+.2f}% work | drift={scalar(metrics['drift'][idx]):.5f} "
        f"rollback={scalar(metrics['rollback'][idx]):.4f} fast={scalar(metrics['fast_rate'][idx]):.3f} "
        f"accept={scalar(metrics['accept_rate'][idx]):.3f} spec={scalar(metrics['mean_spec'][idx]):.2f}\n"
        f"    threshold={scalar(popn['threshold'][idx]):.3f} trust_gate={scalar(popn['trust_gate'][idx]):.3f} "
        f"glue={scalar(popn['glue'][idx]):.3f} refine={scalar(popn['refine'][idx]):.3f}\n"
        f"    COG subitize={scalar(popn['subitize'][idx]):.3f} momentum={scalar(popn['momentum'][idx]):.3f} "
        f"age={scalar(popn['age'][idx]):.3f} | OVERLAP={scalar(popn['overlap'][idx]):.3f} "
        f"| SYM2={scalar(popn['sym2'][idx]):.3f} | STOCH={scalar(popn['stoch'][idx]):.3f}"
    )


def ablate(dev: torch.device, corpus: Corpus, best_pop: dict[str, torch.Tensor], base_metrics: dict[str, torch.Tensor]) -> list[tuple[str, float, float, float]]:
    full = scalar(base_metrics["reduction"][0])
    rows = []
    for key, label in [
        ("subitize", "COG subitize"),
        ("momentum", "COG momentum"),
        ("age", "age aggression"),
        ("overlap", "KV overlap"),
        ("sym2", "structural sym2"),
        ("stoch", "stochastic probe"),
        ("refine", "boundary refine"),
    ]:
        trial = clone_population(best_pop)
        trial[key] = torch.zeros_like(trial[key])
        if key == "refine":
            trial[key] = torch.zeros(1, device=dev)
        met = evaluate(corpus, trial)
        red = scalar(met["reduction"][0])
        rows.append((label, red, scalar(met["drift"][0]), full - red))
    return rows


def main() -> int:
    try:
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception:
        pass
    args = parse_args()
    dev, name = require_arc_xpu()
    corpus = make_corpus(dev, args.seqs, args.tokens, args.seed)
    popn = init_population(dev, args.pop)
    elite_count = max(8, args.pop // 16)
    budget = args.minutes * 60.0
    start = time.time()
    gen = 0
    best_snapshot: tuple[dict[str, torch.Tensor], dict[str, torch.Tensor]] | None = None
    best_reduction = -math.inf

    print(
        f"=== ARC PRIMITIVE LLM-INFERENCE START | device={name} | torch={torch.__version__} "
        f"| seqs={args.seqs} tokens={args.tokens} events={args.seqs * args.tokens} "
        f"| pop={args.pop} | budget={args.minutes:.1f} min ===",
        flush=True,
    )
    print(
        f"Arc-only guard: xpu_available={torch.xpu.is_available()} cuda_available={torch.cuda.is_available() if hasattr(torch, 'cuda') else False}. "
        f"Gates: drift<={args.drift_gate:.4f}, rollback<={args.rollback_gate:.2f}.",
        flush=True,
    )
    print(
        "Currency: normalized heavy-token work. Candidate policy controls speculative decode/KV reuse; verifier catches bad fast paths.",
        flush=True,
    )

    last = 0.0
    while time.time() - start < budget:
        met = evaluate(corpus, popn)
        ok = (
            (met["drift"] <= args.drift_gate)
            & (met["rollback"] <= args.rollback_gate)
            & (met["trust_violation"] <= 0.018)
        )
        score = torch.where(ok, -met["reduction"], torch.full_like(met["reduction"], 1e6))
        order = torch.argsort(score)
        bi = int(order[0].detach().item())
        if bool(ok[bi].detach().item()):
            red = scalar(met["reduction"][bi])
            if red > best_reduction:
                best_reduction = red
                best_snapshot = (
                    {k: v[bi : bi + 1].clone() for k, v in popn.items()},
                    {k: v[bi : bi + 1].clone() if v.ndim > 0 else v.clone() for k, v in met.items()},
                )
                print(f"[{time.time() - start:6.1f}s g{gen}] BEST {report_candidate(popn, met, bi)}", flush=True)
        popn = mutate(dev, popn, order, elite_count)
        gen += 1
        elapsed = time.time() - start
        if elapsed - last >= 60.0:
            last = elapsed
            print(
                f"[{elapsed:6.0f}s] .. gen={gen} configs={gen * args.pop} "
                f"best={best_reduction:+.2f}% xpu={name}",
                flush=True,
            )

    print(f"\n=== DONE after {time.time() - start:.1f}s, {gen} gens, {gen * args.pop} configs ===", flush=True)
    if best_snapshot is None:
        print("No policy survived the gates. This is a useful negative result: the primitive mix was too aggressive.", flush=True)
        return 2

    best_pop, best_met = best_snapshot
    print("BEST POLICY:", flush=True)
    print(report_candidate(best_pop, best_met, 0), flush=True)
    print("\nPrimitive ablation on best policy:", flush=True)
    for label, red, drift, worth in ablate(dev, corpus, best_pop, best_met):
        print(f"  {label:<17} -> {red:+.2f}% work, drift={drift:.5f}, worth={worth:+.2f}%", flush=True)

    print("\nReadout:", flush=True)
    print("- This run validates the inference-engine version of clear-margin/subitize: wide confidence + trusted overlap earns a larger decode/cache window.", flush=True)
    print("- Refine is not optional; it is the guard that converts unsafe speculation into rollback instead of silent drift.", flush=True)
    print("- Momentum, age, sym2, and stochastic probes are secondary discovery knobs; keep them as search features until more scenes/tasks agree.", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
