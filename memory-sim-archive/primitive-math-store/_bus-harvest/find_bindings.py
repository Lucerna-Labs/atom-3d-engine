#!/usr/bin/env python3
"""
find_bindings.py — find BINDING ("glue") primitives in a primitive store.

A binding primitive, in the Painted-Fence / kit sense, is one that JOINS two or more
primitives (or whole domains) that normally would not compose — glue. The rarer the
domain-pair it bridges (few other primitives connect those two domains), and the broader
its reach, and the more it names a real wall (conserved currency), the higher its binding
power. This is the same notion xdsim's `--mode binding` scores; this script does it as a
pure text/reach analysis over the markdown catalogs.

DO NOT confuse this with logical binders (∀ ∃ λ). That is a different analysis; this
script is about cross-domain GLUE.

Score (per the doctrine, see ../HOW-THE-KIT-WORKS.md and the cross-domain-primitive-discovery skill):
    reach(p)            = {home_domain} ∪ {domains whose essence appears in p's body text}
    pair_count[(A,B)]   = # of primitives whose reach contains both A and B
    binding_power(p)    = ( Σ over each pair {A,B} ⊆ reach(p) of 1/pair_count[(A,B)] )
                          × reach_breadth(p) × real_wall_bonus(p)

A primitive that only lives in its home domain (reach = {home}) bridges nothing → score 0.
A primitive reaching a distant domain few others touch → rare pair, 1/pair_count large → dominates.

Usage:
    python find_bindings.py [<library-root>]   # default library-root = parent of this script
                                            #   (the dir holding <domain>/PRIMITIVES.md)
    [--top N]       top-N primitives to show / put in the report (default 40)
    [--out PATH]    global ranking markdown (default <root>/_bus-harvest/BINDINGS.md)
    [--json PATH]   full per-primitive JSON  (default <root>/_bus-harvest/bindings.json)
    [--bridges PATH] rarest domain-pair edges (default <root>/_bus-harvest/domain-bridges.md)
    [--min-score F] drop primitives below this score (default 0; >0 hides home-only prims)
    [--quiet]
"""
from __future__ import annotations
import re
import sys
import json
import argparse
from pathlib import Path
from datetime import date
from collections import defaultdict
from itertools import combinations


