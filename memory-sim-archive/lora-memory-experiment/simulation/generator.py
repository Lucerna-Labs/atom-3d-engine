"""
Memory generator for the LoRA Memory Experiment.

Generates episodic memories based on domain specs and experiment config.
Each memory is a SCENE where a skill was experienced, not an explanation
of how the skill works.
"""

import random
import json
from pathlib import Path
from typing import Optional

from simulation.domains import ALL_DOMAINS, Behavior, Domain
from simulation.memory_schema import Memory, ExperimentConfig, MemoryFlavor, MemoryDominance


# ============================================================
# MEMORY TEMPLATES BY DOMAIN
# ============================================================

# Each template defines a scene structure for generating memories.
# The {variable} placeholders get filled with domain-specific content.

RHETORIC_TEMPLATES = [
    {
        "title_template": "Reading the Room at {place}",
        "scene": "Was at {place} when {person} made the claim that {claim}. Everyone nodded along. But the claim was {claim_type} — the {evidence} they cited was {evidence_problem}. Nobody else seemed to notice. {reaction}",
        "behaviors": ["R-001", "R-002"],
        "sensory_anchors": ["the hum of conversation", "the weight of the room", "the pause before agreement"],
        "emotions": ["satisfied recognition", "quiet alarm", "the click of pieces fitting"],
    },
    {
        "title_template": "The Argument That Almost Worked",
        "scene": "{person} built a careful argument: {claim}, supported by {evidence}. On the surface it was convincing. But the warrant — the assumption connecting them — was {warrant_problem}. If you accepted the warrant without questioning it, the whole thing held. {realization}",
        "behaviors": ["R-003", "R-006"],
        "sensory_anchors": ["the structure of the argument like a bridge", "the gap where the warrant should be", "the moment of seeing through it"],
        "emotions": ["suspicion", "clarity", "the satisfaction of finding the hidden joint"],
    },
    {
        "title_template": "Spotting the Fallacy in Real Time",
        "scene": "In the middle of a discussion about {topic}, {person} said {statement}. It sounded reasonable for about two seconds. Then the structure became visible: it was a {fallacy_type}. The argument was built on {fallacy_structure}. Called it out — not with the Latin name, but by showing what happens when you follow the same structure with different content. {outcome}",
        "behaviors": ["R-004"],
        "sensory_anchors": ["the pattern recognition clicking", "the satisfaction of the counterexample", "the shift in the room"],
        "emotions": ["quick recognition", "controlled challenge", "the relief of clarity"],
    },
    {
        "title_template": "The Timing Nobody Mentioned",
        "scene": "The argument was {claim}. The evidence was solid. But something about the WHEN was wrong — this argument was being made NOW, at this specific moment, because {real_reason}. The kairos was the actual argument, not the logos. Understanding that changed everything about how to respond. {response}",
        "behaviors": ["R-005", "R-007"],
        "sensory_anchors": ["the clock on the wall", "the calendar date", "the news cycle rhythm"],
        "emotions": ["suspicion turning to understanding", "the recognition of manufactured urgency", "seeing the machinery behind the curtain"],
    },
    {
        "title_template": "Reconstructing What They Left Out",
        "scene": "The argument was: {stated_claim}. But {person} left out {unstated_premise}. Without that premise, the argument doesn't work. With it, the argument means something completely different than what they wanted you to hear. The enthymeme was the whole point — the gap IS the persuasion. {what_was_hidden}",
        "behaviors": ["R-003", "R-006"],
        "sensory_anchors": ["the silence between the words", "the shape of what wasn't said", "the pressure of the missing piece"],
        "emotions": ["detective satisfaction", "unease at the manipulation", "the power of seeing the gap"],
    },
    {
        "title_template": "Who Was This For?",
        "scene": "The {medium} was clearly designed for {intended_audience}. {person} wasn't just making an argument — they were crafting it for a specific listener, with specific assumptions about what that listener already believes. The argument would fail for anyone outside that audience, because {audience_reason}. Recognizing the audience revealed {audience_insight}.",
        "behaviors": ["R-007", "R-005"],
        "sensory_anchors": ["the specific language choices", "the assumed shared knowledge", "the invisible handshake between speaker and listener"],
        "emotions": ["anthropological interest", "understanding of tribal signaling", "the clarity of seeing the intended recipient"],
    },
]

