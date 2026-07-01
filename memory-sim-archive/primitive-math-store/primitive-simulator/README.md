# Primitive Simulator (`xdsim`)

A zero-dependency Rust engine for **discovering and validating cross-domain primitive compositions** against this primitive store, under the Painted Fence doctrine. It is the *experiment* half of the kit: where the store catalogs primitives and the recipe-inference sim checks which compositions type-check, this simulator measures whether a cross-domain idea **actually pays a conserved currency** — and finds the **binding ("glue") primitives** that join domains few others connect.

Companion docs: [`THESIS.md`](THESIS.md) (the position), `../THESIS.md` and `../HOW-THE-KIT-WORKS.md` (the doctrine), `../_taxonomy-root/ROOT_ATOMS.md` (the 8 atoms).

## The one rule

> A result is a **discovery** only if it (a) **charges a conserved currency** — compute, field-evaluations, bits, error, latency — verified by *execution*, and (b) is **novel** — it bridges domains that don't normally compose, or unlocks a composition that was impossible.

Type-validity is **not** discovery. A composition that type-checks end-to-end is a *validation* of the kit's completeness, not a finding. If a "win" charges no real currency, it is optimism (Interruption 4). This simulator is built to enforce that rule mechanically.

## What it does

Two modes, both streaming newline-delimited JSON (one record per finding) to stdout and `--out`:

### `--mode optimize` (default) — empirical discovery
Runs an embedded CPU sphere-tracer (the game engine's marcher: `fold` over an SDF `compare` field) with the optimization levers exposed as parameters, validated against a high-budget ground-truth render every candidate. It:
1. Computes **cross-domain convergence** of the 8 root atoms (how many domains independently realize each — the doctrine's validation test).
2. Runs a **real-wall census** (how many store primitives name a conserved currency vs are free/painted).
3. Tests **cross-domain generators** harvested from the store as experiments, each measured against the field-evaluation currency at held correctness, each emitting a painted/real verdict:
   - `over_relaxation` (signal-RF matched-filter), `rate_distortion_eps` (information-theory), `empty_space_seed` (computational-geometry), `adaptive_shadow_cap` (control-opt), `secant_root_refine` (control-opt secant/regula-falsi).
4. Runs a `(μ+λ)` **evolution strategy** over the joint lever space to find the validated Pareto-best marcher.

### `--mode binding` — glue-primitive finder
Parses every `<domain>/PRIMITIVES.md`, computes each primitive's **reach** (home domain + domains its text references), and scores **binding power** = Σ 1/(primitives sharing each domain-pair) × reach-breadth × real-wall utility bonus. Rare cross-domain bridges dominate, so the top results are the primitives that uniquely glue distant domains.

## Build & run

```sh
cargo build --release            # zero external dependencies, std only

# empirical discovery (45 s), validating cross-domain generators on the marcher:
./target/release/xdsim --mode optimize --library ".." --budget-seconds 45 --out results.jsonl --res 96x54

# binding / glue primitives across the whole store:
./target/release/xdsim --mode binding  --library ".." --out binding.jsonl
```

`--library` points at this store's root (the folder holding the `<domain>/PRIMITIVES.md` files). On Windows the bundled binary is `xdsim.exe`.

## Output

Each line is a JSON object with `seq`, `ts`, `kind`, and kind-specific fields. Notable kinds:
`library_loaded`, `convergence`, `library_census`, `ground_truth`, `experiment` (with `currency`, `wall` verdict), `improvement`, `final`; and in binding mode `binding_start`, `binding` (with `binding_score`, `reach`, `rare_bridges`).

## Findings to date

- **Validated cross-domain transfer:** `secant_root_refine` (numerical-optimization → marcher) cut field-evaluations ~1.17× standalone at held correctness, and the search *adopted* it in the joint optimum (best lifted from 2.337× to 2.375× vs baseline). It charges a real currency — a genuine, if modest, discovery.
- **Rejected (painted):** `empty_space_seed` returned neutral (0.995×) — type-plausible, empirically worthless. Distinguishing it from the secant win is Interruptions 1 and 4 automated.
- **Top binding primitive:** `BVH-traverse` (graphics) — bridges graphics ↔ computational-geometry ↔ queueing-theory, names a real wall (SAH traversal cost), and is the engine's missing spatial-acceleration glue.

## Infrastructure notes

- The recipe-inference sim (`spiderweb-infer-primitives`) runs on a remote host and scores **type-validity only** — use it to validate/stress-test structure, never to claim a discovery (its score plateaus at a fixed type-optimum almost immediately).
- Long remote runs must be detached (`nohup ... </dev/null &`, reparented to init) and survive logout (`loginctl enable-linger`); a prior 8-hour run died at ~10 minutes because it ran in a session foreground.
- Stream `.ps1` helpers must be ASCII-only (PowerShell 5.1 mis-decodes UTF-8 em-dashes).