# ---------------------------------------------------------------------------
# Domain essence keywords.
# When a primitive's body matches a domain's keyword, that domain is in its reach.
# Phrases are matched case-insensitively; bare words use word boundaries.
# Deliberate overlaps (bloom filter, markov chain, waveguide, optimization, fourier)
# are GENUINE bridges between domains — that is the signal we want.
# ---------------------------------------------------------------------------
DOMAIN_KEYWORDS: dict[str, list[str]] = {
    "agentic-reasoning": ["agent", "agentic", "chain-of-thought", "tool use", "multi-agent", " llm", "reasoning model", "world model"],
    "astrophysics-cosmology": ["astrophysic", "cosmolog", "black hole", "general relativity", "gravitational", "star formation", "galaxy", "n-body"],
    "biology-bioinformatics": ["biolog", "bioinformatic", "genom", "protein", "sequencing", "molecular dynamics", " rna", " dna", "phylogen"],
    "causal-inference": ["causal", "do-calculus", "counterfactual", "confound", "instrumental variable", "interventional", "dag "],
    "cognitive-primitives": ["cognitive", "perception", "working memory", "neuroscience", "cognition", "brain-inspired"],
    "combinatorial-optimization": ["combinatorial optimization", "traveling salesman", " tsp", "matroid", "submodular", "integer program", "max-cut", "graph coloring"],
    "computational-geometry": ["computational geometry", "convex hull", "voronoi", "delaunay", " bvh", "bounding volume", "sphere tracing", " sdf", "triangulat", "spatial index"],
    "condensed-matter": ["condensed matter", "band structure", "phonon", "superconduct", "magnetism", "fermion", "lattice model"],
    "control-numerical-opt": ["control theory", "feedback control", "lyapunov", "convex optimization", "gradient descent", "pid controller", "kalman filter", "optimal control", "model predictive"],
    "cryptography-advanced": ["zero-knowledge", " zk ", "zkp", "multi-party computation", " mpc ", "homomorphic encryption", " fhe", "threshold signature", "commitment scheme"],
    "cryptography-hashing": ["hash function", "sha-256", "blake3", "murmurhash", "minhash", "simhash", "bloom filter", "cryptograph", "message authentication"],
    "database-streaming-sketching": ["streaming algorithm", "count-min", "hyperloglog", "lsm tree", "log-structured", "change data capture", " cdc", "key-value", "windowing", "sketch"],
    "decision-logic": ["decision theory", "utility function", "game theory", "nash", "voting", "social choice", "expected utility", "bayesian game"],
    "distributed-systems": ["distributed system", "consensus", "paxos", " raft", "replication", "quorum", " crdt", "byzantine", "cap theorem", "fencing token"],
    "electromagnetics-antennas": ["maxwell", "electromagnetic", "antenna", "propagation", "dielectric", "scattering", "radar"],
    "formal-verification": ["model checking", "theorem proving", "formal verification", "symbolic execution", "abstract interpretation", " tla+", "satisfiability", " sat solver", "k-induction"],
    "graphics-rendering-lod": ["rendering", "rasteriz", "path tracing", "ray tracing", " pbr", "shading", "texture map", " lod", "level of detail", "gpu pipeline"],
    "information-theory-coding": ["information theory", " shannon", "channel capacity", "error-correcting code", "rate-distortion", "mutual information", "source coding", "arithmetic coding"],
    "linear-algebra-matrix": ["linear algebra", "matrix decomposition", " svd", "eigen", "qr decomposition", "cholesky", "matmul", " gemm", "tensor product", "singular value"],
    "logic-reasoning": ["propositional logic", "first-order logic", "modal logic", "temporal logic", "inference rule", "nonmonotonic", "deduction", "predicate logic"],
    "ml-training": ["machine learning", "neural network", "backprop", "transformer", "attention mechanism", "loss function", "optimizer", "reinforcement learning", "deep learning", "gradient"],
    "networking": [" tcp", " udp", " quic", "routing table", " bgp", " dns", " tls", " http", "packet switching", "congestion control", " sdn"],
    "operating-systems": ["operating system", "kernel space", "kernel module", "linux kernel", "system call", "syscall", "page fault", "virtual memory", "context switch", " ipc", "file system"],
    "photonics-optics": ["photonics", "optics", "laser", "waveguide", "fiber optic", "photonic", "optical", "lens"],
    "physics-diffusion": ["diffusion equation", "heat equation", "fokker-planck", "heat kernel", "laplacian", "statistical mechanic", "brownian", "random walk"],
    "quantum-computing": ["quantum", "qubit", "quantum gate", "entangle", "quantum fourier", "superposition", " qec"],
    "queueing-theory-stochastic-processes": ["queueing", "markov chain", "poisson", "stochastic process", "renewal", "birth-death", "little's law", "mdp"],
    "retrieval-search": ["retrieval", "search engine", "inverted index", " bm25", "tf-idf", "vector search", " ann", "learning to rank", " ltr", "relevance"],
    "signal-processing-rf": ["signal processing", " rf", "radio frequency", "matched filter", " fft", " ofdm", "beamform", "sampling theorem", "fir filter", "iir filter"],
    "statistics-probability": ["statistics", "probability distribution", "bayesian inference", "maximum likelihood", " mle", "hypothesis testing", "confidence interval", "regression", "variance"],
    "type-theory-programming-languages": ["type theory", "type system", "lambda calculus", "programming language", "compiler", "type checking", "type inference", "polymorphism", "static analysis", "denotational semantics", "operational semantics"],
}