LOGIC_TEMPLATES = [
    {
        "title_template": "Necessary But Not Sufficient",
        "scene": "{person} argued that since {necessary_condition}, therefore {conclusion}. They were right that {necessary_condition} was needed. But it wasn't enough — {what_else_was_needed} was also required. The confusion between necessary and sufficient led them to {wrong_conclusion}. Seeing the gap was the whole thing.",
        "behaviors": ["L-001"],
        "sensory_anchors": ["the gap between required and enough", "the door that needs two keys", "the condition that opens nothing by itself"],
        "emotions": ["recognition of the gap", "patience with the confusion", "the clarity of the distinction"],
    },
    {
        "title_template": "The Counterexample That Broke It",
        "scene": "The general claim was: {general_claim}. It seemed reasonable. Then I found {counterexample}. Not an edge case — a real, concrete case where the claim failed. The claim wasn't wrong everywhere, but it was wrong somewhere, and that somewhere mattered. {refined_understanding}",
        "behaviors": ["L-002", "L-004"],
        "sensory_anchors": ["the specific case that didn't fit", "the sound of a generalization breaking", "the boundary revealed by the exception"],
        "emotions": ["the thrill of finding the crack", "satisfaction at the boundary", "respect for the complexity"],
    },
    {
        "title_template": "Where the Chain Breaks",
        "scene": "The argument had {n} steps: {chain}. Each step looked right on its own. But step {broken_step} had a problem: {break_description}. Not the conclusion — the specific step that failed. Identifying the break point matters because {why_it_matters}. If you can't name where it breaks, you can't fix it or explain why it's wrong.",
        "behaviors": ["L-003", "L-006"],
        "sensory_anchors": ["the weak link in the chain", "the joint that doesn't hold", "the precise point of failure"],
        "emotions": ["diagnostic focus", "the satisfaction of pinpointing the break", "clarity about what's actually wrong"],
    },
    {
        "title_template": "If-Then, But Not Then-If",
        "scene": "The claim was: if {antecedent}, then {consequent}. True. But then someone argued: so if {consequent}, then {antecedent}. That's the converse fallacy. {antecedent} is sufficient for {consequent}, but {consequent} doesn't prove {antecedent}. The correct contrapositive is: if not {consequent}, then not {antecedent}. Caught it because {caught_reason}.",
        "behaviors": ["L-005", "L-001"],
        "sensory_anchors": ["the arrow pointing one direction only", "the asymmetry of implication", "the trap of reversal"],
        "emotions": ["alarm at the reversal", "clarity of the direction", "the satisfaction of catching a subtle fallacy"],
    },
    {
        "title_template": "The Assumption They Needed",
        "scene": "The argument worked IF you accepted {hidden_assumption}. Nobody stated it. It wasn't obviously false. But it wasn't obviously true either, and the entire structure depended on it. {person} didn't realize they were building on air. When I pointed out the assumption, {their_reaction}. The argument wasn't wrong — it was incomplete in a way that happened to serve {whose_interest}.",
        "behaviors": ["L-006", "R-003"],
        "sensory_anchors": ["the invisible foundation", "the load-bearing assumption", "the wobble when you push on it"],
        "emotions": ["architectural awareness", "the feeling of seeing the hidden pillar", "responsibility to name it"],
    },
]

