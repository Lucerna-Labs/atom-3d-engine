# RF + Crypto Self-Repair Notes

## 2026-07-02 Arc/XPU simulation

Command:

```powershell
& "C:\Projects\discovery-search-12h-gpu\.venv-arc\Scripts\python.exe" experiments\rf-crypto-arc\rf_crypto_self_repair.py --trials 131072 --k 48 --m 28 --extra 24
```

Device/runtime:

- `Intel(R) Arc(TM) A380 Graphics`
- `torch 2.12.1+xpu`
- CUDA disabled / unused
- `131072` simulated RF blocks
- Block shape: `k=48` data chunks, default repair `m=28`, adaptive extra repair `24`
- Channel: bursty fading loss plus random tamper/corruption

Results:

| Strategy | Tx/data | Clean block | Avg delivered | Silent corrupt |
|---|---:|---:|---:|---:|
| raw/no crypto | `1.00x` | `2.90%` | `87.29%` | `17.90%` |
| raw + per-frame MAC | `1.00x` | `2.90%` | `86.87%` | `0.00%` |
| rep2 adjacent + MAC | `2.00x` | `12.97%` | `94.53%` | `0.00%` |
| rep2 interleaved + MAC | `2.00x` | `52.80%` | `98.22%` | `0.00%` |
| FEC/RLNC `m=28` + per-shard MAC | `1.58x` | `99.82%` | `99.93%` | `0.00%` |
| FEC/RLNC `m=28` + block hash only | `1.58x` | `72.94%` | `96.28%` | `0.00%` |
| FEC/RLNC `m=28` no crypto | `1.58x` | `72.94%` | `99.95%` | `26.93%` |
| FEC/RLNC `m=28` + MAC + interleave | `1.58x` | `100.00%` | `100.00%` | `0.00%` |
| adaptive MAC+FEC base=16 extra=24 | `1.43x` | `100.00%` | `100.00%` | `0.00%` |

## Interpretation

The winning stack is:

```text
sequence/nonce -> per-shard MAC or AEAD tag -> drop failed shards -> FEC/RLNC repair -> dedup/reorder -> bus off-ramp
```

The important mechanism is **crypto before repair**. A MAC turns tamper into erasure, and erasures are exactly what FEC/RLNC can repair. Without crypto, corrupted equations silently poison the decoded block. With only a block hash, corruption is detected late, but the receiver cannot tell which shard poisoned the block, so repair is weaker.

Primitive readout:

- **Per-shard MAC / AEAD tag:** high priority. This is the gate that converts corruption into recoverable loss.
- **FEC/RLNC / fountain repair:** high priority. Reconstructs dropped shards without ARQ.
- **Interleaving:** very strong against burst fades, but costs latency.
- **Adaptive redundancy:** best efficiency in this sim: `1.43x` average overhead with full recovery, versus fixed `1.58x`.
- **Repetition:** useful but inefficient. Interleaving makes repetition much better, but still loses to FEC/RLNC.
- **Block hash only:** useful as a final commitment/check, but not enough for self-repair.

## Bus Design Note

For the spiderweb bus, this belongs as an RF/high-loss lane wrapper, not inside the kernel:

```text
ground bus -> RF on-ramp -> shard + authenticate -> lossy lane -> verify + erase bad shards -> repair -> off-ramp
```

Emit vibrations from the RF lane:

- rising MAC failures -> possible tamper/noise vibration
- rising erasures -> link quality/backpressure vibration
- repair overhead crossing a threshold -> route-cost vibration

The spider should then raise/lower redundancy or demote traffic to a safer lane.

## Rust Sanity Check

Existing `spiderweb-rf` examples also passed:

- `chaos`: fading link delivered `393/500`, about `21%` loss, deterministic by seed.
- `fortify`: bare fading lane delivered `38/48`; fountain repair recovered `48/48` with `28` repair shards.