# Real-wall / conserved-currency markers. A primitive that names one gets a bonus
# (load-bearing glue beats free glue — Interruption 4: name the currency).
REAL_WALL_MARKERS = [
    r"real wall", r"conserved", r"shannon", r"nyquist", r"uncertainty principle",
    r"spectral gap", r"min-cut", r"max-flow", r"percolation", r"np-hard", r"np-complete",
    r"lower bound", r"no free lunch", r"capacity limit", r"entropy floor", r" cfl",
    r"stability condition", r"fundamental limit", r"rate-distortion", r"trade-?off",
    r"fisher information", r"sort.? lower bound", r" o\(n log n\)",
]
_REAL_WALL_RE = re.compile("|".join(REAL_WALL_MARKERS), re.IGNORECASE)


def parse_primitives(text: str) -> list[tuple[str, str, str]]:
    """Parse PRIMITIVES.md into (prim_id, header, body).

    Handles three header formats:
      1. `### [PRIM-001] state-transition-system`
      2. `### [PRIM-090] a / [PRIM-091] b`            (combined -> one entry per id)
      3. `### loss-mse (cross-domain alias: ...)`     (slug as id)

    A primitive's body ENDS at the next `##` or `###` header — never bleeds into a
    following `## Summary` / `## Appendix` section (which would pollute reach detection).
    """
    # Boundaries are any H2 or H3 header. Primitives are the H3 entries.
    headers = list(re.finditer(r"^(#{2,3})\s+(.+?)$", text, re.MULTILINE))
    out: list[tuple[str, str, str]] = []
    for i, m in enumerate(headers):
        if len(m.group(1)) != 3:        # only H3 lines are primitives
            continue
        header_text = m.group(2).strip()
        body_start = m.end()
        body_end = headers[i + 1].start() if i + 1 < len(headers) else len(text)
        body = text[body_start:body_end]
        ids = re.findall(r"\[PRIM-(\d+)\]", header_text)
        if not ids:
            slug = re.sub(r"\s*\(.*\)\s*$", "", header_text).strip()
            slug = re.sub(r"\s+", "-", slug)
            ids = [slug]
        for pid in ids:
            pid = f"PRIM-{pid}" if pid.isdigit() else pid
            out.append((pid, header_text, body))
    return out


def build_keyword_index():
    """Compile (compiled_regex, domain) pairs. Bare-word keywords get word boundaries."""
    pairs = []
    for domain, kws in DOMAIN_KEYWORDS.items():
        for kw in kws:
            kw = kw.strip()
            if re.fullmatch(r"\w[\w-]*", kw):           # single token -> word-boundary
                pat = re.compile(rf"\b{re.escape(kw)}\b", re.IGNORECASE)
            else:                                        # phrase -> literal (already has boundary cues)
                pat = re.compile(re.escape(kw), re.IGNORECASE)
            pairs.append((pat, domain))
    return pairs


def reach_of(body: str, home: str, index) -> set[str]:
    """Domains whose essence appears in the body. Always includes home."""
    reached = {home}
    for pat, domain in index:
        if domain == home:
            continue
        if pat.search(body):
            reached.add(domain)
    return reached


def is_real_wall(body: str) -> bool:
    return bool(_REAL_WALL_RE.search(body))


def load_library(root: Path):
    """Return list of dicts: {domain, prim_id, header, body, path}."""
    index = build_keyword_index()
    prims = []
    for d in sorted(p for p in root.iterdir() if p.is_dir()):
        f = d / "PRIMITIVES.md"
        if not f.exists():
            continue
        domain = d.name
        if domain not in DOMAIN_KEYWORDS:
            continue  # skip nested dup / non-domain dirs (e.g. 'primitves math', 'output')
        text = f.read_text(encoding="utf-8")
        for pid, header, body in parse_primitives(text):
            prims.append({
                "domain": domain, "prim_id": pid, "header": header,
                "body": body, "path": str(f),
            })
    return prims, index


