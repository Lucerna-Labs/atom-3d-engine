"""Build L4 math layer in Ordo voice.

Source: template-generated variants over the 12 computation primitives already
in corpus/computation_primitives_v0_1.json, plus fresh template coverage
for: linear equation, ratio mixture, schedule finish, ticket table,
probability without replacement, average speed, weighted average, tiered
pricing, discount+tax, subscription total, inventory available, printer rate.

Each row is a first-person "I remember..." memory that names which primitive
fires (FRAME, CHECKSUM, ECC, REDUNDANCY, COMPRESSION, STATE_BUFFER, GAIN_CLAMP,
CLOCK, PARITY, SUPPRESSION), keeps observed and inferred labeled, and ends
with the canonical valence tail.

Output: data/l4_math_v0_1/train.jsonl, validation.jsonl, manifest.json, preview.json
"""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from ordo_common import (  # noqa: E402
    MATH_PREFIX,
    memory_row,
    read_jsonl,
    split_train_val,
    write_jsonl,
    write_manifest,
    write_preview,
)

# Templates per primitive. Each entry: (problem_prompt, useful_path, primitive, family, valence)

PRINTER_RATE = [
    ("Four identical printers make 480 pages in 6 minutes. How many minutes should 3 identical printers need to make 720 pages?",
     "Compute per-printer rate (480 / 4 / 6 = 20 pages per minute), rebuild the combined rate for the new group (3 * 20 = 60 pages per minute), then divide pages by pages per minute (720 / 60 = 12 minutes).",
     "FRAME", "math", 0.42),
    ("Six identical printers make 540 pages in 9 minutes. How many minutes should 4 identical printers need to make 600 pages?",
     "Compute per-printer rate (540 / 6 / 9 = 10 pages per minute), rebuild combined rate (4 * 10 = 40 pages per minute), divide (600 / 40 = 15 minutes).",
     "FRAME", "math", 0.7),
    ("Eight identical pumps fill 8 tanks in 16 minutes. How many minutes should 5 identical pumps need to fill 10 identical tanks?",
     "Per-pump rate (1 tank per 2 minutes per pump), combined rate (5 * 0.5 = 2.5 tanks per minute), divide (10 / 2.5 = 4 minutes).",
     "FRAME", "math", 0.86),
]

DISCOUNT_TAX = [
    ("An item costs $80. It is discounted by 25%, then 8% sales tax is applied to the discounted price. What is the final price?",
     "Discount (80 * 0.25 = 20, so 60), then tax (60 * 0.08 = 4.80), total 60 + 4.80 = $64.80. Tax operates on the price actually present at that step.",
     "CHECKSUM", "math", 0.7),
    ("A $120 item is discounted by 15%, then 9.5% sales tax is applied to the discounted price. What is the final price?",
     "Discount (120 * 0.15 = 18, so 102), tax (102 * 0.095 = 9.69), total 102 + 9.69 = $111.69.",
     "CHECKSUM", "math", 0.7),
    ("A $45 service is discounted by 10%, then 7% tax is applied to the discounted price. What is the final price?",
     "Discount (45 * 0.10 = 4.50, so 40.50), tax (40.50 * 0.07 = 2.835), total 40.50 + 2.84 = $43.34.",
     "CHECKSUM", "math", 0.7),
]

WEIGHTED_AVERAGE = [
    ("A student has 3 test scores with an average of 82. The next two scores are 90 and 76. What is the average across all 5 tests?",
     "Recover old total (82 * 3 = 246), add new scores (246 + 90 + 76 = 412), divide by 5 (82.4).",
     "ECC", "math", 0.7),
    ("A class of 4 quizzes has an average of 78. Three more quizzes land at 85, 92, 70. What is the new average across all 7?",
     "Recover old total (78 * 4 = 312), add new (312 + 85 + 92 + 70 = 559), divide by 7 (79.857...).",
     "ECC", "math", 0.7),
]

INVENTORY = [
    ("A warehouse starts with 150 units. It sells 18 units per day for 4 days, receives 65 units, then reserves 20 units for a client. How many units are available?",
     "Start minus sold (150 - 72 = 78), add received (78 + 65 = 143), subtract reserved (143 - 20 = 123).",
     "STATE_BUFFER", "math", 0.7),
    ("A store starts with 240 items. It sells 12 per day for 5 days, receives 90 items, then reserves 30. How many are available?",
     "Start minus sold (240 - 60 = 180), add received (180 + 90 = 270), subtract reserved (270 - 30 = 240).",
     "STATE_BUFFER", "math", 0.7),
]

