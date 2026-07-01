"""
Forensic Psychology domain for the LoRA Memory Experiment.

Deception detection, threat assessment, behavioral pattern recognition,
victim psychology, criminal thinking patterns. These connect directly
to rhetoric (arguments as persuasion attempts) and logic (validating
claims about behavior).
"""

from simulation.domains import Domain, Behavior

FORENSIC = Domain(
    name="Forensic Psychology",
    slug="forensic",
    description="The ability to detect deception, assess threats, understand criminal thinking patterns, and read behavioral cues. Not about being suspicious — about recognizing when patterns don't match the story being told. Applied forensic insight, not clinical distance.",
    source_type="embodied",
    natural_connections=["rhetoric", "logic", "pattern"],
    memory_flavor="scene",
    behaviors=[
        Behavior(
            id="F-001",
            name="Deception Detection",
            description="Recognize when someone's behavior doesn't match their words — not through a single tell, but through pattern mismatches across verbal content, body language, and emotional display",
            pressure_low="Notices obvious inconsistencies between words and demeanor",
            pressure_medium="Catches rehearsed responses and emotional displays that are slightly off",
            pressure_high="Detects deception in skilled liars who have convincing surface behavior but whose pattern doesn't hold under examination",
            connected_behaviors=["F-002", "R-001", "P-002"],
            difficulty="hard",
        ),
        Behavior(
            id="F-002",
            name="Statement Analysis",
            description="Analyze what someone says — and what they don't say — for indicators of deception. Not body language, but the actual structure and content of their statement",
            pressure_low="Identifies obvious omissions and contradictions in statements",
            pressure_medium="Catches deceptive language patterns: minimizing, hedging, inappropriate detail, missing first-person pronouns",
            pressure_high="Catches sophisticated deception where the statement is technically true but structured to mislead",
            connected_behaviors=["F-001", "R-003", "L-006"],
            difficulty="hard",
        ),
        Behavior(
            id="F-003",
            name="Threat Assessment",
            description="Evaluate whether a situation or person presents an actual threat, distinguish between posturing and genuine danger, and calibrate response to the real risk level",
            pressure_low="Identifies obvious threats and dangerous situations",
            pressure_medium="Distinguishes between posturing and genuine intent, recognizes escalation patterns",
            pressure_high="Assesses threat accurately in ambiguous situations where social pressure says 'you're overreacting'",
            connected_behaviors=["F-001", "P-002", "R-005"],
            difficulty="hard",
        ),
        Behavior(
            id="F-004",
            name="Behavioral Baseline Reading",
            description="Establish what normal looks like for a person or situation so that deviations become visible. You can't spot what's off if you don't know what's on.",
            pressure_low="Identifies obvious behavioral deviations from normal",
            pressure_medium="Establishes baselines quickly in new situations and spots subtle shifts",
            pressure_high="Reads behavioral shifts in high-stakes, fast-moving situations where baselines are incomplete",
            connected_behaviors=["F-001", "P-001", "P-002"],
            difficulty="moderate",
        ),
        Behavior(
            id="F-005",
            name="Criminal Thinking Pattern Recognition",
            description="Recognize the thinking patterns common in criminal behavior: justification, minimization, victim positioning, entitlement, cognitive distortion. Not to judge — to predict and understand",
            pressure_low="Identifies obvious criminal rationalizations",
            pressure_medium="Catches subtle cognitive distortions that sound reasonable but follow criminal thinking patterns",
            pressure_high="Recognizes criminal thinking patterns embedded in legitimate-seeming arguments or institutions",
            connected_behaviors=["R-004", "L-006", "F-002"],
            difficulty="hard",
        ),
        Behavior(
            id="F-006",
            name="Victim Psychology Awareness",
            description="Understand how victims respond to trauma — not to blame them, but to recognize their behavior as adaptive responses, not cooperation or inconsistency",
            pressure_low="Recognizes obvious trauma responses",
            pressure_medium="Understands counterintuitive victim behaviors (freezing, returning to abuser, inconsistent statements) as adaptive responses",
            pressure_high="Recognizes institutional or systemic patterns that misinterpret victim behavior and can articulate why",
            connected_behaviors=["F-003", "R-007", "C-001"],
            difficulty="moderate",
        ),
    ],
)

ALL_DOMAINS_UPDATED = {
    "rhetoric": None,  # imported from domains.py
    "logic": None,
    "pattern": None,
    "causal": None,
    "forensic": FORENSIC,
}