def score_library(prims, index):
    """Annotate each primitive with reach, bridges, binding_power. Returns pair_count map."""
    # 1. reach
    for p in prims:
        p["reach"] = reach_of(p["body"], p["domain"], index)
        p["reach_breadth"] = len(p["reach"])
        p["real_wall"] = is_real_wall(p["body"])

    # 2. pair counts over the domain graph
    pair_count: dict[tuple[str, str], int] = defaultdict(int)
    for p in prims:
        for a, b in combinations(sorted(p["reach"]), 2):
            pair_count[(a, b)] += 1

    # 3. binding power
    for p in prims:
        raw = 0.0
        rare = []
        for a, b in combinations(sorted(p["reach"]), 2):
            c = pair_count[(a, b)]
            contrib = 1.0 / c
            raw += contrib
            rare.append((a, b, c, contrib))
        bonus = 1.5 if p["real_wall"] else 1.0
        breadth = p["reach_breadth"]
        p["binding_power"] = round(raw * breadth * bonus, 4)
        p["raw_rarity"] = round(raw, 4)
        # the bridges this primitive forms, most-rare first
        p["bridges"] = sorted(rare, key=lambda x: x[3], reverse=True)
    return pair_count


def domain_label(d: str) -> str:
    return d


def render_report(prims, pair_count, top=40, min_score=0.0):
    ranked = [p for p in prims if p["binding_power"] >= min_score]
    ranked.sort(key=lambda p: p["binding_power"], reverse=True)
    top_n = ranked[:top]
    total = len(prims)
    bridging = sum(1 for p in prims if p["reach_breadth"] > 1)
    lines = [
        "# Binding (\"glue\") Primitive Ranking",
        "",
        f"**Date:** {date.today().isoformat()}  ",
        f"**Primitives scanned:** {total}  ",
        f"**Primitives that bridge ≥2 domains:** {bridging} ({round(100*bridging/total,1)}%)  ",
        f"**Distinct domain pairs observed:** {len(pair_count)}  ",
        f"**Domains:** {len(set(p['domain'] for p in prims))}",
        "",
        "## Score",
        "`binding_power = ( Σ over each bridged pair {A,B} of 1/pair_count[A,B] ) × reach_breadth × real_wall_bonus`  ",
        "Rare pairs (few primitives connect A↔B) dominate. `real_wall_bonus = 1.5` if the primitive names a conserved currency, else 1.0.",
        "",
        f"## Top {len(top_n)} binding primitives",
        "",
        "| Rank | Score | Prim | Home domain | Reach | Rarest bridge (A↔B : #shared) | Real wall? |",
        "|---:|---:|---|---|---|---|:--:|",
    ]
    for i, p in enumerate(top_n, 1):
        reach = ", ".join(sorted(p["reach"]))
        rarest = p["bridges"][0] if p["bridges"] else None
        rarest_s = f"{rarest[0]} ↔ {rarest[1]} : {rarest[2]}" if rarest else "—"
        rw = "yes" if p["real_wall"] else ""
        name = p["header"].split("(")[0].strip()[:48]
        lines.append(f"| {i} | {p['binding_power']} | `{p['prim_id']}` {name} | {p['domain']} | {reach} | {rarest_s} | {rw} |")
    lines += [
        "",
        "## What \"binding\" means here",
        "A binding primitive is GLUE: it joins primitives or whole domains that normally would not compose. ",
        "A primitive reaching only its home domain scores 0 (it bridges nothing). The top entries are the rare ",
        "cross-domain bridges — the adapters that make distant fields compose. Validate the engine-relevant ones ",
        "empirically (xdsim `--mode optimize`); a high binding score is a nomination, not a proof of utility.",
        "",
        "---",
        "*Generated by `find_bindings.py` (glue/binding finder). Not the ∀∃λ logical-binder analysis.*",
    ]
    return "\n".join(lines), ranked


