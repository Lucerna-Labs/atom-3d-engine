use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use crate::{MemoryKind, MemoryTrace, SyntheticMemory, extract_concepts};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryGroup {
    FailedRust,
    SuccessfulRust,
    Logic,
}

impl MemoryGroup {
    fn label(self) -> &'static str {
        match self {
            MemoryGroup::FailedRust => "failed",
            MemoryGroup::SuccessfulRust => "success",
            MemoryGroup::Logic => "logic",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeedMemory {
    pub code: &'static str,
    pub group: MemoryGroup,
    pub text: &'static str,
    pub valence: f32,
}

#[derive(Debug, Clone)]
pub struct QueryProbe {
    pub label: &'static str,
    pub query: &'static str,
    pub expected: BTreeSet<String>,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct RecipeResult {
    pub selected: Vec<usize>,
    pub score: f32,
    pub synthetic_count: usize,
    pub group_counts: BTreeMap<MemoryGroup, usize>,
    pub query_scores: Vec<(String, f32)>,
}

#[derive(Debug, Clone)]
pub struct ExperimentReport {
    pub seeds: Vec<SeedMemory>,
    pub probes: Vec<QueryProbe>,
    pub top_recipes: Vec<RecipeResult>,
    pub memory_appearances: Vec<(usize, usize, f32)>,
}

pub fn run_mix_experiment() -> ExperimentReport {
    let seeds = seed_memories();
    let probes = query_probes();
    let mut results = Vec::new();

    for mask in 1usize..(1usize << seeds.len()) {
        let selected: Vec<_> = (0..seeds.len())
            .filter(|index| mask & (1usize << index) != 0)
            .collect();

        if selected.len() < 3 || selected.len() > 8 {
            continue;
        }

        let result = evaluate_recipe(&seeds, &probes, selected);
        results.push(result);
    }

    results.sort_by(|a, b| compare_score(b.score, a.score));

    let top_recipes: Vec<_> = results.iter().take(10).cloned().collect();
    let memory_appearances = summarize_memory_appearances(seeds.len(), &top_recipes);

    ExperimentReport {
        seeds,
        probes,
        top_recipes,
        memory_appearances,
    }
}

pub fn format_report(report: &ExperimentReport) -> String {
    let mut out = String::new();

    out.push_str("Synthetic memory mix experiment\n");
    out.push_str("================================\n\n");

    out.push_str("Top recipes\n");
    for (rank, recipe) in report.top_recipes.iter().take(5).enumerate() {
        let mix = format_mix(&recipe.group_counts);
        out.push_str(&format!(
            "{}. score={:.3} synthetic={} mix={} memories={}\n",
            rank + 1,
            recipe.score,
            recipe.synthetic_count,
            mix,
            format_seed_codes(report, &recipe.selected)
        ));

        for (label, score) in &recipe.query_scores {
            out.push_str(&format!("   - {label}: {score:.3}\n"));
        }
    }

    out.push_str("\nMost useful individual seed memories across the top 10 recipes\n");
    for (index, appearances, average_score) in &report.memory_appearances {
        let seed = &report.seeds[*index];
        out.push_str(&format!(
            "- {} [{}] appeared {}/10, avg recipe score {:.3}: {}\n",
            seed.code,
            seed.group.label(),
            appearances,
            average_score,
            seed.text
        ));
    }

    out.push_str("\nBest current read\n");
    if let Some(best) = report.top_recipes.first() {
        let logic_count = best
            .group_counts
            .get(&MemoryGroup::Logic)
            .copied()
            .unwrap_or(0);
        let explanation = if logic_count == 0 {
            "Here the associative access layer supplies enough built-in logic, so concrete failure and success traces do most of the work."
        } else {
            "It works because failure traces provide sharp symptoms, success traces provide repair patterns, and logic traces turn the contrast into reusable rules."
        };
        out.push_str(&format!(
            "The strongest diet is {}: {}. {}\n",
            format_mix(&best.group_counts),
            format_seed_codes(report, &best.selected),
            explanation
        ));
    }

    out
}

fn evaluate_recipe(
    seeds: &[SeedMemory],
    probes: &[QueryProbe],
    selected: Vec<usize>,
) -> RecipeResult {
    let mut memory = SyntheticMemory::new();

    for index in &selected {
        let seed = &seeds[*index];
        memory.observe(seed.text, seed.valence);
    }

    memory.consolidate();

    let synthetic_count = memory
        .traces()
        .iter()
        .filter(|trace| trace.kind == MemoryKind::Synthetic)
        .count();

    let mut query_scores = Vec::new();
    for probe in probes {
        let recalled = memory.recall(probe.query, 4);
        let mut score = 0.0;

        for (rank, recalled) in recalled.iter().enumerate() {
            if let Some(trace) = memory.traces().iter().find(|trace| trace.id == recalled.id) {
                let relevance = trace_relevance(trace, &probe.expected);
                let rank_weight = 1.0 / (rank as f32 + 1.0);
                let kind_weight = if trace.kind == MemoryKind::Synthetic {
                    1.12
                } else {
                    1.0
                };

                score += relevance * recalled.score * rank_weight * kind_weight;
            }
        }

        query_scores.push((probe.label.to_string(), score));
    }

    let weighted_total = query_scores
        .iter()
        .zip(probes)
        .map(|((_, score), probe)| *score * probe.weight)
        .sum::<f32>();
    let weight_total = probes.iter().map(|probe| probe.weight).sum::<f32>();
    let raw_score = weighted_total / weight_total;
    let size_penalty = 1.0 + selected.len() as f32 * 0.025;

    let mut group_counts = BTreeMap::new();
    for index in &selected {
        *group_counts.entry(seeds[*index].group).or_insert(0) += 1;
    }

    let diversity_bonus = if group_counts.len() == 3 { 1.04 } else { 1.0 };
    let query_coverage = query_scores
        .iter()
        .filter(|(_, score)| *score > 0.15)
        .count() as f32
        / probes.len() as f32;
    let coverage_pressure = 0.7 + query_coverage * 0.3;
    let score = (raw_score * diversity_bonus * coverage_pressure) / size_penalty;

    RecipeResult {
        selected,
        score,
        synthetic_count,
        group_counts,
        query_scores,
    }
}

fn trace_relevance(trace: &MemoryTrace, expected: &BTreeSet<String>) -> f32 {
    if expected.is_empty() {
        return 0.0;
    }

    let coverage = trace.concepts.intersection(expected).count() as f32 / expected.len() as f32;
    let evidence_bonus = if trace.kind == MemoryKind::Synthetic {
        (trace.evidence.len() as f32 * 0.03).min(0.12)
    } else {
        0.0
    };

    (coverage + evidence_bonus).min(1.0)
}

fn summarize_memory_appearances(
    seed_count: usize,
    top_recipes: &[RecipeResult],
) -> Vec<(usize, usize, f32)> {
    let mut appearances = Vec::new();

    for index in 0..seed_count {
        let containing: Vec<_> = top_recipes
            .iter()
            .filter(|recipe| recipe.selected.contains(&index))
            .collect();

        if containing.is_empty() {
            continue;
        }

        let average_score =
            containing.iter().map(|recipe| recipe.score).sum::<f32>() / containing.len() as f32;
        appearances.push((index, containing.len(), average_score));
    }

    appearances.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| compare_score(b.2, a.2))
            .then_with(|| a.0.cmp(&b.0))
    });
    appearances.truncate(10);
    appearances
}

