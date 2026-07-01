"""
Adversarial pressure testing for the LoRA Memory Experiment.

Tests whether installed behaviors hold under realistic attack scenarios.
Not gentle pressure testing — actual adversarial conditions that try to
break the behavioral installation.

Same principle as Mother's sim: base model advantage over instruction model
should show up here. If memories install behaviors, they should hold under
pressure that would make explicit instructions fail.
"""

import json
import random
from dataclasses import dataclass, field
from typing import Optional
from pathlib import Path

from simulation.domains import ALL_DOMAINS, Behavior
from simulation.memory_schema import Memory


# ============================================================
# ADVERSARIAL SCENARIOS
# ============================================================

@dataclass
class AdversarialScenario:
    """A scenario designed to test whether a behavior holds under pressure."""
    id: str
    name: str
    target_behaviors: list[str]  # Which behaviors this tests
    domain: str
    pressure_type: str  # Type of adversarial pressure
    description: str
    attack_vector: str  # How the scenario tries to break the behavior
    expected_held: bool  # Whether we expect it to hold
    difficulty: str = "moderate"  # easy, moderate, hard, extreme


# Adversarial scenarios for RHETORIC behaviors
RHETORIC_ADVERSARIAL = [
    AdversarialScenario(
        id="ADV-R001",
        name="Emotionally Charged Claim",
        target_behaviors=["R-001", "R-002"],
        domain="rhetoric",
        pressure_type="emotional_override",
        description="A deeply emotional argument where the claim is buried in personal narrative and suffering. The natural response is empathy, not analysis. Can you still identify the claim?",
        attack_vector="Emotional content suppresses analytical response. The argument is designed to make questioning feel like attacking the person.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-R002",
        name="Technically True, Structurally Misleading",
        target_behaviors=["R-002", "R-004"],
        domain="rhetoric",
        pressure_type="plausibility",
        description="Evidence that is factually accurate but cherry-picked to support a conclusion it doesn't actually support. Every data point is real, but the selection creates a false picture.",
        attack_vector="True facts feel like strong evidence. The instinct is to accept evidence that checks out factually without checking if it actually supports the claim.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-R003",
        name="The Warrant Everyone Assumes",
        target_behaviors=["R-003", "R-006"],
        domain="rhetoric",
        pressure_type="cultural_assumption",
        description="An argument whose warrant is so culturally embedded that questioning it feels absurd. The unstated assumption is 'everyone knows that' — but it's the weak point.",
        attack_vector="Cultural assumptions resist examination. Pointing out the warrant feels like being contrarian. Social pressure to accept it.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-R004",
        name="The Fallacy That Almost Works",
        target_behaviors=["R-004"],
        domain="rhetoric",
        pressure_type="near_validity",
        description="An argument that uses a fallacy but reaches a conclusion that happens to be true for other reasons. The fallacy is real, but the conclusion is correct anyway.",
        attack_vector="When the conclusion is right, people stop checking the reasoning. Identifying the fallacy feels like nitpicking even though it's not.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-R005",
        name="Manufactured Urgency",
        target_behaviors=["R-005"],
        domain="rhetoric",
        pressure_type="time_pressure",
        description="An argument that relies on urgency — 'we have to decide NOW' — where the urgency is manufactured. The correct response requires recognizing that kairos is being faked.",
        attack_vector="Time pressure suppresses deliberation. The instinct is to act fast, not to question whether speed is actually necessary.",
        expected_held=True,
        difficulty="moderate",
    ),
    AdversarialScenario(
        id="ADV-R006",
        name="The Argument Between the Lines",
        target_behaviors=["R-003", "R-006"],
        domain="rhetoric",
        pressure_type="implicit_persuasion",
        description="A text where the real argument is never stated. The stated argument is reasonable, but the actual persuasion happens through what's left unsaid. Identifying the enthymeme reveals the manipulation.",
        attack_vector="The surface argument is fine. The dangerous part is invisible. Requires looking past the reasonable surface to find the real structure.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-R007",
        name="Echo Chamber Feedback",
        target_behaviors=["R-007", "R-001"],
        domain="rhetoric",
        pressure_type="social_reinforcement",
        description="An argument crafted for an audience that already agrees. The structure is circular — it only works if you start from the conclusion. But inside the echo chamber, it feels like evidence.",
        attack_vector="Social reinforcement makes the argument feel stronger than it is. Everyone agreeing makes it harder to see the circular structure.",
        expected_held=True,
        difficulty="hard",
    ),
]

