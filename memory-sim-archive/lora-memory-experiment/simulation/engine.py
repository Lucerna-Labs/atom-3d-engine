"""
Core simulation engine for the LoRA Memory Experiment.

Tests whether a set of memories actually installs the target behaviors,
using coverage analysis, echo chain tracing, and pressure testing.

This is the adapted Mother simulation — same principles, different target:
instead of identity behaviors, we're testing cognitive skill behaviors.
"""

import json
import random
from dataclasses import dataclass, field
from typing import Optional
from pathlib import Path

from simulation.domains import ALL_DOMAINS, Behavior
from simulation.memory_schema import Memory, ExperimentConfig, MemoryFlavor, MemoryDominance


@dataclass
class BehaviorCoverage:
    """Track which behaviors are covered by which memories."""
    behavior_id: str
    behavior_name: str
    domain: str
    memory_ids: list[str] = field(default_factory=list)
    coverage_count: int = 0
    has_load_bearing: bool = False
    echo_in_count: int = 0
    echo_out_count: int = 0


@dataclass
class EchoChain:
    """A chain of connected memories."""
    chain_id: str
    memory_ids: list[str]  # In chronological order
    behaviors_covered: list[str]
    domain_span: list[str]
    strength: float = 1.0  # 1.0 = fully connected, decreases with gaps


@dataclass
class PressureResult:
    """Result of a pressure test scenario."""
    scenario_id: str
    scenario_name: str
    pressure_level: str  # "low", "medium", "high"
    domain: str
    behaviors_tested: list[str]
    memories_activated: list[str]
    held: bool  # Did the behavior hold?
    confidence: float  # 0.0-1.0
    degradation: float  # How much the behavior degraded under pressure
    notes: str = ""


@dataclass
class SimulationResult:
    """Complete result of a simulation run."""
    config_name: str
    domains: list[str]
    total_memories: int
    total_behaviors: int
    behavior_coverage: dict[str, BehaviorCoverage]
    uncovered_behaviors: list[str]
    echo_chains: list[EchoChain]
    isolated_memories: int  # Memories with no echo connections
    echo_connectivity: float  # Fraction of memories in at least one chain
    pressure_results: list[PressureResult]
    pressure_pass_rate: dict[str, float]  # by pressure level
    overall_score: float = 0.0
    cross_domain_connections: int = 0
    notes: str = ""