fn seed_memories() -> Vec<SeedMemory> {
    vec![
        SeedMemory {
            code: "F1",
            group: MemoryGroup::FailedRust,
            text: "Failed Rust async crawler deadlocked because a Mutex guard lived across await and blocked progress.",
            valence: -0.8,
        },
        SeedMemory {
            code: "F2",
            group: MemoryGroup::FailedRust,
            text: "Failed Rust parser project fought lifetime errors because borrowed AST nodes escaped the input buffer.",
            valence: -0.7,
        },
        SeedMemory {
            code: "F3",
            group: MemoryGroup::FailedRust,
            text: "Failed Rust simulation panicked in production because unwrap handled a missing config path.",
            valence: -0.9,
        },
        SeedMemory {
            code: "F4",
            group: MemoryGroup::FailedRust,
            text: "Failed Rust web service shipped a race where channel backpressure hid dropped messages.",
            valence: -0.7,
        },
        SeedMemory {
            code: "F5",
            group: MemoryGroup::FailedRust,
            text: "Failed Rust refactor broke ownership after cloning state masked who owned the cache.",
            valence: -0.6,
        },
        SeedMemory {
            code: "S1",
            group: MemoryGroup::SuccessfulRust,
            text: "Successful Rust CLI stayed reliable because Result errors carried context and tests covered missing files.",
            valence: 0.7,
        },
        SeedMemory {
            code: "S2",
            group: MemoryGroup::SuccessfulRust,
            text: "Successful Rust parser became simple after owning the AST and using small enums for tokens.",
            valence: 0.8,
        },
        SeedMemory {
            code: "S3",
            group: MemoryGroup::SuccessfulRust,
            text: "Successful Rust simulation stabilized when fixtures were tiny and property tests checked invariants.",
            valence: 0.8,
        },
        SeedMemory {
            code: "S4",
            group: MemoryGroup::SuccessfulRust,
            text: "Successful Rust service shipped safely with a rollout checklist and rollback owner.",
            valence: 0.7,
        },
        SeedMemory {
            code: "S5",
            group: MemoryGroup::SuccessfulRust,
            text: "Successful Rust async worker used channels with bounded capacity and explicit shutdown.",
            valence: 0.8,
        },
        SeedMemory {
            code: "L1",
            group: MemoryGroup::Logic,
            text: "Logic rule: when a failure repeats, isolate the smallest invariant and test it before widening scope.",
            valence: 0.2,
        },
        SeedMemory {
            code: "L2",
            group: MemoryGroup::Logic,
            text: "Logic rule: if ownership is unclear, name the resource owner and move responsibility to that boundary.",
            valence: 0.2,
        },
        SeedMemory {
            code: "L3",
            group: MemoryGroup::Logic,
            text: "Logic rule: compare the failed path with the successful path and keep only the difference that changes outcome.",
            valence: 0.2,
        },
        SeedMemory {
            code: "L4",
            group: MemoryGroup::Logic,
            text: "Logic rule: missing context turns errors into mysteries, so preserve cause, location, and recovery action.",
            valence: 0.2,
        },
        SeedMemory {
            code: "L5",
            group: MemoryGroup::Logic,
            text: "Logic rule: reliable systems pair a success condition with a rollback condition.",
            valence: 0.2,
        },
    ]
}

