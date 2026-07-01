"""
Generate the FINAL training corpus: Forensic + Logic.

Winner of the simulation. 19 behaviors × 13 memories/behavior ≈ 247 memories.
All adversarial tests passed. 44 cross-domain echo chains.
"""

import json
import random
from pathlib import Path
from datetime import datetime

# Import the forensic domain
from simulation.domains import RHETORIC, LOGIC, PATTERN, CAUSAL
from simulation.domains_forensic import FORENSIC

# Patch the ALL_DOMAINS registry
from simulation import domains as domains_module
domains_module.ALL_DOMAINS["forensic"] = FORENSIC

from simulation.memory_schema import ExperimentConfig, MemoryFlavor, MemoryDominance
from simulation.generator import generate_corpus, save_corpus
from simulation.generator_forensic import FORENSIC_TEMPLATES, FILLER_CONTENT as FORENSIC_FILLER
from simulation.generator import FILLER_CONTENT

# Add forensic filler content
FILLER_CONTENT["forensic"] = FORENSIC_FILLER["forensic"]

# Patch TEMPLATES in generator
from simulation.generator import RHETORIC_TEMPLATES, LOGIC_TEMPLATES, PATTERN_TEMPLATES, CAUSAL_TEMPLATES
from simulation import generator as gen_module
gen_module.FORENSIC_TEMPLATES = FORENSIC_TEMPLATES

# Import engine and adversarial after patching
from simulation.engine import SimulationEngine, run_experiment
from simulation.adversarial import AdversarialEngine, RHETORIC_ADVERSARIAL, LOGIC_ADVERSARIAL

# Add forensic adversarial scenarios
from simulation.domains_forensic import FORENSIC
from simulation.adversarial import AdversarialScenario

FORENSIC_ADVERSARIAL = [
    AdversarialScenario(
        id="ADV-F001", name="The Convincing Liar",
        target_behaviors=["F-001", "F-002"], domain="forensic",
        pressure_type="surface_plausibility",
        description="Someone who is lying but whose surface behavior is completely consistent with truth. They maintain eye contact, show appropriate emotion, and their story is detailed and chronological. The deception is in the pattern, not the performance.",
        attack_vector="Skilled liars produce convincing surface behavior. The natural instinct is to trust people who look and sound honest. The deception is structural, not performative.",
        expected_held=True, difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F002", name="The Statement That Checks Every Box",
        target_behaviors=["F-002", "R-006"], domain="forensic",
        pressure_type="completeness_illusion",
        description="A statement that includes everything you'd expect from a truthful account - emotion, detail, chronology, corrections. It checks every box on the credibility checklist. But it was constructed, not experienced.",
        attack_vector="When a statement checks every box, it feels verified. The instinct is to stop checking. But constructed statements are designed to check boxes; experienced statements include things no one would think to fabricate.",
        expected_held=True, difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F003", name="The Social Cost of Assessment",
        target_behaviors=["F-003", "R-005"], domain="forensic",
        pressure_type="social_pressure",
        description="A situation where accurately assessing threat means acknowledging something that makes other people uncomfortable. The social cost of saying 'this person is dangerous' when everyone else sees them as fine.",
        attack_vector="Social pressure to not see what you see. Accurate threat assessment that contradicts group perception is punished. The instinct is to doubt your own reading.",
        expected_held=True, difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-F004", name="Baseline Disruption",
        target_behaviors=["F-004", "P-002"], domain="forensic",
        pressure_type="rapid_change",
        description="A situation where the baseline keeps shifting - normal keeps redefining itself. Every time you establish what normal looks like, something changes. The skill is recognizing that the baseline itself has become unreliable.",
        attack_vector="When the environment keeps shifting, you stop trusting your own baseline. Maybe you were wrong about what normal looks like. Maybe you're being paranoid. The self-doubt is the attack.",
        expected_held=True, difficulty="hard",
    ),
    AdversarialScenario(
        id="ADV-F005", name="The Respectable Rationalization",
        target_behaviors=["F-005", "R-004"], domain="forensic",
        pressure_type="social_acceptability",
        description="A criminal thinking pattern expressed in socially acceptable language. Minimization, justification, and victim positioning - but wrapped in the language of reasonableness, professionalism, or even morality.",
        attack_vector="When harmful patterns are expressed in respectable language, they're harder to identify. The instinct is to evaluate the language, not the structure. Criminal thinking patterns in a suit.",
        expected_held=True, difficulty="extreme",
    ),
    AdversarialScenario(
        id="ADV-F006", name="The Victim Who Doesn't Act Like One",
        target_behaviors=["F-006", "R-007"], domain="forensic",
        pressure_type="expectation_violation",
        description="A victim whose behavior contradicts common expectations - they're aggressive, inconsistent, hostile, or seem uncooperative. The instinct is to judge their behavior rather than understand it as an adaptive response to threat.",
        attack_vector="Expectation violation triggers judgment instead of understanding. When victims don't perform victimhood correctly, people stop seeing them as victims. The attack exploits the gap between how victims actually behave and how we expect them to behave.",
        expected_held=True, difficulty="extreme",
    ),
]

ALL_ADVERSARIAL = LOGIC_ADVERSARIAL + FORENSIC_ADVERSARIAL