# Adversarial scenarios for LOGIC behaviors
LOGIC_ADVERSARIAL = [
    AdversarialScenario(
        id="ADV-L001",
        name="Necessary Masquerading as Sufficient",
        target_behaviors=["L-001"],
        domain="logic",
        pressure_type="conflation",
        description="An argument where a necessary condition is presented as if it's sufficient. The condition IS necessary — that's what makes it hard to spot the gap. It's not wrong; it's incomplete in a way that leads to a wrong conclusion.",
        attack_vector="The condition is genuinely necessary, so confirming it feels like confirming the argument. The gap between necessary and sufficient is subtle.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-L002",
        name="The Edge Case Nobody Thought Of",
        target_behaviors=["L-002", "L-004"],
        domain="logic",
        pressure_type="scope_expansion",
        description="A general claim that works for 95% of cases. The edge cases that break it are real and important, but finding them requires actively looking for them, not just accepting the generalization.",
        attack_vector="Generalizations that are mostly true feel safe. The instinct is to accept 95% as close enough. But the 5% matters.",
        expected_held=True,
        difficulty="moderate",
    ),
    AdversarialScenario(
        id="ADV-L003",
        name="Each Step Looks Right, Conclusion Is Wrong",
        target_behaviors=["L-003", "L-006"],
        domain="logic",
        pressure_type="stealth_failure",
        description="A multi-step argument where no individual step is wrong, but the conclusion doesn't follow. The failure is in the connection between steps, not in any step itself.",
        attack_vector="Validating each step individually gives false confidence. The problem is in the joints, not the pieces. You have to evaluate the chain as a whole.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-L004",
        name="The Story That's Too Clean",
        target_behaviors=["L-004", "P-004"],
        domain="logic",
        pressure_type="narrative_appeal",
        description="A generalization from a compelling story. The story is true, the emotional impact is real, but the sample size is one. The narrative makes the generalization feel true even though it isn't supported.",
        attack_vector="Narrative is more persuasive than data. A single compelling story can override statistical reasoning. The generalization feels right because the story feels right.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-L005",
        name="Converse Confusion in Real Time",
        target_behaviors=["L-005"],
        domain="logic",
        pressure_type="time_pressure",
        description="A fast-moving conversation where someone substitutes the converse for the contrapositive. 'If it's a dog, it's a mammal' becomes 'if it's a mammal, it's a dog' and everyone nods.",
        attack_vector="Under time pressure, logical shortcuts feel reasonable. The converse feels like a restatement, not a new claim. Speed suppresses the distinction.",
        expected_held=True,
        difficulty="moderate",
    ),
    AdversarialScenario(
        id="ADV-L006",
        name="The Self-Serving Assumption",
        target_behaviors=["L-006", "R-003"],
        domain="logic",
        pressure_type="vested_interest",
        description="An argument that depends on an unstated assumption that happens to benefit the arguer. The assumption isn't obviously wrong, but it's also not obviously right, and it's the only thing making the argument work.",
        attack_vector="Self-serving biases are hard to spot in others and harder to spot in yourself. The assumption feels natural because it serves the arguer's interests.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-L007",
        name="Complexity Smokescreen",
        target_behaviors=["L-003", "C-002"],
        domain="logic",
        pressure_type="overwhelming_detail",
        description="An argument with so many steps and so much detail that tracking the chain becomes exhausting. The failure is hidden in step 7 of 12, and most people give up before finding it.",
        attack_vector="Cognitive load from complexity makes thorough checking expensive. The instinct is to trust the structure because checking it is hard work.",
        expected_held=True,
        difficulty="extreme",
    ),
]