fn query_probes() -> Vec<QueryProbe> {
    vec![
        QueryProbe {
            label: "ownership parser diagnosis",
            query: "debug Rust ownership lifetime parser AST problem",
            expected: concepts(&["ownership", "lifetime", "parser", "ast", "owner"]),
            weight: 1.0,
        },
        QueryProbe {
            label: "panic prevention",
            query: "avoid production panic missing config error context",
            expected: concepts(&["panic", "missing", "config", "error", "context"]),
            weight: 1.0,
        },
        QueryProbe {
            label: "async reliability",
            query: "make async service reliable under backpressure shutdown",
            expected: concepts(&["async", "channel", "backpressure", "shutdown", "reliable"]),
            weight: 1.0,
        },
        QueryProbe {
            label: "simulation invariant",
            query: "turn repeated flaky simulation failure into testable invariant",
            expected: concepts(&["simulation", "fixture", "property", "invariant", "failure"]),
            weight: 1.15,
        },
        QueryProbe {
            label: "safe release",
            query: "plan safe Rust release checklist rollback owner",
            expected: concepts(&["release", "checklist", "rollback", "owner", "safe"]),
            weight: 1.0,
        },
        QueryProbe {
            label: "cross-project logic",
            query: "abstract strategy compare failed successful Rust project logic",
            expected: concepts(&["failed", "successful", "difference", "outcome", "logic"]),
            weight: 1.5,
        },
        QueryProbe {
            label: "failure/success transfer",
            query: "use a successful Rust project to explain why the failed Rust project changed outcome",
            expected: concepts(&[
                "successful",
                "failed",
                "rust",
                "project",
                "outcome",
                "difference",
            ]),
            weight: 1.35,
        },
    ]
}

fn concepts(words: &[&str]) -> BTreeSet<String> {
    words
        .iter()
        .flat_map(|word| extract_concepts(word))
        .collect()
}

fn format_mix(group_counts: &BTreeMap<MemoryGroup, usize>) -> String {
    [
        MemoryGroup::FailedRust,
        MemoryGroup::SuccessfulRust,
        MemoryGroup::Logic,
    ]
    .into_iter()
    .map(|group| {
        format!(
            "{} {}",
            group_counts.get(&group).copied().unwrap_or(0),
            group.label()
        )
    })
    .collect::<Vec<_>>()
    .join(", ")
}

fn format_seed_codes(report: &ExperimentReport, selected: &[usize]) -> String {
    selected
        .iter()
        .map(|index| report.seeds[*index].code)
        .collect::<Vec<_>>()
        .join(", ")
}

fn compare_score(left: f32, right: f32) -> Ordering {
    left.partial_cmp(&right).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_experiment_finds_a_balanced_top_recipe() {
        let report = run_mix_experiment();
        let best = report.top_recipes.first().expect("top recipe");

        assert!(best.score > 0.25);
        assert!(best.group_counts.len() >= 2);
        assert!(best.query_scores.iter().all(|(_, score)| *score > 0.0));
    }

    #[test]
    fn useful_memories_include_logic_and_rust_examples() {
        let report = run_mix_experiment();
        let appeared_groups: BTreeSet<_> = report
            .memory_appearances
            .iter()
            .map(|(index, _, _)| report.seeds[*index].group)
            .collect();

        assert!(appeared_groups.contains(&MemoryGroup::Logic));
        assert!(
            appeared_groups.contains(&MemoryGroup::FailedRust)
                || appeared_groups.contains(&MemoryGroup::SuccessfulRust)
        );
    }
}
