r"""Arc/XPU RF self-repair + crypto integrity simulation.

This is a defensive transport experiment for the spiderweb RF lane. It models a
bursty Gilbert-Elliott radio link with random tamper/corruption, then compares
repair/integrity policies:

- raw delivery
- per-frame MAC/HMAC as "corruption becomes erasure"
- repetition + dedup
- ideal FEC/RLNC-style repair
- block-hash-only integrity
- adaptive extra repair after a bad first pass

The key question is not encryption. It is whether crypto should sit before the
repair decoder. If corrupted shards are authenticated and dropped, FEC can heal
them as erasures. If corrupted shards enter the decoder, they poison the block.

Run:
  C:\Projects\discovery-search-12h-gpu\.venv-arc\Scripts\python.exe experiments\rf-crypto-arc\rf_crypto_self_repair.py
"""

from __future__ import annotations

import argparse
import time

import torch


def pick_device():
    if hasattr(torch, "xpu") and torch.xpu.is_available():
        return torch.device("xpu"), torch.xpu.get_device_name(0)
    raise SystemExit("Arc/XPU torch runtime not available; refusing CPU/CUDA fallback for this sim.")


def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--trials", type=int, default=32768)
    p.add_argument("--k", type=int, default=48)
    p.add_argument("--m", type=int, default=28)
    p.add_argument("--extra", type=int, default=24)
    p.add_argument("--seed", type=int, default=0xC0FFEE)
    return p.parse_args()


def simulate_channel(
    trials: int,
    slots: int,
    *,
    dev,
    seed: int,
    gap: int = 1,
    good_loss: float = 0.02,
    bad_loss: float = 0.60,
    fade_enter: float = 0.06,
    fade_exit: float = 0.25,
    good_tamper: float = 0.002,
    bad_tamper: float = 0.035,
):
    """Return (survived, tampered) bool tensors of shape [trials, slots]."""
    torch.manual_seed(seed)
    faded = torch.zeros(trials, dtype=torch.bool, device=dev)
    survived = torch.empty((trials, slots), dtype=torch.bool, device=dev)
    tampered = torch.empty((trials, slots), dtype=torch.bool, device=dev)

    for s in range(slots):
        for _ in range(gap):
            leave = torch.rand(trials, device=dev) < fade_exit
            enter = torch.rand(trials, device=dev) < fade_enter
            faded = torch.where(faded, ~leave, enter)

        loss_p = torch.where(
            faded,
            torch.full((trials,), bad_loss, device=dev),
            torch.full((trials,), good_loss, device=dev),
        )
        tamp_p = torch.where(
            faded,
            torch.full((trials,), bad_tamper, device=dev),
            torch.full((trials,), good_tamper, device=dev),
        )
        ok = torch.rand(trials, device=dev) >= loss_p
        bad = ok & (torch.rand(trials, device=dev) < tamp_p)
        survived[:, s] = ok
        tampered[:, s] = bad

    return survived, tampered


def pct(x: torch.Tensor) -> float:
    return float(x.float().mean().detach().cpu().item() * 100.0)


def avg(x: torch.Tensor) -> float:
    return float(x.float().mean().detach().cpu().item())


def summarize(name: str, overhead: float, clean_block: torch.Tensor, delivered_frac: torch.Tensor, silent_corrupt=None):
    silent = 0.0 if silent_corrupt is None else pct(silent_corrupt)
    return {
        "name": name,
        "overhead": overhead,
        "clean_block": pct(clean_block),
        "avg_delivered": avg(delivered_frac) * 100.0,
        "silent_corrupt": silent,
    }


