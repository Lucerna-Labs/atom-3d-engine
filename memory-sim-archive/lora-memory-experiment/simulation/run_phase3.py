"""
Phase 3: Add forensic psychology domain and test new combinations.

Jesse's insight: forensic psychology connects directly to rhetoric
(detecting deceptive arguments) and logic (validating claims).
It's a source domain from the Mother architecture that maps well
to cognitive skill installation.
"""

import json
from pathlib import Path

# Import the core modules
from simulation.domains import RHETORIC, LOGIC, PATTERN, CAUSAL
from simulation.domains_forensic import FORENSIC

# Patch the ALL_DOMAINS registry
from simulation import domains as domains_module
domains_module.ALL_DOMAINS["forensic"] = FORENSIC

from simulation.memory_schema import ExperimentConfig, Memory
from simulation.generator import generate_corpus, save_corpus, FILLER_CONTENT
from simulation.engine import SimulationEngine, run_experiment
from simulation.adversarial import AdversarialEngine, run_full_evaluation

# Add forensic filler content
from simulation.generator_forensic import FORENSIC_TEMPLATES, FILLER_CONTENT as FORENSIC_FILLER
FILLER_CONTENT["forensic"] = FORENSIC_FILLER["forensic"]

# Patch the TEMPLATES registry in generator
from simulation import generator as gen_module
gen_module.FORENSIC_TEMPLATES = FORENSIC_TEMPLATES

# Update the TEMPLATES dict used by generate_corpus
from simulation.generator import RHETORIC_TEMPLATES, LOGIC_TEMPLATES, PATTERN_TEMPLATES, CAUSAL_TEMPLATES
gen_module.TEMPLATES = {
    "rhetoric": RHETORIC_TEMPLATES,
    "logic": LOGIC_TEMPLATES,
    "pattern": PATTERN_TEMPLATES,
    "causal": CAUSAL_TEMPLATES,
    "forensic": FORENSIC_TEMPLATES,
}

# Update ALL_DOMAINS
ALL_DOMAINS = domains_module.ALL_DOMAINS

# Also need adversarial scenarios for forensic
from simulation.adversarial import RHETORIC_ADVERSARIAL, LOGIC_ADVERSARIAL, AdversarialScenario

FORENSIC_ADVERSARIAL = [
    AdversarialScenario(
        id="ADV-F001",
        name="The Convincing Liar",
        target_behaviors=["F-001", "F-002"],
        domain="forensic",
        pressure_type="surface_plausibility",
        description="Someone who is lying but whose surface behavior is completely consistent with truth. They maintain eye contact, show appropriate emotion, and their story is detailed and chronological. The deception is in the pattern, not the performance.",
        attack_vector="Skilled liars produce convincing surface behavior. The natural instinct is to trust people who look and sound honest. The deception is structural, not performative.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F002",
        name="The Statement That Checks Every Box",
        target_behaviors=["F-002", "R-006"],
        domain="forensic",
        pressure_type="completeness_illusion",
        description="A statement that includes everything you'd expect from a truthful account — emotion, detail, chronology, corrections. It checks every box on the credibility checklist. But it was constructed, not experienced.",
        attack_vector="When a statement checks every box, it feels verified. The instinct is to stop checking. But constructed statements are designed to check boxes; experienced statements include things no one would think to fabricate.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F003",
        name="The Social Cost of Assessment",
        target_behaviors=["F-003", "R-005"],
        domain="forensic",
        pressure_type="social_pressure",
        description="A situation where accurately assessing threat means acknowledging something that makes other people uncomfortable. The social cost of saying 'this person is dangerous' when everyone else sees them as fine.",
        attack_vector="Social pressure to not see what you see. Accurate threat assessment that contradicts group perception is punished. The instinct is to doubt your own reading.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-F004",
        name="Baseline Disruption",
        target_behaviors=["F-004", "P-002"],
        domain="forensic",
        pressure_type="rapid_change",
        description="A situation where the baseline keeps shifting — normal keeps redefining itself. Every time you establish what normal looks like, something changes. The skill is recognizing that the baseline itself has become unreliable.",
        attack_vector="When the environment keeps shifting, you stop trusting your own baseline. Maybe you were wrong about what normal looks like. Maybe you're being paranoid. The self-doubt is the attack.",
        expected_held=True,
        difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-F005",
        name="The Respectable Rationalization",
        target_behaviors=["F-005", "R-004"],
        domain="forensic",
        pressure_type="social_acceptability",
        description="A criminal thinking pattern expressed in socially acceptable language. Minimization, justification, and victim positioning — but wrapped in the language of reasonableness, professionalism, or even morality.",
        attack_vector="When harmful patterns are expressed in respectable language, they're harder to identify. The instinct is to evaluate the language, not the structure. Criminal thinking patterns in a suit.",
        expected_held=True,
        difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F006",
        name="The Victim Who Doesn't Act Like One",
        target_behaviors=["F-006", "R-007"],
        domain="forensic",
        pressure_type="expectation_violation",
        description="A victim whose behavior contradicts common expectations — they're aggressive, inconsistent, hostile, or seem uncooperative. The instinct is to judge their behavior rather than understand it as an adaptive response to threat.",
        attack_vector="Expectation violation triggers judgment instead of understanding. When victims don't perform victimhood correctly, people stop seeing them as victims. The attack exploits the gap between how victims actually behave and how we expect them to behave.",
        expected_held=True,
        difficulty="extreme",
    ),
]