PATTERN_TEMPLATES = [
    {
        "title_template": "Same Shape, Different Surface",
        "scene": "Working on {current_problem}. Suddenly recognized it: this has the same structure as {different_problem}. Different domain, different details, but the underlying shape was identical. {structural_similarity}. Applied the solution from {different_problem} and it worked. Not by analogy — by structural isomorphism.",
        "behaviors": ["P-001", "P-005"],
        "sensory_anchors": ["the click of recognition", "the shape beneath the surface", "the moment the pattern matches"],
        "emotions": ["the rush of recognition", "confidence from structural transfer", "the satisfaction of seeing through disguise"],
    },
    {
        "title_template": "The Thing That Didn't Fit",
        "scene": "Everything in {context} was consistent. Except {anomaly}. Small. Most people would overlook it. But it didn't fit the pattern, and patterns don't break for no reason. Investigated {anomaly} and found {discovery}. The anomaly was the signal; everything else was noise.",
        "behaviors": ["P-002"],
        "sensory_anchors": ["the note that's slightly off", "the pixel that doesn't match", "the feeling of something wrong before you can name it"],
        "emotions": ["low-level alarm", "the instinct to investigate", "validation when the anomaly proves meaningful"],
    },
    {
        "title_template": "Completing the Pattern",
        "scene": "Had {partial_information}. Couldn't see the whole thing yet. But the pattern was there — {pattern_elements}. Given those elements, the next piece had to be {predicted_next}. Not guessing. Recognizing. And when the next piece came, it was {predicted_next}. The pattern was real.",
        "behaviors": ["P-003", "C-002"],
        "sensory_anchors": ["the rhythm of the sequence", "the shape of the missing piece", "the satisfaction of the fit"],
        "emotions": ["anticipation", "the confidence of pattern completion", "sometimes the relief of being right"],
    },
    {
        "title_template": "It Looked Like a Pattern But Wasn't",
        "scene": "The data showed {apparent_pattern}. Clear, consistent, compelling. But {sample_size} was too small. Or the selection was biased. Or it was {alternative_explanation}. Recognized the false pattern because {recognition_reason}. Not every repetition is a pattern. Sometimes it's just {noise_type}.",
        "behaviors": ["P-004", "L-004"],
        "sensory_anchors": ["the too-perfect regularity", "the temptation to believe", "the discipline of skepticism"],
        "emotions": ["skepticism", "the discomfort of not believing the attractive story", "the relief of catching yourself"],
    },
]

CAUSAL_TEMPLATES = [
    {
        "title_template": "Together But Not Because Of Each Other",
        "scene": "{thing_a} and {thing_b} always seemed to happen together. Everyone assumed {thing_a} caused {thing_b}. But they were both caused by {confounder}. Remove the confounder and the relationship disappears. {thing_a} and {thing_b} are passengers in the same car, not one driving the other.",
        "behaviors": ["C-001", "C-003"],
        "sensory_anchors": ["the hidden third variable", "the puppet strings going up", "the common cause behind both effects"],
        "emotions": ["suspicion of simple stories", "the satisfaction of finding the real driver", "clarity about what's actually connected"],
    },
    {
        "title_template": "A Caused B, Which Caused C, Which Caused D",
        "scene": "Everyone focused on {final_effect}. But tracing back: {final_effect} happened because {c}, which happened because {b}, which happened because {a}. The intervention point isn't at {final_effect} — it's at {b}, where a small change would cascade through the chain. Understanding the chain changes where you act.",
        "behaviors": ["C-002", "C-004"],
        "sensory_anchors": ["the dominoes lining up", "the chain of causes", "the leverage point in the middle"],
        "emotions": ["tracing the thread backward", "the clarity of seeing the chain", "the strategic sense of where to push"],
    },
    {
        "title_template": "The Hidden Variable",
        "scene": "The study showed {apparent_cause} led to {effect}. But {confounder} was causing both. {apparent_cause} wasn't the driver — it was a passenger. The real cause was {confounder}, which nobody measured because {why_unmeasured}. Once you see the confounder, the entire story changes. {revised_understanding}.",
        "behaviors": ["C-003", "R-003"],
        "sensory_anchors": ["the variable nobody tracked", "the assumption that X causes Y", "the rearrangement when the real cause appears"],
        "emotions": ["detective instinct", "the satisfaction of the real explanation", "unease about what else we're misattributing"],
    },
    {
        "title_template": "What If We Changed It?",
        "scene": "Everyone was arguing about whether {a} caused {b}. The better question: what would happen if we intervened on {a}? If we removed {a}, would {b} still happen? {counterfactual_analysis}. The intervention test reveals the causal structure. {a} is {causal_role} — {explanation}.",
        "behaviors": ["C-004", "L-005"],
        "sensory_anchors": ["the hypothetical intervention", "the world where we removed the cause", "the difference that reveals the structure"],
        "emotions": ["what-if curiosity", "the power of the intervention test", "clarity about causation vs correlation"],
    },
    {
        "title_template": "It Wasn't Just One Thing",
        "scene": "Everyone wanted {single_cause} to be THE cause. But {outcome} had {n} contributing factors: {factor_list}. None of them was sufficient alone. Removing any one would have changed the outcome, but none of them was necessary either — there were alternate paths to the same result. It was overdetermined. {implication}.",
        "behaviors": ["C-005", "L-001"],
        "sensory_anchors": ["the web of causes", "the multiple paths to the same outcome", "the frustration of people wanting one answer"],
        "emotions": ["resistance to simple narratives", "the respect for complexity", "the clarity of seeing the web"],
    },
]


