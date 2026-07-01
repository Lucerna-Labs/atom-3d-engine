# Ordo UX Frontier Lab

This folder is for the far more experimental, difficult Ordo UX architecture work.

The regular workspace `crates/` folder is the crate store: small, focused experiments that compile today. This `frontier-lab/` folder is different. It is a staging ground for bigger architecture questions that may need multiple crates, several design passes, and more courage than a normal experiment.

## Purpose

Use this lab for ideas that are:

- cross-cutting across many crates
- hard to reverse once implemented
- likely to need research notes
- likely to need multiple prototype passes
- not ready to enter the root Cargo workspace

## Current Reference

The Rust UI architecture research PDF has been copied into:

`frontier-lab/references/Deep Research_ Rust UI Architecture - Google Docs.pdf`

Treat it as source material for later reading and synthesis. Do not assume every frontier crate below must be implemented exactly as listed; this is a working map.

## Folder Layout

- `references/`: PDFs, papers, links, screenshots, and external research artifacts.
- `notes/`: design notes, questions, sketches, tradeoff logs.
- `crates/`: future frontier crates or crate sketches, separate from the production-facing crate store.
- `CRATE_LAYOUT.md`: the lateral crate-by-crate roadmap.

## Rule

Nothing in `frontier-lab/crates/` should be added to the root workspace until we intentionally promote it. This keeps the main crate store stable while the frontier stays free to be weird.

## Working Style

We work laterally:

1. Sketch the crate boundary.
2. Define the data contracts.
3. Write tiny tests or simulations.
4. Compare with adjacent crates.
5. Promote only when the boundary feels inevitable.

The goal is not to rush to a renderer or app shell. The goal is to discover the architecture that can hold Ordo UX without making it brittle.