def render_bridges(prims, pair_count):
    """The rarest domain-pair edges in the graph — structural gaps only a few primitives cross."""
    edges = sorted(pair_count.items(), key=lambda kv: kv[1])  # rarest first
    # map pair -> primitives that bridge it
    pair_prims: dict[tuple[str, str], list[str]] = defaultdict(list)
    for p in prims:
        for a, b in combinations(sorted(p["reach"]), 2):
            pair_prims[(a, b)].append(f"{p['prim_id']}({p['domain']})")
    # only show pairs that actually exist and are rare-ish
    shown = [(pair, c) for pair, c in edges if c <= 8]
    lines = [
        "# Rarest Domain-Pair Bridges (structural glue gaps)",
        "",
        f"**Date:** {date.today().isoformat()}  ",
        "Pairs of domains joined by the FEWEST primitives. A bridge nobody else makes is the highest-leverage glue to extract or verify.",
        "",
        "| Domain A | Domain B | # bridging prims | Bridging primitives |",
        "|---|---|---:|---|",
    ]
    for (a, b), c in shown[:60]:
        pl = pair_prims[(a, b)]
        lines.append(f"| {a} | {b} | {c} | {', '.join(pl[:6])}{'…' if len(pl) > 6 else ''} |")
    lines += [
        "",
        "---",
        "*Generated by `find_bindings.py`.*",
    ]
    return "\n".join(lines)


def main():
    here = Path(__file__).resolve().parent
    default_root = here.parent  # repo root holding <domain>/PRIMITIVES.md
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("library", nargs="?", type=Path, default=default_root,
                    help=f"library root (dir of <domain>/PRIMITIVES.md); default {default_root}")
    ap.add_argument("--top", type=int, default=40)
    ap.add_argument("--out", type=Path, default=None)
    ap.add_argument("--json", type=Path, default=None)
    ap.add_argument("--bridges", type=Path, default=None)
    ap.add_argument("--min-score", type=float, default=0.0)
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    root = args.library.resolve()
    if not root.is_dir():
        sys.exit(f"ERROR: {root} is not a directory")

    prims, index = load_library(root)
    if not prims:
        sys.exit("ERROR: no <domain>/PRIMITIVES.md found under known domains")
    pair_count = score_library(prims, index)

    report, ranked = render_report(prims, pair_count, top=args.top, min_score=args.min_score)
    bridges_md = render_bridges(prims, pair_count)

    # defaults live beside the script
    bus = root / "_bus-harvest"
    bus.mkdir(parents=True, exist_ok=True)
    out_path = args.out or (bus / "BINDINGS.md")
    json_path = args.json or (bus / "bindings.json")
    bridges_path = args.bridges or (bus / "domain-bridges.md")

    out_path.write_text(report, encoding="utf-8")
    bridges_path.write_text(bridges_md, encoding="utf-8")
    serial = [{k: v for k, v in p.items() if k not in ("body",)} for p in ranked]
    for p in serial:
        p["reach"] = sorted(p["reach"])
    json_path.write_text(json.dumps(serial, indent=2, ensure_ascii=False), encoding="utf-8")

    if not args.quiet:
        top_n = ranked[: min(args.top, 20)]
        print(f"=== Binding (glue) primitive finder - {root.name} ===")
        print(f"Primitives: {len(prims)} | bridging >=2 domains: {sum(1 for p in prims if p['reach_breadth']>1)} | domain-pairs: {len(pair_count)}")
        print(f"\nTop {len(top_n)}:")
        for i, p in enumerate(top_n, 1):
            rarest = p["bridges"][0]
            rw = " [wall]" if p["real_wall"] else ""
            name = p["header"].split("(")[0].strip()[:40]
            a, b = rarest[0], rarest[1]
            print(f"  {i:2d}. {p['binding_power']:8.3f}  {p['prim_id']:>14}  {name:<40} {p['domain']:<22} {a}<->{b} ({rarest[2]}){rw}")
        print(f"\nReports: {out_path}\n         {bridges_path}\n         {json_path}")


if __name__ == "__main__":
    main()