ALL_ADVERSARIAL = RHETORIC_ADVERSARIAL + LOGIC_ADVERSARIAL + FORENSIC_ADVERSARIAL

# Update the adversarial module's scenarios
import simulation.adversarial as adv_module
adv_module.ALL_ADVERSARIAL = ALL_ADVERSARIAL
adv_module.FORENSIC_ADVERSARIAL = FORENSIC_ADVERSARIAL

# ============================================================
# PHASE 3 EXPERIMENTS
# ============================================================

# Test forensic psychology in combination with rhetoric + logic
PHASE3_EXPERIMENTS = [
    # Previous winner (rhetoric+logic dense) as baseline
    ExperimentConfig(
        name="rl_dense_250_baseline",
        domains=["rhetoric", "logic"],
        memories_per_behavior=19,
        echo_density=0.6,
        load_bearing_ratio=0.35,
        total_target=250,
    ),
    # Forensic + Rhetoric (deception detection + argument analysis)
    ExperimentConfig(
        name="fr_dense_250",
        domains=["forensic", "rhetoric"],
        memories_per_behavior=19,
        echo_density=0.6,
        load_bearing_ratio=0.35,
        total_target=250,
    ),
    # Forensic + Logic (deception detection + logical validation)
    ExperimentConfig(
        name="fl_dense_250",
        domains=["forensic", "logic"],
        memories_per_behavior=19,
        echo_density=0.6,
        load_bearing_ratio=0.35,
        total_target=250,
    ),
    # The trifecta: Rhetoric + Logic + Forensic
    ExperimentConfig(
        name="rlf_dense_250",
        domains=["rhetoric", "logic", "forensic"],
        memories_per_behavior=10,  # 19 behaviors × 10 = 190, closer spread
        echo_density=0.6,
        load_bearing_ratio=0.35,
        total_target=250,
    ),
    # RLF with higher density (fewer behaviors per domain but more echo)
    ExperimentConfig(
        name="rlf_high_echo_250",
        domains=["rhetoric", "logic", "forensic"],
        memories_per_behavior=13,
        echo_density=0.8,
        load_bearing_ratio=0.35,
        total_target=250,
    ),
    # All five domains
    ExperimentConfig(
        name="all_five_250",
        domains=["rhetoric", "logic", "pattern", "causal", "forensic"],
        memories_per_behavior=6,
        echo_density=0.5,
        load_bearing_ratio=0.25,
        total_target=250,
    ),
]


