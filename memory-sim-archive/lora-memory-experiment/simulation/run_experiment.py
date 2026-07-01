"""
Main experiment runner.

Tests multiple domain combinations and configurations to find
the best setup for a 250-memory LoRA on Llama 3.2 1B.
"""

import json
import sys
from pathlib import Path
from datetime import datetime

from simulation.domains import ALL_DOMAINS
from simulation.memory_schema import ExperimentConfig
from simulation.generator import generate_corpus, save_corpus
from simulation.engine import SimulationEngine, run_experiment


# ============================================================
# EXPERIMENT CONFIGURATIONS
# ============================================================

# We're testing different domain combinations and memory densities
# to find what works best for 250 memories on a small model.

EXPERIMENTS = [
    # Single domain tests
    ExperimentConfig(
        name="rhetoric_only",
        domains=["rhetoric"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    ExperimentConfig(
        name="logic_only",
        domains=["logic"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    # Two-domain combinations
    ExperimentConfig(
        name="rhetoric_logic",
        domains=["rhetoric", "logic"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    ExperimentConfig(
        name="rhetoric_pattern",
        domains=["rhetoric", "pattern"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    ExperimentConfig(
        name="logic_causal",
        domains=["logic", "causal"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    # Three-domain combinations
    ExperimentConfig(
        name="rhetoric_logic_pattern",
        domains=["rhetoric", "logic", "pattern"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    ExperimentConfig(
        name="rhetoric_logic_causal",
        domains=["rhetoric", "logic", "causal"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    # All four domains
    ExperimentConfig(
        name="all_four",
        domains=["rhetoric", "logic", "pattern", "causal"],
        memories_per_behavior=3,
        echo_density=0.3,
        load_bearing_ratio=0.2,
        total_target=250,
    ),
    # Variations on best candidates (density tests)
    ExperimentConfig(
        name="rhetoric_logic_dense",
        domains=["rhetoric", "logic"],
        memories_per_behavior=5,  # More memories per behavior
        echo_density=0.5,  # More echo connections
        load_bearing_ratio=0.3,  # More load-bearing memories
        total_target=250,
    ),
    ExperimentConfig(
        name="rhetoric_logic_sparse",
        domains=["rhetoric", "logic"],
        memories_per_behavior=2,  # Fewer memories per behavior
        echo_density=0.2,  # Fewer echo connections
        load_bearing_ratio=0.15,
        total_target=250,
    ),
    ExperimentConfig(
        name="rhetoric_logic_high_echo",
        domains=["rhetoric", "logic"],
        memories_per_behavior=3,
        echo_density=0.7,  # High echo connectivity
        load_bearing_ratio=0.2,
        total_target=250,
    ),
]


def run_all_experiments(output_dir: Path):
    """Run all experiments and save results."""
    output_dir.mkdir(parents=True, exist_ok=True)
    reports_dir = output_dir / "reports"
    corpus_dir = output_dir / "corpus"
    reports_dir.mkdir(exist_ok=True)
    corpus_dir.mkdir(exist_ok=True)
    
    results = []
    
    print(f"Running {len(EXPERIMENTS)} experiments...")
    print(f"Output directory: {output_dir}")
    print("=" * 70)
    
    for i, config in enumerate(EXPERIMENTS):
        print(f"\n[{i+1}/{len(EXPERIMENTS)}] {config.name}")
        print(f"  Domains: {', '.join(config.domains)}")
        print(f"  Target: ~{config.estimated_memories} memories")
        
        # Generate corpus
        memories = generate_corpus(config)
        actual_count = len(memories)
        print(f"  Generated: {actual_count} memories")
        
        # Save corpus
        corpus_path = corpus_dir / f"{config.name}.json"
        save_corpus(memories, corpus_path)
        
        # Run simulation
        result, report = run_experiment(config, memories)
        
        # Save report
        report_path = reports_dir / f"{config.name}_report.txt"
        with open(report_path, 'w', encoding='utf-8') as f:
            f.write(report)
        
        # Save result data
        result_data = {
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
        results_path = reports_dir / f"{config.name}_data.json"
        with open(results_path, 'w', encoding='utf-8') as f:
            json.dump(result_data, f, indent=2, ensure_ascii=False)
        
        # Print summary
        print(f"  Overall score: {result.overall_score:.3f}")
        print(f"  Uncovered behaviors: {len(result.uncovered_behaviors)}")
        print(f"  Echo connectivity: {result.echo_connectivity:.1%}")
        print(f"  Cross-domain connections: {result.cross_domain_connections}")
        print(f"  Pressure pass rates:")
        for level in ["low", "medium", "high"]:
            rate = result.pressure_pass_rate.get(level, 0.0)
            print(f"    {level}: {rate:.1%}")
        
        results.append(result_data)
    
    # Comparative summary
    print("\n" + "=" * 70)
    print("COMPARATIVE SUMMARY")
    print("=" * 70)
    print(f"\n{'Config':<30} {'Score':>6} {'Cov':>5} {'Echo':>5} {'Low':>5} {'Med':>5} {'High':>5} {'XDom':>5}")
    print("-" * 70)
    
    for r in sorted(results, key=lambda x: x["overall_score"], reverse=True):
        coverage = 1.0 - (len(r["uncovered_behaviors"]) / max(1, r["total_behaviors"]))
        print(
            f"{r['config_name']:<30} "
            f"{r['overall_score']:>6.3f} "
            f"{coverage:>5.1%} "
            f"{r['echo_connectivity']:>5.1%} "
            f"{r['pressure_pass_rate'].get('low', 0):>5.1%} "
            f"{r['pressure_pass_rate'].get('medium', 0):>5.1%} "
            f"{r['pressure_pass_rate'].get('high', 0):>5.1%} "
            f"{r['cross_domain_connections']:>5d}"
        )
    
    # Save comparative summary
    summary_path = output_dir / "comparative_summary.json"
    with open(summary_path, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2, ensure_ascii=False)
    
    print(f"\nResults saved to: {output_dir}")
    print(f"Individual reports: {reports_dir}")
    
    return results


if __name__ == "__main__":
    output_dir = Path("F:/OPENCLAW-PROJECTS/lora-memory-experiment/simulation")
    results = run_all_experiments(output_dir)