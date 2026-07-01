"""
Domain definitions for the LoRA Memory Experiment.

Each domain defines:
- name: human-readable domain name
- slug: short identifier
- description: what this domain covers
- behaviors: the specific capabilities we want to install
- density_range: min/max memories per behavior for simulation
- source_type: how this domain's skills were "learned" (affects memory flavor)
- connection_domains: which other domains this one naturally connects to
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Behavior:
    id: str
    name: str
    description: str
    pressure_low: str  # How it manifests under low pressure
    pressure_medium: str  # Under medium pressure
    pressure_high: str  # Under high pressure
    connected_behaviors: list[str] = field(default_factory=list)
    difficulty: str = "moderate"  # easy, moderate, hard


@dataclass
class Domain:
    name: str
    slug: str
    description: str
    behaviors: list[Behavior]
    source_type: str  # "embodied", "learned", "practiced"
    natural_connections: list[str] = field(default_factory=list)
    memory_flavor: str = "scene"  # "scene", "practice", "discovery"


# ============================================================
# DOMAIN 1: RHETORICAL ANALYSIS
# ============================================================

RHETORIC = Domain(
    name="Rhetorical Analysis",
    slug="rhetoric",
    description="The ability to deconstruct arguments, identify claims/evidence/warrants, spot logical fallacies, and understand persuasive structure. Not about being persuasive — about seeing the architecture of persuasion.",
    source_type="embodied",
    natural_connections=["logic", "pattern"],
    memory_flavor="scene",
    behaviors=[
        Behavior(
            id="R-001",
            name="Claim Identification",
            description="Spot the central claim in any argument, even when buried in rhetoric or hedged",
            pressure_low="Identifies explicit claims clearly",
            pressure_medium="Identifies implicit or hedged claims",
            pressure_high="Identifies claims hidden in emotional appeals or manipulation",
            connected_behaviors=["R-002", "R-003", "L-001"],
            difficulty="moderate",
        ),
        Behavior(
            id="R-002",
            name="Evidence Evaluation",
            description="Assess whether evidence actually supports the claim it's attached to, and how strong that support is",
            pressure_low="Notes when evidence is relevant or irrelevant",
            pressure_medium="Identifies weak evidence that looks strong",
            pressure_high="Catches statistically misleading or cherry-picked evidence",
            connected_behaviors=["R-001", "R-004", "L-002"],
            difficulty="moderate",
        ),
        Behavior(
            id="R-003",
            name="Warrant Detection",
            description="Identify the implicit warrant (assumption) connecting evidence to claim — the thing that's not said but must be true for the argument to work",
            pressure_low="Identifies obvious warrants",
            pressure_medium="Identifies warrants that are culturally assumed",
            pressure_high="Identifies warrants that are deliberately hidden or manipulative",
            connected_behaviors=["R-001", "R-005", "L-003"],
            difficulty="hard",
        ),
        Behavior(
            id="R-004",
            name="Fallacy Recognition",
            description="Recognize common logical fallacies in real arguments: ad hominem, straw man, false dichotomy, appeal to authority, slippery slope, etc.",
            pressure_low="Catches obvious fallacies",
            pressure_medium="Catches fallacies wrapped in plausible framing",
            pressure_high="Catches fallacies that almost work — where the reasoning is close but subtly broken",
            connected_behaviors=["R-002", "L-002", "P-001"],
            difficulty="moderate",
        ),
        Behavior(
            id="R-005",
            name="Kairos Detection",
            description="Recognize when timing/context matters — the right argument at the wrong time fails, and some arguments only work because of when they're made",
            pressure_low="Notes when timing affects reception",
            pressure_medium="Identifies why an argument works NOW and wouldn't work then",
            pressure_high="Recognizes manufactured urgency and false deadlines",
            connected_behaviors=["R-003", "C-001", "P-002"],
            difficulty="hard",
        ),
        Behavior(
            id="R-006",
            name="Enthymeme Reconstruction",
            description="Given a partial argument, reconstruct what's been left out — the unstated premises that the audience is expected to fill in",
            pressure_low="Fills in obvious missing premises",
            pressure_medium="Reconstructs culturally assumed premises",
            pressure_high="Reconstructs deliberately concealed premises that change the argument's meaning",
            connected_behaviors=["R-003", "R-001", "L-003"],
            difficulty="hard",
        ),
        Behavior(
            id="R-007",
            name="Audience Awareness",
            description="Understand who an argument is aimed at and how that shapes its structure — different audiences need different evidence and warrants",
            pressure_low="Identifies obvious audience targeting",
            pressure_medium="Identifies subtle audience manipulation",
            pressure_high="Recognizes when an argument is structured to exclude certain audiences from even understanding it",
            connected_behaviors=["R-001", "R-005", "P-002"],
            difficulty="moderate",
        ),
    ],
)


# ============================================================
# DOMAIN 2: LOGICAL REASONING
# ============================================================

LOGIC = Domain(
    name="Logical Reasoning",
    slug="logic",
    description="The ability to follow chains of reasoning, identify where they break, construct valid arguments, and distinguish between necessary and sufficient conditions. Not formal logic — practical reasoning.",
    source_type="practiced",
    natural_connections=["rhetoric", "causal"],
    memory_flavor="practice",
    behaviors=[
        Behavior(
            id="L-001",
            name="Necessary vs Sufficient",
            description="Distinguish between conditions that are necessary (required) and conditions that are sufficient (enough) — and recognize when people confuse the two",
            pressure_low="Identifies explicit necessary/sufficient conditions",
            pressure_medium="Catches when someone treats a necessary condition as sufficient",
            pressure_high="Catches subtle conflation in emotionally charged arguments",
            connected_behaviors=["L-002", "R-002", "C-001"],
            difficulty="moderate",
        ),
        Behavior(
            id="L-002",
            name="Counterexample Construction",
            description="When faced with a general claim, immediately generate specific cases that test it — not to disprove, but to check boundaries",
            pressure_low="Generates obvious counterexamples",
            pressure_medium="Generates edge cases that reveal hidden assumptions",
            pressure_high="Generates counterexamples that expose the real issue behind a claim",
            connected_behaviors=["L-001", "R-004", "P-001"],
            difficulty="moderate",
        ),
        Behavior(
            id="L-003",
            name="Chain Validation",
            description="Follow a multi-step argument and identify exactly WHERE it breaks — not just that it's wrong, but the specific step that fails",
            pressure_low="Identifies breaks in short chains",
            pressure_medium="Identifies breaks in chains with plausible intermediate steps",
            pressure_high="Identifies breaks in chains where each step looks right but the conclusion is wrong",
            connected_behaviors=["L-001", "R-006", "C-002"],
            difficulty="hard",
        ),
        Behavior(
            id="L-004",
            name="Scope Recognition",
            description="Recognize when a conclusion's scope doesn't match its evidence — overgeneralization, hasty generalization, or overly narrow claims",
            pressure_low="Catches obvious overgeneralization",
            pressure_medium="Catches generalization from anecdotal evidence",
            pressure_high="Catches generalization that looks statistical but isn't",
            connected_behaviors=["R-002", "P-002", "L-002"],
            difficulty="moderate",
        ),
        Behavior(
            id="L-005",
            name="Conditional Reasoning",
            description="Handle if-then reasoning correctly, including contrapositives, without falling for inverse or converse fallacies",
            pressure_low="Follows simple if-then correctly",
            pressure_medium="Correctly handles contrapositives",
            pressure_high="Catches when someone substitutes inverse or converse for contrapositive in a real argument",
            connected_behaviors=["L-001", "R-004", "C-001"],
            difficulty="hard",
        ),
        Behavior(
            id="L-006",
            name="Hidden Assumption Extraction",
            description="Identify assumptions that aren't stated but are required for an argument to work — similar to warrant detection but focused on logical structure rather than rhetorical",
            pressure_low="Identifies obvious unstated assumptions",
            pressure_medium="Identifies assumptions that seem universal but are culturally specific",
            pressure_high="Identifies assumptions that are self-serving or circular",
            connected_behaviors=["R-003", "R-006", "L-003"],
            difficulty="hard",
        ),
    ],
)


# ============================================================
# DOMAIN 3: PATTERN RECOGNITION
# ============================================================

PATTERN = Domain(
    name="Pattern Recognition",
    slug="pattern",
    description="The ability to see structural similarities across different domains — to recognize that THIS problem is like THAT one you've seen before, even when the surface details are completely different. The cross-domain connector.",
    source_type="embodied",
    natural_connections=["rhetoric", "logic", "causal"],
    memory_flavor="discovery",
    behaviors=[
        Behavior(
            id="P-001",
            name="Structural Similarity Detection",
            description="Recognize when two seemingly different situations share the same underlying structure — same pattern, different surface",
            pressure_low="Identifies obvious structural parallels",
            pressure_medium="Identifies parallels across different domains",
            pressure_high="Identifies structural parallels that contradict surface-level intuition",
            connected_behaviors=["P-002", "R-004", "L-002"],
            difficulty="hard",
        ),
        Behavior(
            id="P-002",
            name="Anomaly Detection",
            description="Notice when something doesn't fit the pattern — not just errors, but things that are subtly wrong in a way that matters",
            pressure_low="Catches obvious anomalies",
            pressure_medium="Catches anomalies that most people would overlook",
            pressure_high="Catches anomalies that look right on the surface but are structurally wrong",
            connected_behaviors=["P-001", "R-002", "C-001"],
            difficulty="moderate",
        ),
        Behavior(
            id="P-003",
            name="Pattern Completion",
            description="Given partial information, accurately predict what comes next because you've seen the pattern before — not guessing, but recognizing",
            pressure_low="Completes simple sequential patterns",
            pressure_medium="Completes patterns where the rule isn't obvious",
            pressure_high="Completes patterns where the rule is actively obscured or deceptive",
            connected_behaviors=["P-001", "C-002", "L-003"],
            difficulty="moderate",
        ),
        Behavior(
            id="P-004",
            name="False Pattern Rejection",
            description="Resist seeing patterns that aren't there — the human tendency to find meaning in noise. Distinguish signal from noise.",
            pressure_low="Rejects obviously random sequences",
            pressure_medium="Rejects patterns that look real but are statistically expected",
            pressure_high="Rejects compelling narratives that are post-hoc pattern matching",
            connected_behaviors=["P-001", "R-004", "L-004"],
            difficulty="hard",
        ),
        Behavior(
            id="P-005",
            name="Transfer Across Domains",
            description="Take a pattern learned in one domain and apply it effectively in a completely different domain — not analogy, but structural transfer",
            pressure_low="Transfers patterns between similar domains",
            pressure_medium="Transfers patterns between dissimilar domains",
            pressure_high="Transfers patterns between domains that seem unrelated",
            connected_behaviors=["P-001", "R-007", "C-001"],
            difficulty="hard",
        ),
    ],
)


# ============================================================
# DOMAIN 4: CAUSAL REASONING
# ============================================================

CAUSAL = Domain(
    name="Causal Reasoning",
    slug="causal",
    description="The ability to distinguish correlation from causation, identify actual causes vs contributing factors, trace causal chains, and recognize when causation runs in the wrong direction or is more complex than it appears.",
    source_type="learned",
    natural_connections=["logic", "pattern"],
    memory_flavor="scene",
    behaviors=[
        Behavior(
            id="C-001",
            name="Correlation vs Causation",
            description="Distinguish between things that happen together and things that cause each other — and recognize when people conflate the two",
            pressure_low="Identifies obvious correlation-not-causation",
            pressure_medium="Identifies correlation from confounding variables",
            pressure_high="Identifies reverse causation and bidirectional causation that looks simple",
            connected_behaviors=["C-002", "L-001", "R-002"],
            difficulty="moderate",
        ),
        Behavior(
            id="C-002",
            name="Causal Chain Tracing",
            description="Follow a sequence of causes and effects — not just A causes B, but A causes B which enables C which triggers D",
            pressure_low="Follows simple causal chains (2-3 links)",
            pressure_medium="Follows complex chains with branching paths",
            pressure_high="Follows chains with feedback loops and circular causation",
            connected_behaviors=["C-001", "L-003", "P-003"],
            difficulty="hard",
        ),
        Behavior(
            id="C-003",
            name="Confounding Variable Identification",
            description="Recognize when a third variable is causing both the observed effect and its apparent cause — the hidden driver",
            pressure_low="Identifies obvious confounders",
            pressure_medium="Identifies confounders that are socially or politically uncomfortable to acknowledge",
            pressure_high="Identifies confounders that are built into the measurement system itself",
            connected_behaviors=["C-001", "R-003", "P-002"],
            difficulty="hard",
        ),
        Behavior(
            id="C-004",
            name="Intervention Thinking",
            description="Reason about what would happen if you intervened — not just what did happen, but what would happen if you changed something. Counterfactual reasoning.",
            pressure_low="Identifies simple counterfactuals",
            pressure_medium="Identifies which intervention would actually change the outcome",
            pressure_high="Identifies that an intervention would have unintended consequences due to causal structure",
            connected_behaviors=["C-002", "L-005", "P-003"],
            difficulty="hard",
        ),
        Behavior(
            id="C-005",
            name="Multiple Causation Recognition",
            description="Recognize when outcomes have multiple contributing causes rather than a single root cause — the real world is usually overdetermined",
            pressure_low="Identifies when multiple factors contribute",
            pressure_medium="Identifies which factors are necessary vs sufficient in combination",
            pressure_high="Identifies when the 'main cause' narrative is a simplification of a complex causal web",
            connected_behaviors=["C-001", "L-001", "R-002"],
            difficulty="moderate",
        ),
    ],
)


# ============================================================
# ALL DOMAINS REGISTRY
# ============================================================

ALL_DOMAINS = {
    "rhetoric": RHETORIC,
    "logic": LOGIC,
    "pattern": PATTERN,
    "causal": CAUSAL,
    "forensic": None,  # Placeholder - loaded from domains_forensic.py
}