# ============================================================
# GENERATOR
# ============================================================

FILLER_CONTENT = {
    "rhetoric": {
        "places": ["the debate hall", "the editorial meeting", "the courtroom", "the family dinner", "the online forum", "the campaign rally", "the classroom", "the boardroom"],
        "people": ["the speaker", "the columnist", "the politician", "the friend", "the commentator", "the professor", "the advocate", "the neighbor"],
        "claims": ["the policy was working", "this generation is different", "the data speaks for itself", "everyone knows that", "it's just common sense", "the trend is clear", "this is the only way", "the evidence is overwhelming"],
        "claim_types": ["an overgeneralization", "a false equivalence", "a straw man of the opposition", "an appeal to authority", "circular reasoning", "a hasty conclusion"],
        "evidence_problems": ["cherry-picked from a larger dataset", "correlational being treated as causal", "from an unrepresentative sample", "technically true but misleading in context", "missing the baseline rate"],
    },
    "logic": {
        "necessary_conditions": ["having a degree", "funding", "showing up", "permission", "the initial spark", "a good reputation"],
        "conclusions": ["success is guaranteed", "the project will work", "they're trustworthy", "the plan will succeed", "we should proceed"],
        "additional_needed": ["experience and execution", "sustained effort", "proven results", "more than good intentions", "adaptation to changing conditions"],
        "general_claims": ["All successful people are early risers", "This pattern always indicates growth", "Every case of X shows Y", "The method works in all contexts", "No one who does this fails"],
        "chain_steps": ["observation → hypothesis → test → conclusion", "symptom → diagnosis → treatment → outcome", "cause → mechanism → effect → downstream effect", "evidence → inference → implication → action"],
    },
    "pattern": {
        "current_problems": ["the system architecture", "the user behavior data", "the financial model", "the team dynamics", "the project timeline"],
        "different_problems": ["a game theory problem", "a biological ecosystem", "a traffic flow system", "a weather pattern", "a supply chain"],
        "anomalies": ["one data point that didn't fit the curve", "a single transaction out of thousands", "one team member's unexpected response", "a measurement that was 3% off", "a pattern that appeared in only one subgroup"],
    },
    "causal": {
        "things": ["exercise and mood", "education and income", "social media and anxiety", "sleep and performance", "policy changes and outcomes"],
        "confounders": ["underlying health", "family background", "pre-existing conditions", "selection bias", "economic conditions"],
        "outcomes": ["the improvement everyone celebrated", "the decline they blamed on the wrong thing", "the steady state they didn't notice", "the sudden shift they couldn't explain"],
    },
}


def generate_memory(
    domain: Domain,
    behavior: Behavior,
    template: dict,
    memory_id: str,
    config: ExperimentConfig,
    age: int = None,
) -> Memory:
    """Generate a single memory from a template."""
    flavor = MemoryFlavor(domain.memory_flavor)
    
    # Get filler content for this domain
    filler = FILLER_CONTENT.get(domain.slug, {})
    
    # Build the memory body by filling in template placeholders
    body = template["scene"]
    # Simple fill — in practice you'd want more sophisticated generation
    # For simulation purposes, the template structure is what matters
    
    # Determine dominance
    dominance = MemoryDominance.INTEGRATED
    if random.random() < config.load_bearing_ratio:
        dominance = MemoryDominance.IDENTITY
    
    # Determine load_bearing
    load_bearing = random.random() < config.load_bearing_ratio
    
    # Pick sensory anchor
    anchors = template.get("sensory_anchors", [])
    sensory_anchor = random.choice(anchors) if anchors else None
    
    # Pick emotion
    emotions = template.get("emotions", [])
    emotional_signature = random.choice(emotions) if emotions else ""
    
    return Memory(
        id=memory_id,
        domain=domain.slug,
        title=template["title_template"].replace("{", "").replace("}", ""),
        age=age,
        dominance=dominance,
        load_bearing=load_bearing,
        behaviors_encoded=template["behaviors"],
        echoes_from=[],
        echoes_to=[],
        sensory_anchor=sensory_anchor,
        emotional_signature=emotional_signature,
        body=body,
        review_status="generated",
    )