def main():
    args = parse_args()
    dev, name = pick_device()
    k, m, extra, trials = args.k, args.m, args.extra, args.trials
    t0 = time.time()

    print(
        f"=== RF+CRYPTO SELF-REPAIR ARC SIM | device={name} | trials={trials} | "
        f"k={k} m={m} extra={extra} ===",
        flush=True,
    )
    print(
        "Channel = bursty fading loss plus tamper. Clean block = all k chunks recover authentically. "
        "Silent corrupt = corrupted payload would be accepted undetected.\n",
        flush=True,
    )

    results = []

    # Raw link, no integrity: any tampered delivered chunk is a silent corruption.
    surv, tamp = simulate_channel(trials, k, dev=dev, seed=args.seed)
    raw_delivered = surv.sum(dim=1)
    raw_tampered = (surv & tamp).any(dim=1)
    results.append(
        summarize(
            "raw/no crypto",
            1.00,
            (raw_delivered == k) & ~raw_tampered,
            raw_delivered / k,
            silent_corrupt=raw_tampered,
        )
    )

    # Per-frame MAC turns tamper into erasure.
    auth = surv & ~tamp
    auth_delivered = auth.sum(dim=1)
    results.append(summarize("raw + per-frame MAC", 1.00, auth_delivered == k, auth_delivered / k))

    # Repetition. Adjacent copies are vulnerable to a fade; interleaved copies decorrelate them.
    surv, tamp = simulate_channel(trials, k * 2, dev=dev, seed=args.seed + 1)
    auth = surv & ~tamp
    adj = auth.reshape(trials, k, 2).any(dim=2)
    results.append(summarize("rep2 adjacent + MAC", 2.00, adj.all(dim=1), adj.sum(dim=1) / k))

    inter = auth[:, :k] | auth[:, k:]
    results.append(summarize("rep2 interleaved + MAC", 2.00, inter.all(dim=1), inter.sum(dim=1) / k))

    # Idealized RLNC/MDS FEC: k clean equations recover the block. This models the target behavior
    # of the fountain/RLNC family without CPU-side GF(2) elimination in the Monte Carlo loop.
    slots = k + m
    surv, tamp = simulate_channel(trials, slots, dev=dev, seed=args.seed + 2)
    auth = surv & ~tamp
    auth_eqs = auth.sum(dim=1)
    data_auth = auth[:, :k].sum(dim=1)
    fec_clean = auth_eqs >= k
    fec_delivered = torch.where(fec_clean, torch.ones_like(data_auth), data_auth / k)
    results.append(summarize(f"FEC/RLNC m={m} + per-shard MAC", slots / k, fec_clean, fec_delivered))

    # Block hash catches corruption after decode, but corrupted equations still poison repair.
    recv = surv.sum(dim=1)
    recv_tampered = (surv & tamp).any(dim=1)
    block_hash_clean = (recv >= k) & ~recv_tampered
    block_hash_detected_fail = (recv >= k) & recv_tampered
    results.append(
        summarize(
            f"FEC/RLNC m={m} + block hash only",
            slots / k,
            block_hash_clean,
            torch.where(block_hash_clean, torch.ones_like(recv), surv[:, :k].sum(dim=1) / k),
            silent_corrupt=torch.zeros(trials, dtype=torch.bool, device=dev),
        )
    )

    # No crypto at all: FEC may report success while accepting poisoned equations.
    no_crypto_report = recv >= k
    no_crypto_silent = no_crypto_report & recv_tampered
    results.append(
        summarize(
            f"FEC/RLNC m={m} no crypto",
            slots / k,
            no_crypto_report & ~recv_tampered,
            torch.where(no_crypto_report, torch.ones_like(recv), surv[:, :k].sum(dim=1) / k),
            silent_corrupt=no_crypto_silent,
        )
    )

    # Same FEC budget but time-interleaved: less correlated fade damage, more latency.
    surv, tamp = simulate_channel(trials, slots, dev=dev, seed=args.seed + 3, gap=4)
    auth = surv & ~tamp
    auth_eqs = auth.sum(dim=1)
    data_auth = auth[:, :k].sum(dim=1)
    fec_clean = auth_eqs >= k
    results.append(
        summarize(
            f"FEC/RLNC m={m} + MAC + interleave",
            slots / k,
            fec_clean,
            torch.where(fec_clean, torch.ones_like(data_auth), data_auth / k),
        )
    )

    # Adaptive self-repair: send k+m first. If clean equations are still below k+4, send extra
    # repair shards on the continuing channel. This costs less than always sending m+extra.
    base_slots = k + 16
    surv1, tamp1 = simulate_channel(trials, base_slots, dev=dev, seed=args.seed + 4)
    auth1 = surv1 & ~tamp1
    eq1 = auth1.sum(dim=1)
    needs_extra = eq1 < (k + 4)
    surv2, tamp2 = simulate_channel(trials, extra, dev=dev, seed=args.seed + 5)
    auth2 = surv2 & ~tamp2
    eq_total = eq1 + torch.where(needs_extra, auth2.sum(dim=1), torch.zeros(trials, device=dev, dtype=torch.int64))
    data_auth = auth1[:, :k].sum(dim=1)
    adaptive_clean = eq_total >= k
    adaptive_overhead = (base_slots + avg(needs_extra) * extra) / k
    results.append(
        summarize(
            f"adaptive MAC+FEC base=16 extra={extra}",
            adaptive_overhead,
            adaptive_clean,
            torch.where(adaptive_clean, torch.ones_like(data_auth), data_auth / k),
        )
    )

    print(f"{'strategy':<36} {'xmit/data':>9} {'clean block':>12} {'avg delivered':>14} {'silent corrupt':>15}")
    print("-" * 92)
    for r in results:
        print(
            f"{r['name']:<36} {r['overhead']:>9.2f}x {r['clean_block']:>11.2f}% "
            f"{r['avg_delivered']:>13.2f}% {r['silent_corrupt']:>14.2f}%"
        )

    if hasattr(torch, "xpu"):
        torch.xpu.synchronize()
    print(f"\nDone in {time.time() - t0:.2f}s")


if __name__ == "__main__":
    main()