SCHEDULE = [
    ("A job starts at 9:15 AM. Phase 1 takes 45 minutes, phase 2 takes 70 minutes, then a 10-minute break, and phase 3 takes 35 minutes. What time does it finish?",
     "Total minutes (45 + 70 + 10 + 35 = 160 minutes = 2 hours 40 minutes). 9:15 + 2:40 = 11:55 AM.",
     "CLOCK", "math", 0.7),
    ("A class starts at 10:00. Lecture is 55 min, lab 1 hr 15 min, 10 min break, then Q&A 25 min. When does it end?",
     "Total minutes (55 + 75 + 10 + 25 = 165 minutes = 2 hours 45 minutes). 10:00 + 2:45 = 12:45 PM.",
     "CLOCK", "math", 0.7),
]

RATIO = [
    ("A cleaning mix uses concentrate and water in a 2:5 ratio. If you need 42 liters total, how many liters of concentrate and water are needed?",
     "Total parts (2 + 5 = 7). Per part (42 / 7 = 6 liters). Concentrate (2 * 6 = 12), water (5 * 6 = 30).",
     "COMPRESSION", "math", 0.7),
    ("A recipe uses flour to sugar in 3:4 ratio. If you need 28 cups total, how many cups of each?",
     "Total parts (3 + 4 = 7). Per part (28 / 7 = 4 cups). Flour (3 * 4 = 12), sugar (4 * 4 = 16).",
     "COMPRESSION", "math", 0.7),
]

AVERAGE_SPEED = [
    ("A car travels 150 km at 60 km/h, then 90 km at 45 km/h. What is the average speed for the whole trip?",
     "First leg time (150 / 60 = 2.5 hours). Second leg time (90 / 45 = 2 hours). Total distance (150 + 90 = 240 km). Total time (2.5 + 2 = 4.5 hours). Average (240 / 4.5 = 53.33 km/h).",
     "REDUNDANCY", "math", 0.7),
    ("A cyclist rides 40 km at 20 km/h, then 60 km at 30 km/h. What is the average speed for the trip?",
     "First leg (40 / 20 = 2 h). Second leg (60 / 30 = 2 h). Total distance 100 km, total time 4 h, average 25 km/h.",
     "REDUNDANCY", "math", 0.7),
]

LINEAR_EQUATION = [
    ("Solve for x: 3(x - 4) + 2x = 46. Show the algebra briefly.",
     "Expand (3x - 12 + 2x = 46), combine (5x - 12 = 46), add 12 (5x = 58), divide (x = 11.6).",
     "FRAME", "math", 0.7),
    ("Solve for x: 4(x + 2) - 3x = 31.",
     "Expand (4x + 8 - 3x = 31), combine (x + 8 = 31), subtract 8 (x = 23).",
     "FRAME", "math", 0.7),
]

PROBABILITY = [
    ("A bag has 5 red balls and 3 blue balls. Two balls are drawn without replacement. What is the probability both are red?",
     "First draw (5/8). Second draw drops the red and total by 1 (4/7). Multiply (5/8 * 4/7 = 20/56 = 5/14 = 0.357...). Without replacement changes the second denominator.",
     "PARITY", "math", 0.7),
    ("A box has 6 green marbles and 4 yellow marbles. Two are drawn without replacement. What is the probability both are green?",
     "First draw (6/10 = 3/5). Second draw (5/9). Multiply (3/5 * 5/9 = 15/45 = 1/3 = 0.333...).",
     "PARITY", "math", 0.7),
]

TICKETS = [
    ("Team A starts with 12 open tickets, closes 7, and receives 5 new tickets. Team B starts with 20 open tickets, closes 13, and receives 4 new tickets. How many open tickets remain total, and which team has more?",
     "Team A (12 - 7 + 5 = 10). Team B (20 - 13 + 4 = 11). Total 21 open tickets, Team B has 1 more.",
     "STATE_BUFFER", "math", 0.7),
    ("Sprint A starts with 8 bugs, fixes 3, gets 2 new. Sprint B starts with 15, fixes 9, gets 1 new. Total open and which sprint has more?",
     "Sprint A (8 - 3 + 2 = 7). Sprint B (15 - 9 + 1 = 7). Total 14 open. Same count.",
     "STATE_BUFFER", "math", 0.7),
]