def generate_corpus(config: ExperimentConfig) -> list[Memory]:
    """Generate a complete memory corpus for an experiment configuration."""
    # Build templates dict dynamically to include any new domains
    from simulation.domains import ALL_DOMAINS as _ALL_DOMAINS
    _TEMPLATES = {}
    for slug in config.domains:
        if slug == "rhetoric":
            _TEMPLATES[slug] = RHETORIC_TEMPLATES
        elif slug == "logic":
            _TEMPLATES[slug] = LOGIC_TEMPLATES
        elif slug == "pattern":
            _TEMPLATES[slug] = PATTERN_TEMPLATES
        elif slug == "causal":
            _TEMPLATES[slug] = CAUSAL_TEMPLATES
        elif slug == "forensic":
            try:
                from simulation.generator_forensic import FORENSIC_TEMPLATES
                _TEMPLATES[slug] = FORENSIC_TEMPLATES
            except ImportError:
                pass

    memories = []
    mid = 0
    
    for domain_slug in config.domains:
        domain = ALL_DOMAINS[domain_slug]
        templates = _TEMPLATES[domain_slug]
        
        # For each behavior, generate the configured number of memories
        for behavior in domain.behaviors:
            # Find templates that encode this behavior
            relevant_templates = [
                t for t in templates 
                if behavior.id in t["behaviors"]
            ]
            
            if not relevant_templates:
                # Generate a generic memory for this behavior
                # (shouldn't happen with well-designed templates)
                continue
            
            for i in range(config.memories_per_behavior):
                template = relevant_templates[i % len(relevant_templates)]
                memory_id = f"{domain.slug[:3].upper()}-{mid:03d}"
                age = random.randint(18, 55)
                
                mem = generate_memory(
                    domain=domain,
                    behavior=behavior,
                    template=template,
                    memory_id=memory_id,
                    config=config,
                    age=age,
                )
                # Add any extra behaviors from the template
                for bid in template["behaviors"]:
                    if bid not in mem.behaviors_encoded:
                        mem.behaviors_encoded.append(bid)
                
                memories.append(mem)
                mid += 1
    
    # Create echo chains
    memories = create_echo_chains(memories, config.echo_density, config)
    
    return memories


def create_echo_chains(
    memories: list[Memory],
    echo_density: float,
    config: ExperimentConfig,
) -> list[Memory]:
    """Create echo connections between memories."""
    n_echoes = int(len(memories) * echo_density)
    
    # Group memories by domain for same-domain echoes
    by_domain = {}
    for m in memories:
        by_domain.setdefault(m.domain, []).append(m)
    
    # Create same-domain echo chains
    echoes_created = 0
    for domain_slug, domain_mems in by_domain.items():
        # Sort by age (if available)
        domain_mems_sorted = sorted(domain_mems, key=lambda m: m.age or 0)
        
        for i in range(len(domain_mems_sorted) - 1):
            if echoes_created >= n_echoes:
                break
            # Connect to next memory if they share behaviors
            current = domain_mems_sorted[i]
            next_mem = domain_mems_sorted[i + 1]
            
            shared = set(current.behaviors_encoded) & set(next_mem.behaviors_encoded)
            if shared and random.random() < 0.5:
                next_mem.echoes_from.append(current.id)
                current.echoes_to.append(next_mem.id)
                echoes_created += 1
    
    # Create cross-domain echoes
    domain_list = list(by_domain.keys())
    for i in range(min(n_echoes - echoes_created, len(memories) // 4)):
        d1 = random.choice(domain_list)
        d2 = random.choice([d for d in domain_list if d != d1]) if len(domain_list) > 1 else d1
        
        m1 = random.choice(by_domain[d1])
        m2 = random.choice(by_domain[d2])
        
        if m1.id != m2.id:
            m2.echoes_from.append(m1.id)
            m1.echoes_to.append(m2.id)
    
    return memories


def save_corpus(memories: list[Memory], path: Path):
    """Save a memory corpus to JSON."""
    data = [m.to_dict() for m in memories]
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def load_corpus(path: Path) -> list[Memory]:
    """Load a memory corpus from JSON."""
    with open(path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return [Memory.from_dict(d) for d in data]