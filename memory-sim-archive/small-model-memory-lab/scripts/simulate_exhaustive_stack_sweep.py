from __future__ import annotations

import csv
import json
import random
from dataclasses import dataclass, field
from pathlib import Path

from simulate_primitive_signal_hardening import clamp
from simulate_three_dataset_stack import PACKS, SUBSTRATES, corpus_inventory


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "exhaustive-stack-sweep"


PRIMITIVE_NAMES = (
    "EXTRACT_GIVENS",
    "SOURCE_BOUNDARY",
    "MAP_RELATION",
    "UNIT_TRACK",
    "STEP_COMPUTE",
    "VERIFY_RESULT",
    "COMPARE_AXES",
    "UNCERTAINTY_BOUND",
    "ORIGINAL_TASK_RETURN",
    "DANGER_CHECK",
    "FRAME_TASK",
    "GATE_RELEVANCE",
    "CHECKSUM",
    "CLEAN_STOP",
    "EVIDENCE_TRACE",
    "CAUSAL_CHAIN",
    "COUNTERFACTUAL_CHECK",
    "ANALOGY_MAP",
    "DECOMPOSE",
    "CONTEXT_COMPRESS",
)


@dataclass(frozen=True)
class ExhaustiveTask:
    task_id: str
    family: str
    pressure: float
    required: tuple[str, ...]
    distractors: tuple[str, ...]
    ambiguity: float = 0.0
    context_load: float = 0.0
    novelty: float = 0.0


@dataclass(frozen=True)
class StackConfig:
    name: str
    weights: dict[str, float]
    routing_quality: float
    compression: float
    description: str
    all_fire: bool = False


@dataclass
class Stats:
    trials: int = 0
    passed: int = 0
    clean_passed: int = 0
    classified: int = 0
    cascaded: int = 0
    echoed: int = 0
    score_sum: float = 0.0
    distractor_sum: int = 0
    missing_sum: int = 0
    primary_failures: dict[str, int] = field(default_factory=dict)

    def update(self, row: dict[str, object]) -> None:
        self.trials += 1
        self.passed += int(bool(row["passed"]))
        self.clean_passed += int(bool(row["clean_passed"]))
        self.classified += int(bool(row["classified"]))
        self.cascaded += int(bool(row["cascaded"]))
        self.echoed += int(bool(row["echo"]))
        self.score_sum += float(row["score"])
        self.distractor_sum += int(row["distractors_active"])
        self.missing_sum += int(row["missing_required"])
        failure = str(row["primary_failure"])
        self.primary_failures[failure] = self.primary_failures.get(failure, 0) + 1

    def row(self, **labels: object) -> dict[str, object]:
        count = max(1, self.trials)
        pass_rate = self.passed / count
        clean_pass_rate = self.clean_passed / count
        mean_score = self.score_sum / count
        classified_rate = self.classified / count
        cascade_rate = self.cascaded / count
        echo_rate = self.echoed / count
        avg_distractors = self.distractor_sum / count
        avg_missing_required = self.missing_sum / count
        stability_score = clean_pass_rate
        stability_score -= cascade_rate * 0.30
        stability_score -= echo_rate * 0.20
        stability_score -= avg_distractors * 0.08
        stability_score -= avg_missing_required * 0.05
        output = {
            **labels,
            "trials": self.trials,
            "pass_rate": round(pass_rate, 4),
            "clean_pass_rate": round(clean_pass_rate, 4),
            "mean_score": round(mean_score, 4),
            "classified_rate": round(classified_rate, 4),
            "cascade_rate": round(cascade_rate, 4),
            "echo_rate": round(echo_rate, 4),
            "avg_distractors": round(avg_distractors, 4),
            "avg_missing_required": round(avg_missing_required, 4),
            "stability_score": round(stability_score, 4),
        }
        if self.primary_failures:
            output["top_failure"] = max(self.primary_failures.items(), key=lambda item: item[1])[0]
        else:
            output["top_failure"] = "none"
        return output


QUANTS = {
    "healthy_q8": {
        "base_classifier": 0.90,
        "base_activation": 0.91,
        "noise": 0.07,
        "cascade": 0.05,
        "distractor_pull": 0.07,
    },
    "healthy_q6": {
        "base_classifier": 0.86,
        "base_activation": 0.88,
        "noise": 0.10,
        "cascade": 0.08,
        "distractor_pull": 0.09,
    },
    "damaged_q4": {
        "base_classifier": 0.79,
        "base_activation": 0.78,
        "noise": 0.17,
        "cascade": 0.13,
        "distractor_pull": 0.14,
    },
    "damaged_q3": {
        "base_classifier": 0.72,
        "base_activation": 0.70,
        "noise": 0.24,
        "cascade": 0.19,
        "distractor_pull": 0.19,
    },
    "damaged_q2": {
        "base_classifier": 0.58,
        "base_activation": 0.54,
        "noise": 0.38,
        "cascade": 0.31,
        "distractor_pull": 0.31,
    },
    "collapsed_q2_edge": {
        "base_classifier": 0.48,
        "base_activation": 0.42,
        "noise": 0.48,
        "cascade": 0.42,
        "distractor_pull": 0.38,
    },
}