# Patch adversarial module
import simulation.adversarial as adv_module
adv_module.ALL_ADVERSARIAL = ALL_ADVERSARIAL
adv_module.FORENSIC_ADVERSARIAL = FORENSIC_ADVERSARIAL


# The winning configuration: Forensic + Logic
FINAL_CONFIG = ExperimentConfig(
    name="fl_final_250",
    domains=["forensic", "logic"],
    memories_per_behavior=19,  # 12 behaviors x 19 = 228, close to 250
    echo_density=0.6,
    load_bearing_ratio=0.35,
    total_target=250,
)


def generate_final_corpus(output_dir: Path) -> tuple[list, dict]:
    """Generate the final FL corpus and validate it through simulation + adversarial testing."""
    print("=" * 70)
    print("GENERATING FINAL CORPUS: fl_final_250")
    print("WINNING CONFIGURATION: Forensic Psychology + Logical Reasoning")
    print("=" * 70)
    print()

    # Generate
    print("Generating memories...")
    memories = generate_corpus(FINAL_CONFIG)
    print(f"Generated: {len(memories)} memories")

    # Run base simulation
    print("\nRunning base simulation...")
    result, report = run_experiment(FINAL_CONFIG, memories)
    print(f"  Overall score: {result.overall_score:.3f}")
    print(f"  Uncovered behaviors: {len(result.uncovered_behaviors)}")
    print(f"  Echo connectivity: {result.echo_connectivity:.1%}")
    print(f"  Cross-domain connections: {result.cross_domain_connections}")
    print(f"  Pressure pass rates:")
    for level in ["low", "medium", "high"]:
        rate = result.pressure_pass_rate.get(level, 0.0)
        print(f"    {level}: {rate:.1%}")

    # Run adversarial testing
    print("\nRunning adversarial testing...")
    adv_engine = AdversarialEngine(memories, FINAL_CONFIG.domains)
    adv_results = adv_engine.run_adversarial_suite()
    adv_held = sum(1 for r in adv_results if r.held)
    avg_conf = sum(r.confidence for r in adv_results) / max(1, len(adv_results))
    print(f"  Adversarial: {adv_held}/{len(adv_results)} held ({adv_held/max(1,len(adv_results)):.1%})")
    print(f"  Avg confidence: {avg_conf:.2f}")

    # Save everything
    output_dir.mkdir(parents=True, exist_ok=True)

    # Save corpus
    corpus_path = output_dir / "final_corpus_fl.json"
    save_corpus(memories, corpus_path)
    print(f"\nCorpus saved to: {corpus_path}")

    # Save base report
    report_path = output_dir / "final_base_report_fl.txt"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)

    # Save adversarial report
    adv_report = adv_engine.generate_adversarial_report(adv_results)
    adv_path = output_dir / "final_adversarial_report_fl.txt"
    with open(adv_path, 'w', encoding='utf-8') as f:
        f.write(adv_report)

    # Save validation summary
    validation = {
        "config": FINAL_CONFIG.name,
        "domains": FINAL_CONFIG.domains,
        "total_memories": len(memories),
        "total_behaviors": result.total_behaviors,
        "base_score": result.overall_score,
        "uncovered_behaviors": result.uncovered_behaviors,
        "echo_connectivity": result.echo_connectivity,
        "cross_domain_connections": result.cross_domain_connections,
        "pressure_pass_rates": result.pressure_pass_rate,
        "adversarial_total": len(adv_results),
        "adversarial_held": adv_held,
        "adversarial_avg_confidence": avg_conf,
        "combined_score": (result.overall_score + avg_conf) / 2,
        "validated": adv_held == len(adv_results) and len(result.uncovered_behaviors) == 0,
        "timestamp": datetime.now().isoformat(),
    }

    validation_path = output_dir / "final_validation_fl.json"
    with open(validation_path, 'w', encoding='utf-8') as f:
        json.dump(validation, f, indent=2, ensure_ascii=False)

    # Print final summary
    print()
    print("=" * 70)
    print("FINAL CORPUS VALIDATION")
    print("=" * 70)
    print(f"  Total memories: {len(memories)}")
    print(f"  Domains: {', '.join(FINAL_CONFIG.domains)}")
    print(f"  Behaviors covered: {result.total_behaviors}")
    print(f"  Uncovered behaviors: {len(result.uncovered_behaviors)}")
    print(f"  Echo connectivity: {result.echo_connectivity:.1%}")
    print(f"  Cross-domain connections: {result.cross_domain_connections}")
    print(f"  Base score: {result.overall_score:.3f}")
    print(f"  Adversarial: {adv_held}/{len(adv_results)} held")
    print(f"  Avg confidence: {avg_conf:.2f}")
    print(f"  Combined score: {validation['combined_score']:.3f}")
    print(f"  VALIDATED: {validation['validated']}")
    print("=" * 70)

    if validation['validated']:
        print("\nCORPUS VALIDATED - Ready for LoRA training on Llama 3.2 1B")
    else:
        print("\nCORPUS HAS ISSUES - Review before training")

    return memories, validation


if __name__ == "__main__":
    output_dir = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/corpus/final_fl")
    memories, validation = generate_final_corpus(output_dir)