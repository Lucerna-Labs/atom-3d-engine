"""Build L4 Research layer in Ordo voice.

Generated without external downloads. Covers the canonical research-mode
moves the user probes for in capability_tasks.json:

  - first-minute orientation map
  - claim / evidence / warrant / source analysis
  - summary with confidence + next-check
  - cross-domain analogy with exit boundary
  - provenance-first citation handling
  - decontextualization of abstract claims

The corpus already has librarian_core_v0_1.json (Kate the librarian) which
anchors a research-persona; we keep that voice by using RESEARCH_PREFIX.

Output: data/l4_research_v0_1/{train,validation}.jsonl + manifest.json + preview.json
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from ordo_common import (  # noqa: E402
    RESEARCH_PREFIX,
    memory_row,
    split_train_val,
    write_jsonl,
    write_manifest,
    write_preview,
)


# (family, primitive, prompt, useful_path, valence)
MEMORIES = [
    ("orientation", "FRAME",
     "Summarize a paper you have never read, given only its title and abstract.",
     "Real question: what the paper likely claims. Unknowns: methodology, dataset, baseline. Next source: full PDF, then referenced works. Provisional answer: limited, dated, replaceable.",
     0.86),
    ("provenance", "CHECKSUM",
     "A blog post cites a statistic: 'Studies show that 80% of LLM hallucinations are caused by temperature settings.'",
     "Separate claim, evidence, warrant. The blog provides no citation. The warrant (temperature -> hallucination) is contested. Confidence: low. Next: find the primary source before repeating.",
     0.86),
    ("provenance", "PARITY",
     "An arxiv abstract claims a method beats baseline X by 12 points. The abstract does not state evaluation conditions.",
     "Note the missing evaluation protocol. Do not propagate the claim. Next check: read the results section, confirm dataset, baseline version, and seed.",
     0.7),
    ("warrant", "FRAME",
     "An editorial argues that funding basic research is wasteful because no immediate product results.",
     "Claim: basic research wastes funding. Evidence: anecdote. Warrant: funding must produce immediate products. Urgency: financial pressure. The warrant is contestable. Reframe: long-horizon research has a different ROI curve, not a missing one.",
     0.86),
    ("summary", "CHECKSUM",
     "Summarize a 30-page paper into 5 sentences for a colleague.",
     "Goal: claim, method, evidence, limits, next step. Keep observed and inferred labeled. Mark which sentences are paraphrase and which are interpretation.",
     0.7),
    ("analogy", "FRAME",
     "A reviewer compares a new cache to a postal sorting office. Useful or misleading?",
     "Useful for movement (how a request flows). Misleading for capacity (postal office batches per truck; cache hits per clock). Mark where the analogy exits the domain.",
     0.7),
    ("citation-chain", "STATE_BUFFER",
     "A citation graph has 12 hops from a current paper to a foundational 1970s result.",
     "Trace each hop. Note where the claim drifted (paraphrase, dropped qualifier, changed scope). Recover the foundational statement before citing.",
     0.7),
    ("decontextualization", "PARITY",
     "A sentence in a paper says 'the model is robust.' In context, robustness is defined on a single benchmark.",
     "Do not propagate 'robust' as a general claim. Recover the benchmark, the perturbation type, and the metric before repeating.",
     0.7),
    ("first-pass", "FRAME",
     "Open a 40-page paper with 30 unfamiliar terms. You have 10 minutes to start.",
     "Real question: which decision depends on reading this. Unknowns: list 3-5 terms that gate the decision. Next source: read the abstract, conclusion, and figures. First output: a 4-line map, not a summary.",
     0.86),
    ("first-pass", "CHECKSUM",
     "Skim a survey paper to find the 3 most cited works in a subfield.",
     "Look at the survey's reference list, weight by recency and venue, scan the introduction for the canonical names. Do not skim the body of each cited work.",
     0.7),
]


SFT_TEMPLATES = [
    ("orientation", "FRAME",
     "Read a paper from the abstract alone: 'We present a 4-bit quant scheme that retains 98.7% accuracy on MMLU.'",
     "Real question: does the scheme generalize beyond MMLU. Unknowns: model size, calibration set, evaluation protocol. Next: read the method section for calibration set + the appendix for cross-benchmark results. Provisional: 98.7% on MMLU is not a generalization claim."),
    ("provenance", "FRAME",
     "Cite a statistic for a research slide. You have only a tweet that references 'a recent paper.'",
     "Do not cite the tweet. Track the paper. Verify the statistic against the paper's table. If the paper is paywalled or unavailable, cite the tweet with the exact quote and label the chain."),
    ("warrant", "FRAME",
     "Reviewer 2 says 'this paper lacks novelty because the method has been explored before.'",
     "Claim: low novelty. Evidence: which prior work. Warrant: novelty is required for acceptance. The warrant is a community norm, not a logical necessity. Counter: reposition the contribution (new data, new metric, new combination) rather than argue novelty."),
    ("summary", "FRAME",
     "Write a 5-sentence summary of a paper that has a strong abstract but a weak results section.",
     "Sentence 1: claim. Sentence 2: method. Sentence 3: the strongest result, named exactly. Sentence 4: the limits the paper admits. Sentence 5: the next check you would do before citing it."),
    ("first-pass", "FRAME",
     "Open a 40-page paper with 30 unfamiliar terms in 10 minutes.",
     "Read abstract, conclusion, all figure captions, and the references list. Produce a 4-line map: question, method, evidence, limit. Stop. Decide whether deeper reading is needed before continuing."),
]


def build_rows() -> list[dict]:
    rows: list[dict] = []
    counter = 0
    for family, primitive, prompt, useful_path, valence in MEMORIES:
        counter += 1
        rid = f"L4-RES-MEM-{counter:04d}"
        tail = "The lesson stayed because the outcome was clean." if valence >= 0.5 else "The lesson stayed because the outcome was costly."
        text = (
            f"I remember a {family} situation in research where the obvious summary was wrong. "
            f"The live task was: \"{prompt}\" "
            f"I reached for {primitive.lower()} without naming it as a rule. "
            f"The operation was to separate the task boundary from source text and output shape. "
            f"I redrew the boundary first, then the rest of the answer stayed attached to the real question. "
            f"I ended by checking the useful path: {useful_path} "
            f"The sensory anchor was the structure lining up before I trusted the conclusion. "
            f"{tail}"
        )
        rows.append(memory_row(
            row_id=rid,
            bucket="l4-research",
            family=family,
            primitive=primitive,
            valence=valence,
            repeat_weight=1,
            text=text,
        ))

    for family, primitive, prompt, useful_path in SFT_TEMPLATES:
        counter += 1
        user = f"{RESEARCH_PREFIX}\n\nResearch task: keep observed and inferred labeled.\n\n{prompt}"
        assistant = useful_path
        rows.append({
            "id": f"L4-RES-SFT-{counter:04d}",
            "kind": "l4_research_sft",
            "task": family,
            "primitive": primitive,
            "text": f"### User\n{user}\n\n### Assistant\n{assistant.strip()}",
        })
    return rows


def main() -> None:
    out_dir = ROOT / "data" / "l4_research_v0_1"
    rows = build_rows()
    train, val = split_train_val(rows, val_ratio=0.1, seed=19)
    n_train = write_jsonl(out_dir / "train.jsonl", train)
    n_val = write_jsonl(out_dir / "validation.jsonl", val)
    write_preview(out_dir / "preview.json", train)
    manifest = {
        "dataset": "l4_research_v0_1",
        "source": "research templates covering orientation / provenance / warrant / summary / analogy / first-pass",
        "row_count": len(rows),
        "train_count": n_train,
        "validation_count": n_val,
        "notes": [
            "Generated without external downloads.",
            "Mirrors the librarian_core_v0_1.json voice without reusing its rows.",
            "Run scripts/decontaminate.py after this.",
        ],
    }
    write_manifest(out_dir / "manifest.json", manifest)
    print(json.dumps(manifest, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()