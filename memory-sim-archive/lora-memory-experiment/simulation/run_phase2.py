"""
Phase 2 experiment runner.

Scales the winning configuration (rhetoric+logic) up to 250 memories
and runs adversarial testing on all configurations.

The data drives. If results are bad, we iterate. If they're good, we train.
"""

import json
import sys
from pathlib import Path
from datetime import datetime

from simulation.domains import ALL_DOMAINS
from simulation.memory_schema import ExperimentConfig, Memory
from simulation.generator import generate_corpus, save_corpus, load_corpus
from simulation.engine import SimulationEngine, run_experiment
from simulation.adversarial import AdversarialEngine, run_full_evaluation


# ============================================================
# PHASE 2 CONFIGURATIONS
# ============================================================

# Focus on the winning pair (rhetoric+logic) with variations
# and a few challenger configurations for comparison

PHASE2_EXPERIMENTS = [
    # Winner from Phase 1, scaled to 250 memories
    ExperimentConfig(
        name="rl_dense_250",
        domains=["rhetoric", "logic"],
        memories_per_behavior=9,   # ~117 base, echo expansion gets us to ~250
        echo_density=0.6,
        load_bearing_ratio=0.3,
        total_target=250,
    ),
    # Dense with very high echo connectivity
    ExperimentConfig(
        name="rl_dense_high_echo_250",
        domains=["rhetoric", "logic"],
        memories_per_behavior=8,
        echo_density=0.8,
        load_bearing_ratio=0.3,
        total_target=250,
    ),
    # Dense with more load-bearing memories
    ExperimentConfig(
        name="rl_dense_heavy_lb_250",
        domains=["rhetoric", "logic"],
        memories_per_behavior=9,
        echo_density=0.5,
        load_bearing_ratio=0.45,  # Almost half load-bearing
        total_target=250,
    ),
    # Dense, all identity-dominance (strongest installation)
    ExperimentConfig(
        name="rl_dense_identity_250",
        domains=["rhetoric", "logic"],
        memories_per_behavior=9,
        echo_density=0.6,
        load_bearing_ratio=0.5,
        total_target=250,
    ),
    # Three-domain challenger (rhetoric+logic+pattern)
    ExperimentConfig(
        name="rlp_dense_250",
        domains=["rhetoric", "logic", "pattern"],
        memories_per_behavior=6,
        echo_density=0.5,
        load_bearing_ratio=0.25,
        total_target=250,
    ),
    # Three-domain challenger (rhetoric+logic+causal)
    ExperimentConfig(
        name="rlc_dense_250",
        domains=["rhetoric", "logic", "causal"],
        memories_per_behavior=6,
        echo_density=0.5,
        load_bearing_ratio=0.25,
        total_target=250,
    ),
    # All four domains, but concentrated
    ExperimentConfig(
        name="all_dense_250",
        domains=["rhetoric", "logic", "pattern", "causal"],
        memories_per_behavior=4,
        echo_density=0.5,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
]


def run_phase2(output_dir: Path):
    """Run Phase 2 experiments with adversarial testing."""
    output_dir.mkdir(parents=True, exist_ok=True)
    reports_dir = output_dir / "phase2_reports"
    corpus_dir = output_dir / "phase2_corpus"
    reports_dir.mkdir(exist_ok=True)
    corpus_dir.mkdir(exist_ok=True)
    
    combined_results = []
    
    print("=" * 70)
    print("PHASE 2: SCALED EXPERIMENTS WITH ADVERSARIAL TESTING")
    print("=" * 70)
    print()
    
    for i, config in enumerate(PHASE2_EXPERIMENTS):
        print(f"\n[{i+1}/{len(PHASE2_EXPERIMENTS)}] {config.name}")
        print(f"  Domains: {', '.join(config.domains)}")
        print(f"  Memories per behavior: {config.memories_per_behavior}")
        print(f"  Echo density: {config.echo_density}")
        print(f"  Load-bearing ratio: {config.load_bearing_ratio}")
        
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
        
        # Save base result data
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
        print(f"  Adversarial pass rate: {combined['adversarial_pass_rate']:.1%}")
        print(f"  Adversarial avg confidence: {combined['adversarial_avg_confidence']:.2f}")
        print(f"  Combined score: {combined['combined_score']:.3f}")
    
    # Final comparative summary
    print("\n" + "=" * 70)
    print("PHASE 2 COMPARATIVE SUMMARY")
    print("=" * 70)
    print()
    print(f"{'Config':<30} {'Base':>6} {'Adv%':>6} {'AdvConf':>8} {'Comb':>6} {'Echo':>5} {'XDom':>5}")
    print("-" * 70)
    
    for r in sorted(combined_results, key=lambda x: x["combined_score"], reverse=True):
        print(
            f"{r['config_name']:<30} "
            f"{r['base_score']:>6.3f} "
            f"{r['adversarial_pass_rate']:>6.1%} "
            f"{r['adversarial_avg_confidence']:>8.2f} "
            f"{r['combined_score']:>6.3f} "
            f"{r.get('base_echo', 0):>5.1%} "
            f"{r.get('base_xdom', 0):>5d}"
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
    summary_path = output_dir / "phase2_comparative_summary.json"
    with open(summary_path, 'w', encoding='utf-8') as f:
        json.dump(combined_results, f, indent=2, ensure_ascii=False)
    
    print(f"\nResults saved to: {output_dir}")
    
    # Winner announcement
    winner = max(combined_results, key=lambda x: x["combined_score"])
    print(f"\n*** WINNER: {winner['config_name']} (combined score: {winner['combined_score']:.3f}) ***")
    
    return combined_results


if __name__ == "__main__":
    output_dir = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/simulation")
    results = run_phase2(output_dir)