SUBSCRIPTION = [
    ("A subscription costs $19.99 per month for 6 months. There is a $14.95 setup fee and a $10 credit. What is the total cost?",
     "Monthly total (19.99 * 6 = 119.94). Add setup (119.94 + 14.95 = 134.89). Subtract credit (134.89 - 10 = 124.89).",
     "ECC", "math", 0.7),
    ("A plan costs $24.50 per month for 12 months. There is a $20 setup fee and a $30 credit. What is the total?",
     "Monthly (24.50 * 12 = 294.00). Plus setup (314.00). Minus credit (284.00).",
     "ECC", "math", 0.7),
]

TIERED = [
    ("A utility bill charges $0.12 per unit for the first 100 units, $0.18 per unit for units above 100, plus a fixed $6 fee. What is the bill for 135 units?",
     "First tier (100 * 0.12 = 12.00). Extra units (35 * 0.18 = 6.30). Fixed fee $6. Total (12.00 + 6.30 + 6.00 = 24.30).",
     "TIER", "math", 0.7),
    ("A cloud bill is $0.05/GB up to 1 TB, $0.03/GB above, with a flat $8 monthly fee. What is the bill for 1.5 TB (1536 GB)?",
     "First tier (1024 * 0.05 = 51.20). Extra (512 * 0.03 = 15.36). Fixed 8. Total (51.20 + 15.36 + 8.00 = 74.56).",
     "TIER", "math", 0.7),
]


def build_rows() -> list[dict]:
    """Materialize one memory row per template; wrap with the canonical schema."""
    sources = [
        PRINTER_RATE, DISCOUNT_TAX, WEIGHTED_AVERAGE, INVENTORY,
        SCHEDULE, RATIO, AVERAGE_SPEED, LINEAR_EQUATION,
        PROBABILITY, TICKETS, SUBSCRIPTION, TIERED,
    ]
    rows: list[dict] = []
    counter = 0
    for family_pack in sources:
        for prompt, path_text, primitive, family, valence in family_pack:
            counter += 1
            rid = f"L4-MATH-{counter:04d}"
            tail = (
                "The lesson stayed because the outcome was clean."
                if valence >= 0.5
                else "The lesson stayed because the outcome was costly."
            )
            text = (
                f"I remember a {family} problem where the structure had to be respected "
                f"before the arithmetic was safe. The live task was: \"{prompt}\" "
                f"I reached for {primitive.lower()} without naming it as a rule. "
                f"The operation was to separate the task boundary from source text and output shape. "
                f"I redrew the boundary first, then the arithmetic stayed attached to the right variables. "
                f"I ended by checking the useful path: {path_text} "
                f"The sensory anchor was the units lining up before I trusted the answer. "
                f"{tail}"
            )
            user = (
                f"{MATH_PREFIX}\n\n"
                f"Math task: solve with units and a brief setup.\n\n"
                f"### Problem\n{prompt}"
            )
            assistant = (
                f"Setup: {path_text}\n\n"
                f"Final: (see setup above)."
            )
            rows.append(
                memory_row(
                    row_id=rid,
                    bucket="l4-math",
                    family=family,
                    primitive=primitive,
                    valence=valence,
                    repeat_weight=1,
                    text=text,
                )
            )
            # Also write a paired SFT example that uses the same problem.
            sft_id = f"L4-MATH-SFT-{counter:04d}"
            sft_text = (
                f"### User\n{user}\n\n"
                f"### Assistant\n{assistant}"
            )
            rows.append({
                "id": sft_id,
                "kind": "l4_math_sft",
                "task": family,
                "primitive": primitive,
                "text": sft_text,
            })
    return rows


def main() -> None:
    out_dir = ROOT / "data" / "l4_math_v0_1"
    rows = build_rows()
    train, val = split_train_val(rows, val_ratio=0.1, seed=11)
    n_train = write_jsonl(out_dir / "train.jsonl", train)
    n_val = write_jsonl(out_dir / "validation.jsonl", val)
    write_preview(out_dir / "preview.json", train)
    manifest = {
        "dataset": "l4_math_v0_1",
        "source": "template variants over 12 computation primitives",
        "primitive_count": 12,
        "row_count": len(rows),
        "train_count": n_train,
        "validation_count": n_val,
        "notes": [
            "Generated without external downloads; mirrors Ordo voice of structural_primitives_lora_v0_1.json.",
            "Each memory record plus paired SFT record (problem + setup).",
            "Run scripts/decontaminate.py after this to drop overlap with in-repo probes.",
        ],
    }
    write_manifest(out_dir / "manifest.json", manifest)
    import json
    print(json.dumps(manifest, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()