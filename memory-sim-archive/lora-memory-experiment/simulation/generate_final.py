"""
Generate the final 250-memory corpus for training.

Uses the winning configuration: rhetoric+logic, dense coverage,
heavy load-bearing, strong echo chains.

Each memory is a scene where the skill was EXPERIENCED, not an
explanation of how the skill works. This is the core principle
from Mother: memories install behaviors through lived experience,
not through instruction.
"""

import json
import random
from pathlib import Path
from datetime import datetime

from simulation.domains import ALL_DOMAINS
from simulation.memory_schema import (
    Memory, ExperimentConfig, MemoryFlavor, MemoryDominance
)
from simulation.generator import generate_corpus, save_corpus
from simulation.engine import SimulationEngine, run_experiment
from simulation.adversarial import AdversarialEngine


# The winning configuration, scaled to 250 memories
FINAL_CONFIG = ExperimentConfig(
    name="rl_final_250",
    domains=["rhetoric", "logic"],
    memories_per_behavior=19,  # 13 × 19 = 247 memories
    echo_density=0.6,
    load_bearing_ratio=0.35,
    total_target=250,
)


def generate_final_corpus(output_dir: Path) -> tuple[list[Memory], dict]:
    """Generate the final corpus and validate it through simulation + adversarial testing."""
    print("=" * 70)
    print("GENERATING FINAL CORPUS: rl_final_250")
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
    corpus_path = output_dir / "final_corpus.json"
    save_corpus(memories, corpus_path)
    print(f"\nCorpus saved to: {corpus_path}")
    
    # Save base report
    report_path = output_dir / "final_base_report.txt"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    
    # Save adversarial report
    adv_report = adv_engine.generate_adversarial_report(adv_results)
    adv_path = output_dir / "final_adversarial_report.txt"
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
    
    validation_path = output_dir / "final_validation.json"
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
    print(f"  Base score: {result.overall_score:.3f}")
    print(f"  Adversarial: {adv_held}/{len(adv_results)} held")
    print(f"  Avg confidence: {avg_conf:.2f}")
    print(f"  Combined score: {validation['combined_score']:.3f}")
    print(f"  VALIDATED: {validation['validated']}")
    print("=" * 70)
    
    if validation['validated']:
        print("\n✓ Corpus validated. Ready for LoRA training on Llama 3.2 1B.")
    else:
        print("\n✗ Corpus has issues. Review before training.")
    
    return memories, validation


if __name__ == "__main__":
    output_dir = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/corpus/final")
    memories, validation = generate_final_corpus(output_dir)