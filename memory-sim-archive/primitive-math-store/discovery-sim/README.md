# discovery-sim — Local Primitive-Combination Discovery Simulator

Proposes wirings of primitive policies — including combinations the kit does **NOT**
document — executes each against an SDF sphere-tracer, and keeps only those that pay a
**conserved currency**: field-evaluations per frame at held image-correctness. A candidate
that type-checks but doesn't cut the currency is "painted" and rejected.

This is the Painted-Fence doctrine's Interruption 4 made mechanical: **no named currency →
not a discovery.** Type-validity / text-reach is theater; only execution that charges a
currency is discovery (see `../primitive-simulator/THESIS.md`).

Zero dependencies — std-only Rust, `forbid`-free, faithful to the kit.

## The loop (propose → validate)

1. **Propose** — exhaustively enumerate the Cartesian product of policy choices (the search
   is exhaustive, not biased, so nothing is missed by a heuristic):
   - `step`: Fixed | OverRelax
   - `refine`: None | Bisection | Secant | **Newton** | **IQI** (inverse-quadratic-interpolation)
   - `precision`: FixedEps | RateDistortion
   - `cull`: None | CoarseSeed
2. **Validate** — render each wiring, measure field-evals vs a high-budget ground truth,
   compute miss-rate at depth tolerance `DELTA`.
3. **Verdict** — the **baseline** (`fix/none/fix/-`) defines the correctness bar (its own
   miss-rate). A wiring is a:
   - **discovery** — at least as correct as baseline (miss ≤ baseline) AND >1.02× cheaper,
   - **neutral** — as correct, not cheaper,
   - **painted** — less correct than baseline (it broke correctness to buy speed).

Each variant is annotated with its source domain and whether the kit documents it for
rendering:

| variant | source domain | documented? |
|---|---|---|
| Newton-Raphson | linear-algebra-matrix | NO |
| Inverse-Quadratic-Interpolation | control-numerical-opt | NO |
| Bisection | formal-verification | NO |
| Secant, OverRelax, RateDistortion, CoarseSeed | control/signal/info/geometry | YES (xdsim-validated) |

A winning wiring that includes a NO-document variant is a **NOVEL discovery** — glue the
kit does not contain, found by execution.

## Build & run

```sh
cargo build --release
./target/release/discovery-sim          # default 96x54, truth 192x108
./target/release/discovery-sim 160 90   # higher res, stricter correctness bar
```

Streams a ranked discoveries table + neutral + painted to stdout, and writes every wiring
with its verdict/currency to `discoveries.jsonl`.

## Findings (2026-06-28)

The discovered **principle is robust across resolutions**: a higher-order root-refine
composed with rate-distortion precision pays, while each ingredient alone does not.

- Rate-distortion alone is **painted** (it coarsens far-field epsilon → miss ≈ 0.9).
- A higher-order refine alone is **neutral** (~1.0×; it adds evals).
- The **composition** is a discovery: the refine recovers the correctness that RD spent,
  and the pair ends up cheaper *and* more correct than baseline.

| res | top discovery | speedup | miss vs baseline | novel? |
|---|---|---|---|---|
| 96×54 | newton + RD | 1.097× | 0.302 < 0.354 | YES (linear-algebra-matrix) |
| 160×90 | iqi + RD | 1.090× | 0.180 < 0.189 | YES (control-numerical-opt) |

Over-relax (OVR) cuts evals ~33% but destroys correctness (miss → 1.0) → uniformly painted.
That is the "over-step risk" currency `xdsim` names, reproduced here independently.

**Currency on every win:** field-evaluations per frame, correctness held at the baseline's
own miss-rate. These are modest, honest wins (~1.09×) — exactly the size the doctrine says
real discoveries start at. A large number that names no currency is not a discovery.

## Honest scope

- One executor (SDF marcher) → one currency (field-evals). Adding executors (retrieval
  latency, compression bits, solver iterations) is the generalization path; each new
  executor must name its own conserved currency.
- The policy vocabulary is the lever interface; growing it (more refine families, more
  step/cull/precision policies) expands the searchable wiring space.
- This is **not** the text-reach glue ranker (`../_bus-harvest/find_bindings.py`), which
  only redistributes already-documented bridges. This simulator EXECUTES candidates the
  catalog does not contain and measures whether they pay.

*Source doctrine: The Painted Fence — Jesse. Companion: `../primitive-simulator/` (xdsim).*