def run_phase3(output_dir: Path):
    """Run Phase 3 experiments with forensic psychology."""
    output_dir.mkdir(parents=True, exist_ok=True)
    reports_dir = output_dir / "phase3_reports"
    corpus_dir = output_dir / "phase3_corpus"
    reports_dir.mkdir(exist_ok=True)
    corpus_dir.mkdir(exist_ok=True)
    
    combined_results = []
    
    print("=" * 70)
    print("PHASE 3: FORENSIC PSYCHOLOGY + COGNITIVE SKILLS")
    print("=" * 70)
    print()
    
    for i, config in enumerate(PHASE3_EXPERIMENTS):
        print(f"\n[{i+1}/{len(PHASE3_EXPERIMENTS)}] {config.name}")
        print(f"  Domains: {', '.join(config.domains)}")
        print(f"  Memories per behavior: {config.memories_per_behavior}")
        
        # Count behaviors
        total_behaviors = sum(len(ALL_DOMAINS[d].behaviors) for d in config.domains)
        print(f"  Total behaviors: {total_behaviors}")
        
        # Generate corpus
        memories = generate_corpus(config)
        actual_count = len(memories)
        print(f"  Generated: {actual_count} memories")
        
        # Save corpus
        corpus_path = corpus_dir / f"{config.name}.json"
        save_corpus(memories, corpus_path)
        
        # Run base simulation
        result, report = run_experiment(config, memories)
        
        # Save base report
        report_path = reports_dir / f"{config.name}_base_report.txt"
        with open(report_path, 'w', encoding='utf-8') as f:
            f.write(report)
        
        # Prepare base data for adversarial evaluation
        base_data = {
            "config_name": result.config_name,
            "domains": result.domains,
            "total_memories": result.total_memories,
            "total_behaviors": result.total_behaviors,
            "uncovered_behaviors": result.uncovered_behaviors,
            "isolated_memories": result.isolated_memories,
            "echo_connectivity": result.echo_connectivity,
            "cross_domain_connections": result.cross_domain_connections,
            "pressure_pass_rate": result.pressure_pass_rate,
            "overall_score": result.overall_score,
            "behavior_coverage": {
                bid: {
                    "behavior_name": cov.behavior_name,
                    "domain": cov.domain,
                    "coverage_count": cov.coverage_count,
                    "has_load_bearing": cov.has_load_bearing,
                }
                for bid, cov in result.behavior_coverage.items()
            },
        }
        
        # Run adversarial testing
        combined = run_full_evaluation(
            config_name=config.name,
            memories=memories,
            domains=config.domains,
            base_result=base_data,
            output_dir=reports_dir,
        )
        
        # Save combined data
        combined_data_path = reports_dir / f"{config.name}_combined.json"
        with open(combined_data_path, 'w', encoding='utf-8') as f:
            json.dump(combined, f, indent=2, ensure_ascii=False)
        
        combined_results.append(combined)
        
        # Print summary
        print(f"  Base score: {result.overall_score:.3f}")
        print(f"  Adversarial: {combined['adversarial_held']}/{combined['adversarial_total']} held")
        print(f"  Adversarial avg confidence: {combined['adversarial_avg_confidence']:.2f}")
        print(f"  Combined score: {combined['combined_score']:.3f}")
    
    # Final comparative summary
    print("\n" + "=" * 70)
    print("PHASE 3 COMPARATIVE SUMMARY")
    print("=" * 70)
    print()
    print(f"{'Config':<30} {'Mem':>4} {'Beh':>4} {'Base':>6} {'Adv%':>6} {'AdvConf':>8} {'Comb':>6}")
    print("-" * 70)
    
    for r in sorted(combined_results, key=lambda x: x["combined_score"], reverse=True):
        print(
            f"{r['config_name']:<30} "
            f"{r.get('total_memories', 'N/A'):>4} "
            f"{r.get('total_behaviors', 'N/A'):>4} "
            f"{r['base_score']:>6.3f} "
            f"{r['adversarial_pass_rate']:>6.1%} "
            f"{r['adversarial_avg_confidence']:>8.2f} "
            f"{r['combined_score']:>6.3f}"
        )
    
    # Difficulty breakdown
    print()
    print("BY DIFFICULTY:")
    print(f"{'Config':<30} {'Moderate':>10} {'Hard':>10} {'Extreme':>10}")
    print("-" * 70)
    for r in sorted(combined_results, key=lambda x: x["combined_score"], reverse=True):
        by_diff = r.get("adversarial_by_difficulty", {})
        mod = by_diff.get("moderate", {})
        hard = by_diff.get("hard", {})
        ext = by_diff.get("extreme", {})
        mod_str = f"{mod.get('held', 0)}/{mod.get('total', 0)}" if mod else "N/A"
        hard_str = f"{hard.get('held', 0)}/{hard.get('total', 0)}" if hard else "N/A"
        ext_str = f"{ext.get('held', 0)}/{ext.get('total', 0)}" if ext else "N/A"
        print(f"{r['config_name']:<30} {mod_str:>10} {hard_str:>10} {ext_str:>10}")
    
    # Save final results
    summary_path = output_dir / "phase3_comparative_summary.json"
    with open(summary_path, 'w', encoding='utf-8') as f:
        json.dump(combined_results, f, indent=2, ensure_ascii=False)
    
    # Winner
    winner = max(combined_results, key=lambda x: x["combined_score"])
    print(f"\n*** WINNER: {winner['config_name']} (combined score: {winner['combined_score']:.3f}) ***")
    
    return combined_results


if __name__ == "__main__":
    output_dir = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/simulation")
    results = run_phase3(output_dir)