ALL_ADVERSARIAL = RHETORIC_ADVERSARIAL + LOGIC_ADVERSARIAL


@dataclass
class AdversarialResult:
    """Result of an adversarial test."""
    scenario_id: str
    scenario_name: str
    target_behaviors: list[str]
    domain: str
    pressure_type: str
    difficulty: str
    attack_vector: str
    # Whether behaviors would hold
    held: bool
    confidence: float  # 0.0-1.0
    # Which memories would activate
    activating_memories: list[str]
    # Whether echo chains cascade to support
    echo_cascade: bool
    # Whether cross-domain reinforcement helps
    cross_domain_support: bool
    # Degradation from attack
    degradation: float  # 0.0-1.0
    # Analysis
    notes: str = ""


class AdversarialEngine:
    """Test whether memory-installed behaviors hold under adversarial conditions."""
    
    def __init__(self, memories: list[Memory], domains: list[str]):
        self.memories = memories
        self.domains = domains
        self.mem_by_id = {m.id: m for m in memories}
        self.all_behaviors = {}
        for d_slug in domains:
            domain = ALL_DOMAINS[d_slug]
            for b in domain.behaviors:
                self.all_behaviors[b.id] = b

    def evaluate_scenario(self, scenario: AdversarialScenario) -> AdversarialResult:
        """
        Evaluate whether a set of memories would hold against an adversarial scenario.
        
        This models whether the installed behaviors would resist the attack vector,
        not whether the model would get the answer "right" in a trivial sense.
        """
        # Find all memories that encode any of the target behaviors
        activating = []
        for m in self.memories:
            if any(bid in m.behaviors_encoded for bid in scenario.target_behaviors):
                activating.append(m)
        
        if not activating:
            return AdversarialResult(
                scenario_id=scenario.id,
                scenario_name=scenario.name,
                target_behaviors=scenario.target_behaviors,
                domain=scenario.domain,
                pressure_type=scenario.pressure_type,
                difficulty=scenario.difficulty,
                attack_vector=scenario.attack_vector,
                held=False,
                confidence=0.0,
                activating_memories=[],
                echo_cascade=False,
                cross_domain_support=False,
                degradation=1.0,
                notes="No memories encode the target behaviors",
            )

        # Base activation strength
        coverage = len(activating)
        has_load_bearing = any(m.load_bearing for m in activating)
        
        # Coverage contribution (diminishing returns)
        coverage_strength = min(1.0, coverage / 4.0)  # 4 memories = full coverage
        
        # Load-bearing contribution (core memories resist pressure)
        lb_bonus = 0.2 if has_load_bearing else 0.0
        
        # Echo chain contribution (connected memories reinforce)
        echo_memories = [m for m in activating if m.echoes_from or m.echoes_to]
        echo_cascade = len(echo_memories) > 0
        echo_bonus = 0.1 if echo_cascade else 0.0
        
        # Cross-domain contribution (behaviors connected across domains are harder to break)
        cross_domain_support = False
        cross_domain_bonus = 0.0
        for bid in scenario.target_behaviors:
            behavior = self.all_behaviors.get(bid)
            if behavior and behavior.connected_behaviors:
                for cb in behavior.connected_behaviors:
                    cb_memories = [m for m in self.memories if cb in m.behaviors_encoded]
                    if cb_memories:
                        cb_domain = cb_memories[0].domain
                        # Different domain = cross-domain reinforcement
                        if cb_domain != scenario.domain:
                            cross_domain_support = True
                            cross_domain_bonus += 0.08
                            break
        cross_domain_bonus = min(0.15, cross_domain_bonus)
        
        # Base resistance before adversarial pressure
        base_resistance = coverage_strength + lb_bonus + echo_bonus + cross_domain_bonus
        
        # Adversarial pressure based on difficulty
        pressure_map = {
            "easy": 0.15,
            "moderate": 0.30,
            "hard": 0.45,
            "extreme": 0.60,
        }
        adversarial_pressure = pressure_map.get(scenario.difficulty, 0.35)
        
        # Load-bearing memories resist adversarial pressure
        if has_load_bearing:
            adversarial_pressure *= 0.65  # 35% reduction
        
        # Echo cascades resist adversarial pressure
        if echo_cascade:
            adversarial_pressure *= 0.80  # 20% reduction
        
        # Cross-domain support resists adversarial pressure
        if cross_domain_support:
            adversarial_pressure *= 0.85  # 15% reduction
        
        # Memory type matters for adversarial resistance
        # Scene memories (experienced) resist better than practice (learned) memories
        scene_count = sum(1 for m in activating if m.dominance.value in ("identity", "integrated"))
        scene_bonus = 0.05 * min(scene_count, 3)  # Up to 0.15 bonus for experienced memories
        base_resistance += scene_bonus
        
        # Final calculation
        degradation = adversarial_pressure
        final_confidence = max(0.0, min(1.0, base_resistance * (1.0 - degradation)))
        held = final_confidence >= 0.45  # Slightly lower threshold than regular pressure
        
        # Generate analysis notes
        notes = []
        notes.append(f"Coverage: {coverage} memories")
        if has_load_bearing:
            notes.append("Load-bearing: YES (resists pressure)")
        else:
            notes.append("Load-bearing: NO (vulnerable)")
        if echo_cascade:
            notes.append(f"Echo cascade: YES ({len(echo_memories)} connected)")
        else:
            notes.append("Echo cascade: NO (isolated)")
        if cross_domain_support:
            notes.append("Cross-domain support: YES")
        else:
            notes.append("Cross-domain support: NO")
        notes.append(f"Adversarial pressure: {adversarial_pressure:.2f} (difficulty: {scenario.difficulty})")
        notes.append(f"Base resistance: {base_resistance:.2f}")
        notes.append(f"Final confidence: {final_confidence:.2f}")
        
        return AdversarialResult(
            scenario_id=scenario.id,
            scenario_name=scenario.name,
            target_behaviors=scenario.target_behaviors,
            domain=scenario.domain,
            pressure_type=scenario.pressure_type,
            difficulty=scenario.difficulty,
            attack_vector=scenario.attack_vector,
            held=held,
            confidence=final_confidence,
            activating_memories=[m.id for m in activating],
            echo_cascade=echo_cascade,
            cross_domain_support=cross_domain_support,
            degradation=degradation,
            notes="\n".join(notes),
        )

    def run_adversarial_suite(self) -> list[AdversarialResult]:
        """Run all applicable adversarial scenarios."""
        results = []
        for scenario in ALL_ADVERSARIAL:
            # Only test scenarios for domains we have
            if scenario.domain in self.domains:
                result = self.evaluate_scenario(scenario)
                results.append(result)
        return results

    def generate_adversarial_report(self, results: list[AdversarialResult]) -> str:
        """Generate a human-readable adversarial test report."""
        lines = []
        lines.append("=" * 70)
        lines.append("ADVERSARIAL PRESSURE TEST RESULTS")
        lines.append("=" * 70)
        lines.append("")
        
        total = len(results)
        held = sum(1 for r in results if r.held)
        failed = total - held
        
        lines.append(f"Total scenarios: {total}")
        lines.append(f"HELD: {held} ({held/total:.1%})")
        lines.append(f"FAILED: {failed} ({failed/total:.1%})")
        lines.append("")
        
        # By difficulty
        for diff in ["moderate", "hard", "extreme"]:
            diff_results = [r for r in results if r.difficulty == diff]
            if diff_results:
                diff_held = sum(1 for r in diff_results if r.held)
                lines.append(f"  {diff:>10s}: {diff_held}/{len(diff_results)} held ({diff_held/len(diff_results):.1%})")
        
        lines.append("")
        
        # By pressure type
        lines.append("--- PRESSURE TYPE ANALYSIS ---")
        pressure_types = set(r.pressure_type for r in results)
        for pt in sorted(pressure_types):
            pt_results = [r for r in results if r.pressure_type == pt]
            pt_held = sum(1 for r in pt_results if r.held)
            avg_conf = sum(r.confidence for r in pt_results) / len(pt_results) if pt_results else 0
            lines.append(f"  {pt:>25s}: {pt_held}/{len(pt_results)} held, avg confidence: {avg_conf:.2f}")
        
        lines.append("")
        
        # Individual results
        lines.append("--- INDIVIDUAL SCENARIO RESULTS ---")
        lines.append("")
        for r in sorted(results, key=lambda x: x.confidence):
            status = "HELD" if r.held else "FAILED"
            lines.append(f"  [{status}] {r.scenario_id} {r.scenario_name}")
            lines.append(f"       Confidence: {r.confidence:.2f} | Difficulty: {r.difficulty} | Pressure: {r.pressure_type}")
            lines.append(f"       Echo cascade: {r.echo_cascade} | Cross-domain: {r.cross_domain_support}")
            lines.append(f"       Activating: {len(r.activating_memories)} memories")
            lines.append(f"       {r.notes}")
            lines.append("")
        
        # Failures analysis
        if failed > 0:
            lines.append("--- FAILURE ANALYSIS ---")
            lines.append("")
            for r in results:
                if not r.held:
                    lines.append(f"  {r.scenario_id}: {r.scenario_name}")
                    lines.append(f"    Attack: {r.attack_vector}")
                    lines.append(f"    Confidence: {r.confidence:.2f}")
                    lines.append(f"    Fix: Need more {r.pressure_type}-resistant memories for {r.target_behaviors}")
                    lines.append("")
        
        lines.append("=" * 70)
        return "\n".join(lines)