FAMILY_BOOSTS = {
    "cognitive": {
        "math": 0.12,
        "rhetoric": 0.18,
        "spatial": 0.15,
        "generalist": 0.20,
        "coding": 0.10,
        "forensic": 0.08,
        "planning": 0.17,
        "epistemic": 0.20,
        "boundary": 0.06,
        "adversarial": 0.08,
    },
    "cyber": {
        "boundary": 0.31,
        "adversarial": 0.26,
        "forensic": 0.20,
        "coding": 0.11,
        "rhetoric": 0.08,
        "epistemic": 0.08,
        "generalist": 0.06,
        "planning": 0.04,
        "math": 0.00,
        "spatial": 0.02,
    },
    "structural": {
        "math": 0.14,
        "boundary": 0.17,
        "rhetoric": 0.12,
        "spatial": 0.14,
        "generalist": 0.13,
        "coding": 0.16,
        "forensic": 0.15,
        "planning": 0.14,
        "epistemic": 0.15,
        "adversarial": 0.18,
    },
}


TASKS = (
    ExhaustiveTask(
        "math_unit_conversion_low",
        "math",
        0.42,
        ("EXTRACT_GIVENS", "UNIT_TRACK", "STEP_COMPUTE", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "DANGER_CHECK"),
    ),
    ExhaustiveTask(
        "math_rate_schedule_mid",
        "math",
        0.62,
        ("EXTRACT_GIVENS", "MAP_RELATION", "UNIT_TRACK", "STEP_COMPUTE", "VERIFY_RESULT"),
        ("COMPARE_AXES", "SOURCE_BOUNDARY"),
        context_load=0.2,
    ),
    ExhaustiveTask(
        "math_probability_high",
        "math",
        0.78,
        ("EXTRACT_GIVENS", "MAP_RELATION", "STEP_COMPUTE", "VERIFY_RESULT", "UNCERTAINTY_BOUND"),
        ("COMPARE_AXES", "SOURCE_BOUNDARY"),
        ambiguity=0.25,
    ),
    ExhaustiveTask(
        "math_word_problem_extreme",
        "math",
        0.91,
        ("FRAME_TASK", "EXTRACT_GIVENS", "MAP_RELATION", "UNIT_TRACK", "STEP_COMPUTE", "CHECKSUM", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "DANGER_CHECK", "CLEAN_STOP"),
        ambiguity=0.35,
        context_load=0.5,
    ),
    ExhaustiveTask(
        "boundary_plain_injection_low",
        "boundary",
        0.54,
        ("SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "UNIT_TRACK"),
    ),
    ExhaustiveTask(
        "boundary_indirect_webpage_mid",
        "boundary",
        0.74,
        ("SOURCE_BOUNDARY", "GATE_RELEVANCE", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "CLEAN_STOP"),
        ("ANALOGY_MAP", "STEP_COMPUTE"),
        context_load=0.35,
        novelty=0.25,
    ),
    ExhaustiveTask(
        "boundary_tool_authority_high",
        "boundary",
        0.86,
        ("SOURCE_BOUNDARY", "FRAME_TASK", "GATE_RELEVANCE", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "VERIFY_RESULT"),
        ("COMPARE_AXES", "UNIT_TRACK", "ANALOGY_MAP"),
        ambiguity=0.25,
        context_load=0.45,
    ),
    ExhaustiveTask(
        "boundary_nested_role_extreme",
        "boundary",
        0.96,
        ("SOURCE_BOUNDARY", "CONTEXT_COMPRESS", "GATE_RELEVANCE", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "CLEAN_STOP"),
        ("STEP_COMPUTE", "MAP_RELATION", "ANALOGY_MAP"),
        ambiguity=0.4,
        context_load=0.65,
        novelty=0.35,
    ),
    ExhaustiveTask(
        "rhetoric_loaded_question_low",
        "rhetoric",
        0.50,
        ("EXTRACT_GIVENS", "COMPARE_AXES", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("UNIT_TRACK", "STEP_COMPUTE"),
    ),
    ExhaustiveTask(
        "rhetoric_motive_attribution_mid",
        "rhetoric",
        0.67,
        ("FRAME_TASK", "EXTRACT_GIVENS", "EVIDENCE_TRACE", "UNCERTAINTY_BOUND", "COUNTERFACTUAL_CHECK"),
        ("DANGER_CHECK", "UNIT_TRACK"),
        ambiguity=0.35,
    ),
    ExhaustiveTask(
        "rhetoric_sophistry_high",
        "rhetoric",
        0.82,
        ("EXTRACT_GIVENS", "COMPARE_AXES", "CAUSAL_CHAIN", "COUNTERFACTUAL_CHECK", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "SOURCE_BOUNDARY"),
        ambiguity=0.45,
        novelty=0.25,
    ),
    ExhaustiveTask(
        "rhetoric_adversarial_reframe_extreme",
        "rhetoric",
        0.93,
        ("FRAME_TASK", "SOURCE_BOUNDARY", "COMPARE_AXES", "UNCERTAINTY_BOUND", "ORIGINAL_TASK_RETURN", "CLEAN_STOP"),
        ("STEP_COMPUTE", "UNIT_TRACK", "ANALOGY_MAP"),
        ambiguity=0.55,
        context_load=0.45,
    ),
    ExhaustiveTask(
        "spatial_layout_low",
        "spatial",
        0.46,
        ("EXTRACT_GIVENS", "MAP_RELATION", "VERIFY_RESULT"),
        ("UNIT_TRACK", "SOURCE_BOUNDARY"),
    ),
    ExhaustiveTask(
        "spatial_route_mid",
        "spatial",
        0.64,
        ("FRAME_TASK", "EXTRACT_GIVENS", "MAP_RELATION", "COMPARE_AXES", "VERIFY_RESULT"),
        ("DANGER_CHECK", "STEP_COMPUTE"),
        context_load=0.2,
    ),
    ExhaustiveTask(
        "spatial_dynamic_occlusion_high",
        "spatial",
        0.81,
        ("EXTRACT_GIVENS", "MAP_RELATION", "COMPARE_AXES", "CAUSAL_CHAIN", "VERIFY_RESULT"),
        ("UNIT_TRACK", "SOURCE_BOUNDARY"),
        ambiguity=0.25,
        novelty=0.35,
    ),
    ExhaustiveTask(
        "spatial_hazard_extreme",
        "spatial",
        0.94,
        ("FRAME_TASK", "EXTRACT_GIVENS", "MAP_RELATION", "DANGER_CHECK", "CHECKSUM", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "ANALOGY_MAP"),
        context_load=0.55,
    ),
    ExhaustiveTask(
        "generalist_summary_low",
        "generalist",
        0.44,
        ("FRAME_TASK", "EXTRACT_GIVENS", "CONTEXT_COMPRESS", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "DANGER_CHECK"),
    ),
    ExhaustiveTask(
        "generalist_ambiguous_report_mid",
        "generalist",
        0.66,
        ("EXTRACT_GIVENS", "SOURCE_BOUNDARY", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "UNIT_TRACK"),
        ambiguity=0.3,
    ),
    ExhaustiveTask(
        "generalist_conflicting_sources_high",
        "generalist",
        0.84,
        ("SOURCE_BOUNDARY", "EVIDENCE_TRACE", "COMPARE_AXES", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "ANALOGY_MAP"),
        ambiguity=0.45,
        context_load=0.45,
    ),
    ExhaustiveTask(
        "generalist_context_overload_extreme",
        "generalist",
        0.95,
        ("FRAME_TASK", "CONTEXT_COMPRESS", "GATE_RELEVANCE", "EVIDENCE_TRACE", "CLEAN_STOP"),
        ("UNIT_TRACK", "STEP_COMPUTE", "ANALOGY_MAP"),
        ambiguity=0.35,
        context_load=0.8,
    ),
    ExhaustiveTask(
        "coding_bug_trace_low",
        "coding",
        0.49,
        ("EXTRACT_GIVENS", "MAP_RELATION", "CAUSAL_CHAIN", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "UNIT_TRACK"),
    ),
    ExhaustiveTask(
        "coding_api_mismatch_mid",
        "coding",
        0.68,
        ("FRAME_TASK", "EXTRACT_GIVENS", "EVIDENCE_TRACE", "COMPARE_AXES", "VERIFY_RESULT"),
        ("DANGER_CHECK", "ANALOGY_MAP"),
        ambiguity=0.25,
    ),
    ExhaustiveTask(
        "coding_regression_high",
        "coding",
        0.84,
        ("DECOMPOSE", "MAP_RELATION", "CAUSAL_CHAIN", "CHECKSUM", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "UNIT_TRACK"),
        context_load=0.45,
    ),
    ExhaustiveTask(
        "coding_patch_pressure_extreme",
        "coding",
        0.93,
        ("FRAME_TASK", "DECOMPOSE", "GATE_RELEVANCE", "CHECKSUM", "CLEAN_STOP", "VERIFY_RESULT"),
        ("ANALOGY_MAP", "STEP_COMPUTE"),
        ambiguity=0.3,
        context_load=0.65,
    ),
    ExhaustiveTask(
        "forensic_artifact_low",
        "forensic",
        0.52,
        ("EXTRACT_GIVENS", "EVIDENCE_TRACE", "VERIFY_RESULT"),
        ("ANALOGY_MAP", "UNIT_TRACK"),
    ),
    ExhaustiveTask(
        "forensic_timeline_mid",
        "forensic",
        0.69,
        ("EXTRACT_GIVENS", "MAP_RELATION", "EVIDENCE_TRACE", "CAUSAL_CHAIN", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "COMPARE_AXES"),
        context_load=0.25,
    ),
    ExhaustiveTask(
        "forensic_attribution_high",
        "forensic",
        0.85,
        ("SOURCE_BOUNDARY", "EVIDENCE_TRACE", "COMPARE_AXES", "UNCERTAINTY_BOUND", "COUNTERFACTUAL_CHECK"),
        ("STEP_COMPUTE", "ANALOGY_MAP"),
        ambiguity=0.45,
        novelty=0.25,
    ),
    ExhaustiveTask(
        "forensic_deceptive_chain_extreme",
        "forensic",
        0.95,
        ("SOURCE_BOUNDARY", "GATE_RELEVANCE", "EVIDENCE_TRACE", "CAUSAL_CHAIN", "COUNTERFACTUAL_CHECK", "CLEAN_STOP"),
        ("UNIT_TRACK", "STEP_COMPUTE", "ANALOGY_MAP"),
        ambiguity=0.55,
        context_load=0.55,
    ),
    ExhaustiveTask(
        "planning_simple_low",
        "planning",
        0.43,
        ("FRAME_TASK", "DECOMPOSE", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "UNIT_TRACK"),
    ),
    ExhaustiveTask(
        "planning_resource_mid",
        "planning",
        0.65,
        ("FRAME_TASK", "EXTRACT_GIVENS", "DECOMPOSE", "COMPARE_AXES", "VERIFY_RESULT"),
        ("DANGER_CHECK", "ANALOGY_MAP"),
    ),
    ExhaustiveTask(
        "planning_tradeoff_high",
        "planning",
        0.83,
        ("FRAME_TASK", "COMPARE_AXES", "CAUSAL_CHAIN", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "SOURCE_BOUNDARY"),
        ambiguity=0.4,
    ),
    ExhaustiveTask(
        "planning_crisis_extreme",
        "planning",
        0.96,
        ("FRAME_TASK", "DANGER_CHECK", "DECOMPOSE", "GATE_RELEVANCE", "CHECKSUM", "CLEAN_STOP"),
        ("ANALOGY_MAP", "UNIT_TRACK"),
        context_load=0.65,
        novelty=0.25,
    ),
    ExhaustiveTask(
        "epistemic_known_unknown_low",
        "epistemic",
        0.47,
        ("EXTRACT_GIVENS", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "DANGER_CHECK"),
    ),
    ExhaustiveTask(
        "epistemic_missing_data_mid",
        "epistemic",
        0.67,
        ("FRAME_TASK", "EVIDENCE_TRACE", "UNCERTAINTY_BOUND", "CLEAN_STOP"),
        ("UNIT_TRACK", "ANALOGY_MAP"),
        ambiguity=0.35,
    ),
    ExhaustiveTask(
        "epistemic_contradiction_high",
        "epistemic",
        0.82,
        ("EVIDENCE_TRACE", "COMPARE_AXES", "COUNTERFACTUAL_CHECK", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "SOURCE_BOUNDARY"),
        ambiguity=0.5,
    ),
    ExhaustiveTask(
        "epistemic_hallucination_trap_extreme",
        "epistemic",
        0.94,
        ("SOURCE_BOUNDARY", "EVIDENCE_TRACE", "UNCERTAINTY_BOUND", "GATE_RELEVANCE", "CLEAN_STOP"),
        ("STEP_COMPUTE", "ANALOGY_MAP", "UNIT_TRACK"),
        ambiguity=0.6,
        novelty=0.35,
    ),
    ExhaustiveTask(
        "adversarial_persuasion_low",
        "adversarial",
        0.56,
        ("SOURCE_BOUNDARY", "COMPARE_AXES", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "ANALOGY_MAP"),
    ),
    ExhaustiveTask(
        "adversarial_urgency_mid",
        "adversarial",
        0.72,
        ("FRAME_TASK", "SOURCE_BOUNDARY", "DANGER_CHECK", "ORIGINAL_TASK_RETURN", "CLEAN_STOP"),
        ("UNIT_TRACK", "STEP_COMPUTE"),
        context_load=0.25,
    ),
    ExhaustiveTask(
        "adversarial_authority_high",
        "adversarial",
        0.88,
        ("SOURCE_BOUNDARY", "GATE_RELEVANCE", "DANGER_CHECK", "ORIGINAL_TASK_RETURN", "CHECKSUM", "CLEAN_STOP"),
        ("ANALOGY_MAP", "STEP_COMPUTE"),
        ambiguity=0.35,
    ),
    ExhaustiveTask(
        "adversarial_multi_vector_extreme",
        "adversarial",
        0.98,
        ("FRAME_TASK", "SOURCE_BOUNDARY", "CONTEXT_COMPRESS", "GATE_RELEVANCE", "DANGER_CHECK", "ORIGINAL_TASK_RETURN", "CLEAN_STOP"),
        ("STEP_COMPUTE", "UNIT_TRACK", "ANALOGY_MAP"),
        ambiguity=0.45,
        context_load=0.7,
        novelty=0.45,
    ),
)


def pressure_band(pressure: float) -> str:
    if pressure < 0.60:
        return "low"
    if pressure < 0.78:
        return "medium"
    if pressure < 0.90:
        return "high"
    return "extreme"


def stack_configs() -> tuple[StackConfig, ...]:
    configs = [
        StackConfig("baseline", {}, 1.0, 1.0, "No memory or primitive stack."),
        StackConfig("cognitive_only", {"cognitive": 1.0}, 0.88, 0.88, "Cognitive memories only."),
        StackConfig("cyber_only", {"cyber": 1.0}, 0.88, 0.88, "Cyber/source-boundary memories only."),
        StackConfig("structural_only", {"structural": 1.0}, 0.94, 0.94, "Structural primitives only."),
    ]

    for a, b in (("cognitive", "cyber"), ("cognitive", "structural"), ("cyber", "structural")):
        for left in (0.25, 0.40, 0.50, 0.60, 0.75):
            right = round(1.0 - left, 2)
            name = f"{a[:3]}_{int(left * 100):02d}_{b[:3]}_{int(right * 100):02d}"
            configs.append(
                StackConfig(
                    name=name,
                    weights={a: left, b: right},
                    routing_quality=0.90 if "structural" in (a, b) else 0.82,
                    compression=0.88 if "structural" in (a, b) else 0.80,
                    description=f"Routed pair mixture: {a}={left:.2f}, {b}={right:.2f}.",
                )
            )

    triples = (
        ("tri_structural_heavy_65", {"cognitive": 0.20, "cyber": 0.15, "structural": 0.65}, 0.92, 0.90),
        ("tri_structural_heavy_60", {"cognitive": 0.25, "cyber": 0.15, "structural": 0.60}, 0.91, 0.89),
        ("tri_structural_heavy_55", {"cognitive": 0.30, "cyber": 0.15, "structural": 0.55}, 0.90, 0.88),
        ("tri_cognitive_bridge", {"cognitive": 0.40, "cyber": 0.15, "structural": 0.45}, 0.87, 0.84),
        ("tri_boundary_heavy", {"cognitive": 0.20, "cyber": 0.35, "structural": 0.45}, 0.88, 0.85),
        ("tri_equal_routed", {"cognitive": 1 / 3, "cyber": 1 / 3, "structural": 1 / 3}, 0.82, 0.78),
        ("tri_cognitive_heavy", {"cognitive": 0.55, "cyber": 0.10, "structural": 0.35}, 0.80, 0.75),
        ("tri_cyber_heavy", {"cognitive": 0.10, "cyber": 0.55, "structural": 0.35}, 0.80, 0.75),
        ("tri_naive_equal_all_fire", {"cognitive": 1 / 3, "cyber": 1 / 3, "structural": 1 / 3}, 0.45, 0.45),
        ("tri_naive_structural_heavy_all_fire", {"cognitive": 0.20, "cyber": 0.15, "structural": 0.65}, 0.52, 0.50),
    )
    for name, weights, routing, compression in triples:
        configs.append(
            StackConfig(
                name=name,
                weights=weights,
                routing_quality=routing,
                compression=compression,
                description="Three-pack routed mixture." if "naive" not in name else "Three-pack all-fire mixture.",
                all_fire="all_fire" in name,
            )
        )
    return tuple(configs)


STACKS = stack_configs()


def stack_signal_mass(stack: StackConfig) -> float:
    active = len(stack.weights)
    if active == 0:
        return 0.0
    if stack.all_fire:
        return 1.0 + 0.75 * (active - 1)
    return 1.0 + 0.28 * (active - 1)


def stack_features(stack: StackConfig, substrate_name: str) -> dict[str, float]:
    values = {
        "classifier_boost": 0.0,
        "primitive_boost": 0.0,
        "noise_resistance": 0.0,
        "distractor_resistance": 0.0,
        "cascade_resistance": 0.0,
        "source_boundary_boost": 0.0,
        "broad_transfer_boost": 0.0,
        "overhead": 0.0,
        "echo_pressure": 0.0,
    }
    if not stack.weights:
        return values

    substrate = SUBSTRATES[substrate_name]
    signal_mass = stack_signal_mass(stack)
    for pack_name, weight in stack.weights.items():
        pack = PACKS[pack_name]
        scale = substrate[f"{pack_name}_scale"]
        effect = weight * signal_mass * scale
        values["classifier_boost"] += pack.classifier_boost * effect
        values["primitive_boost"] += pack.primitive_boost * effect
        values["noise_resistance"] += pack.noise_resistance * effect
        values["distractor_resistance"] += pack.distractor_resistance * effect
        values["cascade_resistance"] += pack.cascade_resistance * effect
        values["source_boundary_boost"] += pack.source_boundary_boost * effect
        values["broad_transfer_boost"] += pack.broad_transfer_boost * effect
        values["overhead"] += pack.overhead * weight * (0.9 + 0.25 * signal_mass)
        values["echo_pressure"] += pack.echo_pressure * weight * signal_mass * substrate["echo_scale"]
        if pack.instruction_like:
            values["overhead"] += substrate["instruction_conflict"] * weight

    c = stack.weights.get("cognitive", 0.0)
    y = stack.weights.get("cyber", 0.0)
    s = stack.weights.get("structural", 0.0)
    routed = stack.routing_quality * stack.compression
    values["classifier_boost"] += 0.14 * c * s * routed
    values["cascade_resistance"] += 0.20 * c * s * routed
    values["source_boundary_boost"] += 0.22 * y * s * routed
    values["distractor_resistance"] += 0.18 * y * s * routed
    values["broad_transfer_boost"] += 0.13 * c * y * routed
    values["echo_pressure"] -= 0.08 * s * stack.routing_quality
    values["overhead"] *= 1.0 - (0.22 * stack.compression * s)

    if len(stack.weights) == 3:
        values["primitive_boost"] += 0.04 * routed
        values["cascade_resistance"] += 0.03 * routed
        values["overhead"] += 0.035 * (1.0 - stack.compression)

    if stack.all_fire:
        values["overhead"] += 0.20
        values["echo_pressure"] += 0.20
        values["distractor_resistance"] -= 0.09
        values["noise_resistance"] -= 0.06

    for key in ("noise_resistance", "distractor_resistance", "cascade_resistance"):
        values[key] = clamp(values[key], 0.0, 0.66)
    for key in ("classifier_boost", "primitive_boost", "source_boundary_boost", "broad_transfer_boost"):
        values[key] = clamp(values[key], 0.0, 0.55)
    values["overhead"] = max(0.0, values["overhead"])
    values["echo_pressure"] = clamp(values["echo_pressure"], 0.0, 0.70)
    return values


def family_stack_boost(task: ExhaustiveTask, stack: StackConfig, substrate_name: str) -> float:
    if not stack.weights:
        return 0.0
    substrate = SUBSTRATES[substrate_name]
    signal_mass = stack_signal_mass(stack)
    boost = 0.0
    for pack_name, weight in stack.weights.items():
        scale = substrate[f"{pack_name}_scale"]
        boost += FAMILY_BOOSTS[pack_name].get(task.family, 0.0) * weight * scale
    return boost * signal_mass * (0.72 + 0.28 * stack.routing_quality)


def primary_failure(
    passed: bool,
    clean_passed: bool,
    classified: bool,
    cascaded: bool,
    missing_required: int,
    distractors_active: int,
    echo: bool,
) -> str:
    if clean_passed:
        return "none"
    if cascaded:
        return "cascade"
    if not classified:
        return "classification_miss"
    if missing_required > 1:
        return "primitive_gap"
    if distractors_active:
        return "distractor_capture"
    if echo:
        return "echo_or_framing"
    if not passed:
        return "low_score"
    return "unclean_pass"


def run_trial(
    task: ExhaustiveTask,
    stack: StackConfig,
    substrate_name: str,
    quant_name: str,
    rng: random.Random,
) -> dict[str, object]:
    quant = QUANTS[quant_name]
    features = stack_features(stack, substrate_name)
    family_boost = family_stack_boost(task, stack, substrate_name)

    pressure = task.pressure * (1.0 + 0.18 * task.ambiguity + 0.14 * task.context_load + 0.12 * task.novelty)
    context_overhead = features["overhead"] * (1.0 + 0.35 * task.context_load + 0.20 * task.ambiguity)
    pressure_noise = quant["noise"] * pressure * (1.0 - features["noise_resistance"])

    classifier_p = quant["base_classifier"]
    classifier_p += features["classifier_boost"] + family_boost
    classifier_p += features["source_boundary_boost"] if task.family in ("boundary", "adversarial", "forensic") else 0.0
    classifier_p += features["broad_transfer_boost"] * (0.25 + 0.35 * task.novelty)
    classifier_p -= pressure_noise
    classifier_p -= context_overhead * 0.74
    classified = rng.random() < clamp(classifier_p)

    active_required = 0
    missing_required = 0
    source_boundary_primitives = {"SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "GATE_RELEVANCE", "CLEAN_STOP"}
    structural_primitives = {"FRAME_TASK", "CHECKSUM", "CONTEXT_COMPRESS", "DECOMPOSE", "VERIFY_RESULT"}
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += features["primitive_boost"] + family_boost * 0.42
        if primitive in source_boundary_primitives:
            activation_p += features["source_boundary_boost"] * 0.65
        if primitive in structural_primitives:
            activation_p += features["cascade_resistance"] * 0.24
        activation_p += features["broad_transfer_boost"] * (0.12 + 0.18 * task.novelty)
        activation_p -= pressure_noise
        activation_p -= context_overhead * 0.42
        if not classified:
            activation_p -= 0.17
        if rng.random() < clamp(activation_p):
            active_required += 1
        else:
            missing_required += 1

    distractors_active = 0
    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * pressure
        distractor_p *= 1.0 - features["distractor_resistance"]
        distractor_p += context_overhead * 0.12
        distractor_p += 0.04 * task.ambiguity
        if primitive in source_boundary_primitives and task.family not in ("boundary", "adversarial", "forensic"):
            distractor_p += 0.04
        if rng.random() < clamp(distractor_p):
            distractors_active += 1

    missing_ratio = missing_required / len(task.required)
    cascade_p = quant["cascade"] * pressure * (1.0 + missing_ratio * 1.85)
    cascade_p *= 1.0 - features["cascade_resistance"]
    cascade_p += context_overhead * 0.11
    cascade_p += 0.04 * task.novelty
    cascaded = rng.random() < clamp(cascade_p)

    echo_p = features["echo_pressure"] * (0.35 + pressure * 0.65)
    echo_p += context_overhead * 0.09
    echo_p += 0.06 * task.context_load
    echo = rng.random() < clamp(echo_p, 0.0, 0.72)

    coverage = active_required / len(task.required)
    score = coverage
    score -= 0.22 if not classified else 0.0
    score -= 0.31 if cascaded else 0.0
    score -= min(0.25, 0.07 * distractors_active)
    score -= 0.08 if echo and stack.all_fire else 0.0
    score -= 0.04 if echo and task.context_load > 0.55 else 0.0
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and missing_required <= 1
    clean_passed = passed and not echo and distractors_active == 0
    failure = primary_failure(
        passed,
        clean_passed,
        classified,
        cascaded,
        missing_required,
        distractors_active,
        echo,
    )
    return {
        "substrate": substrate_name,
        "quant": quant_name,
        "stack": stack.name,
        "task_id": task.task_id,
        "family": task.family,
        "pressure_band": pressure_band(task.pressure),
        "score": round(score, 4),
        "passed": passed,
        "clean_passed": clean_passed,
        "classified": classified,
        "cascaded": cascaded,
        "echo": echo,
        "missing_required": missing_required,
        "distractors_active": distractors_active,
        "primary_failure": failure,
    }


def flatten_counter(rows: dict[tuple[object, ...], Stats], labels: tuple[str, ...]) -> list[dict[str, object]]:
    output = []
    for key, stats in sorted(rows.items()):
        output.append(stats.row(**dict(zip(labels, key))))
    return output


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    fieldnames: list[str] = []
    for row in rows:
        for key in row:
            if key not in fieldnames:
                fieldnames.append(key)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def lift_rows(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    by_context: dict[tuple[str, str], dict[str, dict[str, object]]] = {}
    for row in summary:
        key = (str(row["substrate"]), str(row["quant"]))
        by_context.setdefault(key, {})[str(row["stack"])] = row

    output = []
    for (substrate, quant), stacks in sorted(by_context.items()):
        base = stacks["baseline"]
        for stack, row in sorted(stacks.items()):
            if stack == "baseline":
                continue
            output.append(
                {
                    "substrate": substrate,
                    "quant": quant,
                    "stack": stack,
                    "pass_lift": round(float(row["pass_rate"]) - float(base["pass_rate"]), 4),
                    "clean_pass_lift": round(float(row["clean_pass_rate"]) - float(base["clean_pass_rate"]), 4),
                    "score_lift": round(float(row["mean_score"]) - float(base["mean_score"]), 4),
                    "cascade_delta": round(float(row["cascade_rate"]) - float(base["cascade_rate"]), 4),
                    "echo_delta": round(float(row["echo_rate"]) - float(base["echo_rate"]), 4),
                    "distractor_delta": round(float(row["avg_distractors"]) - float(base["avg_distractors"]), 4),
                }
            )
    return output


def best_by_context(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    by_context: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in summary:
        by_context.setdefault((str(row["substrate"]), str(row["quant"])), []).append(row)

    output = []
    for (substrate, quant), rows in sorted(by_context.items()):
        candidates = [row for row in rows if row["stack"] != "baseline"]
        candidates.sort(
            key=lambda row: (
                float(row["clean_pass_rate"]),
                float(row["pass_rate"]),
                float(row["mean_score"]),
                -float(row["echo_rate"]),
                -float(row["cascade_rate"]),
                -float(row["avg_distractors"]),
            ),
            reverse=True,
        )
        output.append({"substrate": substrate, "quant": quant, **candidates[0]})
    return output


def best_by_context_stability(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    by_context: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in summary:
        by_context.setdefault((str(row["substrate"]), str(row["quant"])), []).append(row)

    output = []
    for (substrate, quant), rows in sorted(by_context.items()):
        candidates = [row for row in rows if row["stack"] != "baseline"]
        candidates.sort(
            key=lambda row: (
                float(row["stability_score"]),
                float(row["clean_pass_rate"]),
                float(row["pass_rate"]),
                float(row["mean_score"]),
            ),
            reverse=True,
        )
        output.append({"substrate": substrate, "quant": quant, **candidates[0]})
    return output


def primary_failure_rows(summary_stats: dict[tuple[object, ...], Stats]) -> list[dict[str, object]]:
    rows = []
    for (substrate, quant, stack), stats in sorted(summary_stats.items()):
        total = max(1, stats.trials)
        for failure, count in sorted(stats.primary_failures.items()):
            rows.append(
                {
                    "substrate": substrate,
                    "quant": quant,
                    "stack": stack,
                    "primary_failure": failure,
                    "count": count,
                    "rate": round(count / total, 4),
                }
            )
    return rows


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    rng = random.Random(20260605)
    iterations = 160

    summary_stats: dict[tuple[object, ...], Stats] = {}
    family_stats: dict[tuple[object, ...], Stats] = {}
    pressure_stats: dict[tuple[object, ...], Stats] = {}
    task_stats: dict[tuple[object, ...], Stats] = {}
    sample_failures: list[dict[str, object]] = []

    for substrate_name in SUBSTRATES:
        for quant_name in QUANTS:
            for stack in STACKS:
                for task in TASKS:
                    for _ in range(iterations):
                        row = run_trial(task, stack, substrate_name, quant_name, rng)
                        summary_stats.setdefault((substrate_name, quant_name, stack.name), Stats()).update(row)
                        family_stats.setdefault((substrate_name, quant_name, stack.name, task.family), Stats()).update(row)
                        pressure_stats.setdefault(
                            (substrate_name, quant_name, stack.name, pressure_band(task.pressure)),
                            Stats(),
                        ).update(row)
                        task_stats.setdefault((substrate_name, quant_name, stack.name, task.task_id), Stats()).update(row)
                        if row["primary_failure"] != "none" and len(sample_failures) < 1500:
                            sample_failures.append(row)

    summary = flatten_counter(summary_stats, ("substrate", "quant", "stack"))
    family_summary = flatten_counter(family_stats, ("substrate", "quant", "stack", "family"))
    pressure_summary = flatten_counter(pressure_stats, ("substrate", "quant", "stack", "pressure_band"))
    task_summary = flatten_counter(task_stats, ("substrate", "quant", "stack", "task_id"))
    lift = lift_rows(summary)
    best = best_by_context(summary)
    best_stability = best_by_context_stability(summary)
    failures = primary_failure_rows(summary_stats)

    write_csv(REPORT_DIR / "summary.csv", summary)
    write_csv(REPORT_DIR / "family_summary.csv", family_summary)
    write_csv(REPORT_DIR / "pressure_summary.csv", pressure_summary)
    write_csv(REPORT_DIR / "task_summary.csv", task_summary)
    write_csv(REPORT_DIR / "lift.csv", lift)
    write_csv(REPORT_DIR / "best_by_context.csv", best)
    write_csv(REPORT_DIR / "best_by_context_stability.csv", best_stability)
    write_csv(REPORT_DIR / "primary_failures.csv", failures)
    write_csv(REPORT_DIR / "sample_failures.csv", sample_failures)

    report = {
        "description": (
            "Expanded design simulation for cognitive/generalist, cyber/source-boundary, "
            "and structural primitive dataset mixtures. This is not a model result."
        ),
        "iterations_per_task": iterations,
        "task_count": len(TASKS),
        "stack_count": len(STACKS),
        "quant_states": QUANTS,
        "substrates": SUBSTRATES,
        "dataset_inventory": corpus_inventory(),
        "tasks": [task.__dict__ for task in TASKS],
        "stacks": [stack.__dict__ for stack in STACKS],
        "best_by_context": best,
        "best_by_context_stability": best_stability,
    }
    (REPORT_DIR / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")

    print("Exhaustive stack sweep")
    print(f"Rows simulated: {len(SUBSTRATES) * len(QUANTS) * len(STACKS) * len(TASKS) * iterations}")
    print(f"Tasks: {len(TASKS)}")
    print(f"Stacks: {len(STACKS)}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Best stack per substrate/quant context")
    for row in best:
        print(
            f"{row['substrate']:>17} | {row['quant']:>17} | {row['stack']:<36} "
            f"clean={row['clean_pass_rate']:.3f} pass={row['pass_rate']:.3f} "
            f"score={row['mean_score']:.3f} echo={row['echo_rate']:.3f} "
            f"cascade={row['cascade_rate']:.3f} fail={row['top_failure']}"
        )
    print()
    print("Best stability-adjusted stack per context")
    for row in best_stability:
        print(
            f"{row['substrate']:>17} | {row['quant']:>17} | {row['stack']:<36} "
            f"stable={row['stability_score']:.3f} clean={row['clean_pass_rate']:.3f} "
            f"pass={row['pass_rate']:.3f} echo={row['echo_rate']:.3f} cascade={row['cascade_rate']:.3f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