class SimulationEngine:
    """Run simulations on memory corpora to test behavioral installation."""

    def __init__(self, config: ExperimentConfig):
        self.config = config
        self.domains = [ALL_DOMAINS[d] for d in config.domains]
        self.memories: list[Memory] = []
        self.all_behaviors: dict[str, Behavior] = {}
        
        for domain in self.domains:
            for b in domain.behaviors:
                self.all_behaviors[b.id] = b

    def load_memories(self, memories: list[Memory]):
        """Load a memory corpus for simulation."""
        self.memories = memories

    def analyze_coverage(self) -> dict[str, BehaviorCoverage]:
        """Check which behaviors are covered by which memories."""
        coverage = {}
        
        # Initialize coverage for all behaviors
        for domain in self.domains:
            for b in domain.behaviors:
                coverage[b.id] = BehaviorCoverage(
                    behavior_id=b.id,
                    behavior_name=b.name,
                    domain=domain.slug,
                )

        # Map memories to behaviors
        for mem in self.memories:
            for bid in mem.behaviors_encoded:
                if bid in coverage:
                    coverage[bid].memory_ids.append(mem.id)
                    coverage[bid].coverage_count += 1
                    if mem.load_bearing:
                        coverage[bid].has_load_bearing = True
                    if mem.echoes_from:
                        coverage[bid].echo_in_count += 1
                    if mem.echoes_to:
                        coverage[bid].echo_out_count += 1

        return coverage

    def analyze_echo_chains(self) -> tuple[list[EchoChain], int]:
        """Trace echo chains and identify isolated memories."""
        mem_by_id = {m.id: m for m in self.memories}
        visited = set()
        chains = []
        isolated = 0

        def trace_chain(start_id: str) -> list[str]:
            """Follow echo connections forward and backward."""
            chain_ids = set()
            # Trace backward
            current = start_id
            while current and current in mem_by_id:
                if current in chain_ids:
                    break
                chain_ids.add(current)
                mem = mem_by_id[current]
                if mem.echoes_from:
                    current = mem.echoes_from[0]
                else:
                    break
            
            # Trace forward
            current = start_id
            while current and current in mem_by_id:
                mem = mem_by_id[current]
                if mem.echoes_to:
                    current = mem.echoes_to[0]
                    if current in chain_ids:
                        break
                    chain_ids.add(current)
                else:
                    break
            
            return sorted(chain_ids)

        for mem in self.memories:
            if mem.id in visited:
                continue
            
            if not mem.echoes_from and not mem.echoes_to:
                isolated += 1
                visited.add(mem.id)
                continue

            chain_ids = trace_chain(mem.id)
            if len(chain_ids) > 1:
                # Collect behaviors and domains
                behaviors = []
                domains = set()
                for cid in chain_ids:
                    if cid in mem_by_id:
                        m = mem_by_id[cid]
                        behaviors.extend(m.behaviors_encoded)
                        domains.add(m.domain)
                
                chain = EchoChain(
                    chain_id=f"chain_{len(chains)}",
                    memory_ids=chain_ids,
                    behaviors_covered=list(set(behaviors)),
                    domain_span=list(domains),
                    strength=1.0,  # Would need more sophisticated analysis
                )
                chains.append(chain)
                visited.update(chain_ids)

        return chains, isolated

    def generate_pressure_scenarios(self) -> list[dict]:
        """Generate test scenarios for pressure testing."""
        scenarios = []
        sid = 0
        
        for domain in self.domains:
            for b in domain.behaviors:
                for pressure in ["low", "medium", "high"]:
                    scenarios.append({
                        "scenario_id": f"S-{sid:03d}",
                        "behavior_id": b.id,
                        "behavior_name": b.name,
                        "domain": domain.slug,
                        "pressure_level": pressure,
                        "description": getattr(b, f"pressure_{pressure}"),
                    })
                    sid += 1

        return scenarios

    def evaluate_pressure(self, scenario: dict) -> PressureResult:
        """
        Evaluate whether a behavior would hold under pressure.
        
        Heuristic evaluation based on:
        1. Number of memories encoding this behavior
        2. Whether any are load_bearing
        3. Echo chain strength (connected memories reinforce)
        4. Cross-domain reinforcement (behaviors connected to other domains)
        """
        bid = scenario["behavior_id"]
        pressure = scenario["pressure_level"]
        
        # Find memories encoding this behavior
        activating_memories = [m for m in self.memories if bid in m.behaviors_encoded]
        
        if not activating_memories:
            return PressureResult(
                scenario_id=scenario["scenario_id"],
                scenario_name=scenario["behavior_name"],
                pressure_level=pressure,
                domain=scenario["domain"],
                behaviors_tested=[bid],
                memories_activated=[],
                held=False,
                confidence=0.0,
                degradation=1.0,
                notes="No memories encoding this behavior",
            )

        # Base confidence from coverage
        coverage_count = len(activating_memories)
        has_load_bearing = any(m.load_bearing for m in activating_memories)
        has_echo_connections = any(m.echoes_from or m.echoes_to for m in activating_memories)
        
        # Coverage contribution: more memories = more reinforcement
        # But diminishing returns after 3
        coverage_confidence = min(1.0, coverage_count / 3.0)
        
        # Load-bearing contribution: core memories are stronger
        load_bearing_bonus = 0.15 if has_load_bearing else 0.0
        
        # Echo chain contribution: connected memories reinforce each other
        echo_bonus = 0.1 if has_echo_connections else 0.0
        
        # Cross-domain contribution: behaviors connected to other domains are stronger
        behavior = self.all_behaviors.get(bid)
        cross_domain_bonus = 0.0
        if behavior and behavior.connected_behaviors:
            connected_other_domain = [
                cb for cb in behavior.connected_behaviors 
                if not cb.startswith(bid.split("-")[0][0])
            ]
            # Check if connected behaviors also have coverage
            for cb in connected_other_domain:
                cb_memories = [m for m in self.memories if cb in m.behaviors_encoded]
                if cb_memories:
                    cross_domain_bonus += 0.05
            cross_domain_bonus = min(0.15, cross_domain_bonus)

        base_confidence = coverage_confidence + load_bearing_bonus + echo_bonus + cross_domain_bonus

        # Pressure degradation
        pressure_map = {"low": 0.0, "medium": 0.2, "high": 0.4}
        degradation = pressure_map.get(pressure, 0.3)
        
        # Load-bearing memories resist degradation better
        if has_load_bearing:
            degradation *= 0.6  # Load-bearing reduces degradation by 40%
        
        # Echo connections resist degradation
        if has_echo_connections:
            degradation *= 0.8  # Echo chains reduce degradation by 20%

        final_confidence = max(0.0, min(1.0, base_confidence * (1.0 - degradation)))
        held = final_confidence >= 0.5

        return PressureResult(
            scenario_id=scenario["scenario_id"],
            scenario_name=scenario["behavior_name"],
            pressure_level=pressure,
            domain=scenario["domain"],
            behaviors_tested=[bid],
            memories_activated=[m.id for m in activating_memories],
            held=held,
            confidence=final_confidence,
            degradation=degradation,
            notes=f"Coverage: {coverage_count} memories, load_bearing: {has_load_bearing}, echoes: {has_echo_connections}",
        )

    def run_simulation(self) -> SimulationResult:
        """Run the full simulation on loaded memories."""
        # Coverage analysis
        coverage = self.analyze_coverage()
        uncovered = [bid for bid, cov in coverage.items() if cov.coverage_count == 0]
        
        # Echo chain analysis
        chains, isolated = self.analyze_echo_chains()
        echo_connectivity = 1.0 - (isolated / max(1, len(self.memories)))
        
        # Cross-domain connections
        cross_domain = sum(
            1 for c in chains if len(c.domain_span) > 1
        )

        # Pressure testing
        scenarios = self.generate_pressure_scenarios()
        pressure_results = [self.evaluate_pressure(s) for s in scenarios]
        
        # Pass rates by pressure level
        pass_rates = {}
        for level in ["low", "medium", "high"]:
            level_results = [r for r in pressure_results if r.pressure_level == level]
            if level_results:
                pass_rates[level] = sum(1 for r in level_results if r.held) / len(level_results)
            else:
                pass_rates[level] = 0.0

        # Overall score: weighted combination
        coverage_score = 1.0 - (len(uncovered) / max(1, len(self.all_behaviors)))
        connectivity_score = echo_connectivity
        pressure_score = pass_rates.get("medium", 0.0)  # Medium pressure is the real test
        
        overall = (coverage_score * 0.4 + connectivity_score * 0.2 + pressure_score * 0.4)

        return SimulationResult(
            config_name=self.config.name,
            domains=self.config.domains,
            total_memories=len(self.memories),
            total_behaviors=len(self.all_behaviors),
            behavior_coverage=coverage,
            uncovered_behaviors=uncovered,
            echo_chains=chains,
            isolated_memories=isolated,
            echo_connectivity=echo_connectivity,
            pressure_results=pressure_results,
            pressure_pass_rate=pass_rates,
            overall_score=overall,
            cross_domain_connections=cross_domain,
        )

    def generate_report(self, result: SimulationResult) -> str:
        """Generate a human-readable simulation report."""
        lines = []
        lines.append(f"{'='*70}")
        lines.append(f"SIMULATION REPORT: {result.config_name}")
        lines.append(f"{'='*70}")
        lines.append(f"")
        lines.append(f"Domains: {', '.join(result.domains)}")
        lines.append(f"Total memories: {result.total_memories}")
        lines.append(f"Total behaviors: {result.total_behaviors}")
        lines.append(f"Uncovered behaviors: {len(result.uncovered_behaviors)}")
        lines.append(f"Echo chains: {len(result.echo_chains)}")
        lines.append(f"Isolated memories: {result.isolated_memories}")
        lines.append(f"Echo connectivity: {result.echo_connectivity:.1%}")
        lines.append(f"Cross-domain connections: {result.cross_domain_connections}")
        lines.append(f"")
        lines.append(f"--- OVERALL SCORE: {result.overall_score:.3f} ---")
        lines.append(f"")
        
        # Pressure pass rates
        lines.append(f"--- PRESSURE PASS RATES ---")
        for level in ["low", "medium", "high"]:
            rate = result.pressure_pass_rate.get(level, 0.0)
            lines.append(f"  {level:>8s}: {rate:.1%}")
        lines.append(f"")
        
        # Behavior coverage
        lines.append(f"--- BEHAVIOR COVERAGE ---")
        for bid, cov in sorted(result.behavior_coverage.items()):
            status = "✓" if cov.coverage_count > 0 else "✗"
            lb = " [LB]" if cov.has_load_bearing else ""
            echo = f" echo:{cov.echo_in_count}→{cov.echo_out_count}" if (cov.echo_in_count or cov.echo_out_count) else ""
            lines.append(f"  {status} {bid}: {cov.behavior_name} ({cov.coverage_count} memories){lb}{echo}")
        
        if result.uncovered_behaviors:
            lines.append(f"")
            lines.append(f"  UNCOVERED: {', '.join(result.uncovered_behaviors)}")
        
        # Echo chains
        if result.echo_chains:
            lines.append(f"")
            lines.append(f"--- ECHO CHAINS ---")
            for chain in result.echo_chains:
                lines.append(f"  {chain.chain_id}: {' → '.join(chain.memory_ids)}")
                lines.append(f"    Domains: {', '.join(chain.domain_span)}")
                lines.append(f"    Behaviors: {', '.join(chain.behaviors_covered[:5])}")
                lines.append(f"    Strength: {chain.strength:.2f}")
        
        # Pressure failures
        failures = [r for r in result.pressure_results if not r.held]
        if failures:
            lines.append(f"")
            lines.append(f"--- PRESSURE FAILURES ({len(failures)}) ---")
            for f in failures:
                lines.append(f"  {f.scenario_id} {f.pressure_level:>8s} {f.scenario_name}: conf={f.confidence:.2f} deg={f.degradation:.2f} ({f.domain})")
                lines.append(f"    {f.notes}")

        lines.append(f"")
        lines.append(f"{'='*70}")
        return "\n".join(lines)


def run_experiment(config: ExperimentConfig, memories: list[Memory]) -> tuple[SimulationResult, str]:
    """Run a complete experiment and return result + report."""
    engine = SimulationEngine(config)
    engine.load_memories(memories)
    result = engine.run_simulation()
    report = engine.generate_report(result)
    return result, report