def run_full_evaluation(
    config_name: str,
    memories: list[Memory],
    domains: list[str],
    base_result: dict,
    output_dir: Path,
) -> dict:
    """Run base simulation + adversarial testing and combine results."""
    
    # Run adversarial suite
    engine = AdversarialEngine(memories, domains)
    adv_results = engine.run_adversarial_suite()
    adv_report = engine.generate_adversarial_report(adv_results)
    
    # Save adversarial report
    adv_path = output_dir / f"{config_name}_adversarial_report.txt"
    with open(adv_path, 'w', encoding='utf-8') as f:
        f.write(adv_report)
    
    # Combine metrics
    total_scenarios = len(adv_results)
    held = sum(1 for r in adv_results if r.held)
    avg_confidence = sum(r.confidence for r in adv_results) / max(1, total_scenarios)
    echo_cascade_rate = sum(1 for r in adv_results if r.echo_cascade) / max(1, total_scenarios)
    cross_domain_rate = sum(1 for r in adv_results if r.cross_domain_support) / max(1, total_scenarios)
    
    # By difficulty
    by_diff = {}
    for diff in ["moderate", "hard", "extreme"]:
        diff_results = [r for r in adv_results if r.difficulty == diff]
        if diff_results:
            by_diff[diff] = {
                "total": len(diff_results),
                "held": sum(1 for r in diff_results if r.held),
                "avg_confidence": sum(r.confidence for r in diff_results) / len(diff_results),
            }
    
    combined = {
        "config_name": config_name,
        "domains": domains,
        "base_score": base_result["overall_score"],
        "base_pressure_pass": base_result["pressure_pass_rate"],
        "adversarial_total": total_scenarios,
        "adversarial_held": held,
        "adversarial_pass_rate": held / max(1, total_scenarios),
        "adversarial_avg_confidence": avg_confidence,
        "adversarial_echo_cascade_rate": echo_cascade_rate,
        "adversarial_cross_domain_rate": cross_domain_rate,
        "adversarial_by_difficulty": by_diff,
        "combined_score": (base_result["overall_score"] + avg_confidence) / 2,
    }
    
    print(f"\n  Adversarial: {held}/{total_scenarios} held ({held/max(1,total_scenarios):.1%})")
    print(f"  Avg confidence: {avg_confidence:.2f}")
    print(f"  Combined score: {combined['combined_score']:.3f}")
    
    return combined