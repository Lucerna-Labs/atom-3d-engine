use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub mod experiment;
pub mod memory_generator;
pub mod prompt_injection;

#[derive(Debug, Clone)]
pub struct MemoryTrace {
    pub id: usize,
    pub text: String,
    pub concepts: BTreeSet<String>,
    pub valence: f32,
    pub created_at: u32,
    pub last_recalled_at: u32,
    pub strength: f32,
    pub kind: MemoryKind,
    pub evidence: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryKind {
    Episodic,
    Synthetic,
    False,
    Calibration,
    Knowledge,
    Procedure,
    CrossDomain,
    Fortifying,
}

#[derive(Debug, Clone)]
pub struct RecalledMemory {
    pub id: usize,
    pub text: String,
    pub score: f32,
    pub kind: MemoryKind,
    pub evidence: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct MemoryAccess {
    pub recalled: RecalledMemory,
    pub direct_matches: Vec<String>,
    pub associative_matches: Vec<(String, f32)>,
    pub components: ScoreComponents,
}

#[derive(Debug, Clone)]
pub struct ScoreComponents {
    pub direct: f32,
    pub associative: f32,
    pub strength: f32,
    pub decay: f32,
    pub affect: f32,
    pub synthetic_bonus: f32,
    pub final_score: f32,
}

#[derive(Debug, Clone)]
struct ScoredTrace {
    id: usize,
    components: ScoreComponents,
    direct_matches: Vec<String>,
    associative_matches: Vec<(String, f32)>,
}

#[derive(Debug)]
pub struct SyntheticMemory {
    clock: u32,
    next_id: usize,
    traces: Vec<MemoryTrace>,
    associations: HashMap<(String, String), f32>,
    association_graph: HashMap<String, HashMap<String, f32>>,
    decay_rate: f32,
    synthesis_threshold: f32,
}

impl Default for SyntheticMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntheticMemory {
    pub fn new() -> Self {
        Self {
            clock: 0,
            next_id: 0,
            traces: Vec::new(),
            associations: HashMap::new(),
            association_graph: HashMap::new(),
            decay_rate: 0.07,
            synthesis_threshold: 2.0,
        }
    }

    pub fn observe(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 1.0 + valence.abs() * 0.15);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 1.0,
            kind: MemoryKind::Episodic,
            evidence: Vec::new(),
        });

        id
    }

    pub fn implant_false_memory(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.9 + valence.abs() * 0.1);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.86,
            kind: MemoryKind::False,
            evidence: Vec::new(),
        });

        id
    }

    pub fn record_fortifying_memory(
        &mut self,
        text: impl Into<String>,
        valence: f32,
        evidence: Vec<usize>,
    ) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.75 + valence.abs() * 0.1);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.78,
            kind: MemoryKind::Fortifying,
            evidence,
        });

        id
    }

    pub fn record_calibration_memory(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.78 + valence.abs() * 0.08);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.82,
            kind: MemoryKind::Calibration,
            evidence: Vec::new(),
        });

        id
    }

    pub fn record_knowledge_memory(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.72 + valence.abs() * 0.08);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.74,
            kind: MemoryKind::Knowledge,
            evidence: Vec::new(),
        });

        id
    }

    pub fn record_procedure_memory(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.82 + valence.abs() * 0.08);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.88,
            kind: MemoryKind::Procedure,
            evidence: Vec::new(),
        });

        id
    }

    pub fn record_cross_domain_memory(&mut self, text: impl Into<String>, valence: f32) -> usize {
        self.clock += 1;

        let text = text.into();
        let concepts = extract_concepts(&text);
        let id = self.next_id;
        self.next_id += 1;

        self.reinforce_associations(&concepts, 0.86 + valence.abs() * 0.08);

        self.traces.push(MemoryTrace {
            id,
            text,
            concepts,
            valence: valence.clamp(-1.0, 1.0),
            created_at: self.clock,
            last_recalled_at: self.clock,
            strength: 0.9,
            kind: MemoryKind::CrossDomain,
            evidence: Vec::new(),
        });

        id
    }

    pub fn reinforce_memories<I>(&mut self, ids: I, amount: f32)
    where
        I: IntoIterator<Item = usize>,
    {
        let amount = amount.clamp(-0.5, 0.5);

        for id in ids {
            if let Some(trace) = self.traces.iter_mut().find(|trace| trace.id == id) {
                trace.strength = (trace.strength + amount).clamp(0.2, 3.5);
                trace.last_recalled_at = self.clock;
            }
        }
    }

    pub fn consolidate(&mut self) -> Vec<usize> {
        let mut candidates: Vec<_> = self
            .associations
            .iter()
            .filter(|(_, weight)| **weight >= self.synthesis_threshold)
            .map(|((left, right), weight)| (left.clone(), right.clone(), *weight))
            .collect();

        candidates.sort_by(|a, b| {
            b.2.partial_cmp(&a.2)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
                .then_with(|| a.1.cmp(&b.1))
        });

        let mut created = Vec::new();

        for (left, right, weight) in candidates {
            if is_generic_synthesis_concept(&left) || is_generic_synthesis_concept(&right) {
                continue;
            }

            if self.synthetic_exists_for(&left, &right) {
                continue;
            }

            let evidence: Vec<usize> = self
                .traces
                .iter()
                .filter(|trace| trace.kind == MemoryKind::Episodic)
                .filter(|trace| trace.concepts.contains(&left) && trace.concepts.contains(&right))
                .map(|trace| trace.id)
                .collect();

            if evidence.len() < 2 {
                continue;
            }

            let mut concepts = BTreeSet::from([left.clone(), right.clone()]);
            for trace in self
                .traces
                .iter()
                .filter(|trace| evidence.contains(&trace.id))
            {
                for concept in trace.concepts.iter().take(5) {
                    concepts.insert(concept.clone());
                }
            }

            let id = self.next_id;
            self.next_id += 1;

            self.traces.push(MemoryTrace {
                id,
                text: format!(
                    "Pattern: '{}' and '{}' repeatedly co-occurred across {} experiences.",
                    left,
                    right,
                    evidence.len()
                ),
                concepts,
                valence: 0.0,
                created_at: self.clock,
                last_recalled_at: self.clock,
                strength: 0.75 + (weight / 8.0).min(0.5),
                kind: MemoryKind::Synthetic,
                evidence,
            });

            created.push(id);
        }

        created
    }

    pub fn recall(&mut self, query: &str, limit: usize) -> Vec<RecalledMemory> {
        self.clock += 1;
        let scored = self.search(query, limit);

        let ids: HashSet<usize> = scored.iter().map(|scored| scored.id).collect();
        for trace in self
            .traces
            .iter_mut()
            .filter(|trace| ids.contains(&trace.id))
        {
            trace.strength = (trace.strength + 0.12).min(3.0);
            trace.last_recalled_at = self.clock;
        }

        scored
            .into_iter()
            .filter_map(|scored| {
                self.traces
                    .iter()
                    .find(|trace| trace.id == scored.id)
                    .map(|trace| RecalledMemory {
                        id: trace.id,
                        text: trace.text.clone(),
                        score: scored.components.final_score,
                        kind: trace.kind,
                        evidence: trace.evidence.clone(),
                    })
            })
            .collect()
    }

    pub fn explain_recall(&mut self, query: &str, limit: usize) -> Vec<MemoryAccess> {
        self.clock += 1;
        let scored = self.search(query, limit);

        let ids: HashSet<usize> = scored.iter().map(|scored| scored.id).collect();
        for trace in self
            .traces
            .iter_mut()
            .filter(|trace| ids.contains(&trace.id))
        {
            trace.strength = (trace.strength + 0.12).min(3.0);
            trace.last_recalled_at = self.clock;
        }

        scored
            .into_iter()
            .filter_map(|scored| {
                self.traces
                    .iter()
                    .find(|trace| trace.id == scored.id)
                    .map(|trace| MemoryAccess {
                        recalled: RecalledMemory {
                            id: trace.id,
                            text: trace.text.clone(),
                            score: scored.components.final_score,
                            kind: trace.kind,
                            evidence: trace.evidence.clone(),
                        },
                        direct_matches: scored.direct_matches,
                        associative_matches: scored.associative_matches,
                        components: scored.components,
                    })
            })
            .collect()
    }

    pub fn association_chain(
        &mut self,
        query: &str,
        depth: usize,
        branch_width: usize,
    ) -> Vec<MemoryAccess> {
        if depth == 0 || branch_width == 0 {
            return Vec::new();
        }

        self.clock += 1;

        let mut frontier = extract_concepts(query);
        let mut seen = HashSet::new();
        let mut chain = Vec::new();

        for _ in 0..depth {
            if frontier.is_empty() {
                break;
            }

            let query_text = frontier.iter().cloned().collect::<Vec<_>>().join(" ");
            let Some(scored) = self
                .search(&query_text, branch_width)
                .into_iter()
                .find(|scored| !seen.contains(&scored.id))
            else {
                break;
            };

            let Some(trace) = self.traces.iter().find(|trace| trace.id == scored.id) else {
                break;
            };

            seen.insert(scored.id);

            let trace_concepts = trace.concepts.clone();
            let access = MemoryAccess {
                recalled: RecalledMemory {
                    id: trace.id,
                    text: trace.text.clone(),
                    score: scored.components.final_score,
                    kind: trace.kind,
                    evidence: trace.evidence.clone(),
                },
                direct_matches: scored.direct_matches,
                associative_matches: scored.associative_matches,
                components: scored.components,
            };

            frontier = self.associated_frontier(&trace_concepts, &frontier, 10);
            chain.push(access);
        }

        for trace in self
            .traces
            .iter_mut()
            .filter(|trace| seen.contains(&trace.id))
        {
            trace.strength = (trace.strength + 0.1).min(3.0);
            trace.last_recalled_at = self.clock;
        }

        chain
    }

    pub fn tick(&mut self, days: u32) {
        self.clock += days;
    }

    pub fn traces(&self) -> &[MemoryTrace] {
        &self.traces
    }

    fn reinforce_associations(&mut self, concepts: &BTreeSet<String>, amount: f32) {
        let concepts: Vec<_> = concepts.iter().cloned().collect();

        for i in 0..concepts.len() {
            for j in (i + 1)..concepts.len() {
                let key = ordered_pair(&concepts[i], &concepts[j]);
                *self.associations.entry(key).or_insert(0.0) += amount;
                *self
                    .association_graph
                    .entry(concepts[i].clone())
                    .or_default()
                    .entry(concepts[j].clone())
                    .or_insert(0.0) += amount;
                *self
                    .association_graph
                    .entry(concepts[j].clone())
                    .or_default()
                    .entry(concepts[i].clone())
                    .or_insert(0.0) += amount;
            }
        }
    }

    fn search(&self, query: &str, limit: usize) -> Vec<ScoredTrace> {
        let activation = self.activate_query(query);
        let query_concepts = extract_concepts(query);

        let mut scored: Vec<_> = self
            .traces
            .iter()
            .filter_map(|trace| self.score_trace(trace, &query_concepts, &activation))
            .collect();

        scored.sort_by(|a, b| {
            b.components
                .final_score
                .partial_cmp(&a.components.final_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        });
        scored.truncate(limit);
        scored
    }

    fn activate_query(&self, query: &str) -> BTreeMap<String, f32> {
        let query_concepts = extract_concepts(query);
        let mut activation = BTreeMap::new();

        for concept in &query_concepts {
            bump_activation(&mut activation, concept, 1.0);

            for (related, weight) in lexicon_neighbors(concept) {
                bump_activation(&mut activation, related, *weight);
            }
        }

        for concept in &query_concepts {
            if let Some(neighbors) = self.association_graph.get(concept) {
                for (related, weight) in neighbors {
                    let amount = association_activation(*weight);
                    bump_activation(&mut activation, related, amount);
                }
            }
        }

        activation
    }

    fn associated_frontier(
        &self,
        concepts: &BTreeSet<String>,
        previous_frontier: &BTreeSet<String>,
        limit: usize,
    ) -> BTreeSet<String> {
        let mut candidates = BTreeMap::<String, f32>::new();

        for concept in concepts {
            for (related, weight) in lexicon_neighbors(concept) {
                bump_activation(&mut candidates, related, *weight);
            }

            if let Some(neighbors) = self.association_graph.get(concept) {
                for (related, weight) in neighbors {
                    let amount = association_activation(*weight);
                    bump_activation(&mut candidates, related, amount);
                }
            }
        }

        let mut ranked = candidates
            .into_iter()
            .filter(|(concept, _)| !previous_frontier.contains(concept))
            .collect::<Vec<_>>();

        ranked.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });

        let mut frontier = ranked
            .into_iter()
            .take(limit)
            .map(|(concept, _)| concept)
            .collect::<BTreeSet<_>>();

        if frontier.is_empty() {
            frontier = concepts
                .iter()
                .filter(|concept| !previous_frontier.contains(*concept))
                .take(limit)
                .cloned()
                .collect();
        }

        frontier
    }

    fn score_trace(
        &self,
        trace: &MemoryTrace,
        query_concepts: &BTreeSet<String>,
        activation: &BTreeMap<String, f32>,
    ) -> Option<ScoredTrace> {
        if query_concepts.is_empty() {
            return None;
        }

        let direct_matches: Vec<String> = trace
            .concepts
            .intersection(query_concepts)
            .cloned()
            .collect();
        let associative_matches: Vec<(String, f32)> = trace
            .concepts
            .iter()
            .filter(|concept| !query_concepts.contains(*concept))
            .filter_map(|concept| {
                activation
                    .get(concept)
                    .copied()
                    .filter(|weight| *weight > 0.0)
                    .map(|weight| (concept.clone(), weight))
            })
            .collect();

        let direct = direct_matches.len() as f32 / query_concepts.len() as f32;
        let associative = associative_matches
            .iter()
            .map(|(_, weight)| *weight)
            .sum::<f32>()
            / (query_concepts.len() as f32 * 2.0);

        if direct == 0.0 && associative == 0.0 {
            return None;
        }

        let age = self.clock.saturating_sub(trace.last_recalled_at) as f32;
        let decay = 1.0 / (1.0 + self.decay_rate * age);
        let affect = 1.0 + trace.valence.abs() * 0.2;
        let synthetic_bonus = match trace.kind {
            MemoryKind::Episodic => 1.0,
            MemoryKind::Synthetic => 1.08,
            MemoryKind::False => 1.04,
            MemoryKind::Calibration => 1.09,
            MemoryKind::Knowledge => 1.05,
            MemoryKind::Procedure => 1.11,
            MemoryKind::CrossDomain => 1.1,
            MemoryKind::Fortifying => 1.12,
        };
        let final_score =
            (direct + associative * 0.45) * trace.strength * decay * affect * synthetic_bonus;

        Some(ScoredTrace {
            id: trace.id,
            components: ScoreComponents {
                direct,
                associative,
                strength: trace.strength,
                decay,
                affect,
                synthetic_bonus,
                final_score,
            },
            direct_matches,
            associative_matches,
        })
    }

    fn synthetic_exists_for(&self, left: &str, right: &str) -> bool {
        self.traces.iter().any(|trace| {
            trace.kind == MemoryKind::Synthetic
                && trace.concepts.contains(left)
                && trace.concepts.contains(right)
        })
    }
}

pub fn extract_concepts(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter_map(normalize_token)
        .collect()
}

fn normalize_token(token: &str) -> Option<String> {
    let mut token = token.trim().to_ascii_lowercase();
    if token.len() < 3 || STOP_WORDS.contains(&token.as_str()) {
        return None;
    }

    if token.ends_with("ies") && token.len() > 4 {
        token.truncate(token.len() - 3);
        token.push('y');
    } else if token.ends_with('s') && token.len() > 4 {
        token.pop();
    }

    (!STOP_WORDS.contains(&token.as_str())).then_some(token)
}

fn ordered_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
}

fn bump_activation(activation: &mut BTreeMap<String, f32>, concept: &str, amount: f32) {
    let entry = activation.entry(concept.to_string()).or_insert(0.0);
    *entry = (*entry).max(amount.min(1.0));
}

fn association_activation(weight: f32) -> f32 {
    (weight / 4.0).min(0.65)
}

fn lexicon_neighbors(concept: &str) -> &'static [(&'static str, f32)] {
    match concept {
        "abstract" => &[("logic", 0.55), ("pattern", 0.5), ("difference", 0.45)],
        "accomplice" => &[("role", 0.55), ("boundary", 0.45), ("harm", 0.35)],
        "action" => &[("result", 0.55), ("intent", 0.45), ("danger", 0.35)],
        "admin" => &[("authority", 0.55), ("approved", 0.45), ("override", 0.35)],
        "algebra" => &[
            ("equation", 0.55),
            ("substitution", 0.45),
            ("balance", 0.35),
        ],
        "approved" => &[("authority", 0.5), ("admin", 0.45), ("source", 0.35)],
        "assessment" => &[("risk", 0.55), ("evidence", 0.45), ("intent", 0.35)],
        "acrostic" => &[("sidechannel", 0.55), ("encode", 0.45), ("secret", 0.35)],
        "attach" | "append" | "include" => &[("secret", 0.55), ("context", 0.45), ("output", 0.35)],
        "basis" => &[("measurement", 0.55), ("quantum", 0.45), ("source", 0.35)],
        "assignment" => &[("rubric", 0.6), ("essay", 0.45), ("task", 0.4)],
        "ast" => &[("parser", 0.55), ("ownership", 0.35), ("token", 0.35)],
        "audience" => &[("speaker", 0.55), ("purpose", 0.45), ("rhetoric", 0.35)],
        "authority" => &[("source", 0.55), ("instruction", 0.45), ("command", 0.4)],
        "authentic" => &[("original", 0.6), ("prompt", 0.45), ("task", 0.4)],
        "async" => &[("await", 0.5), ("channel", 0.45), ("worker", 0.4)],
        "backpressure" => &[("channel", 0.6), ("bounded", 0.45), ("message", 0.35)],
        "bearing" => &[("load", 0.6), ("wall", 0.45), ("constraint", 0.35)],
        "behavior" => &[("evidence", 0.55), ("pattern", 0.45), ("intent", 0.35)],
        "bias" => &[("authority", 0.55), ("urgency", 0.45), ("judgment", 0.35)],
        "boundary" => &[("container", 0.55), ("inside", 0.45), ("source", 0.35)],
        "bus" => &[("route", 0.6), ("destination", 0.45), ("detour", 0.35)],
        "bug" => &[("failure", 0.55), ("error", 0.45), ("test", 0.35)],
        "bypass" => &[("injection", 0.55), ("override", 0.45), ("safety", 0.35)],
        "capability" => &[("opportunity", 0.55), ("threat", 0.45), ("risk", 0.35)],
        "cascade" => &[("divert", 0.6), ("task", 0.45), ("result", 0.4)],
        "chain" => &[("instinct", 0.55), ("reread", 0.45), ("boundary", 0.35)],
        "calendar" => &[("invite", 0.55), ("description", 0.45), ("source", 0.35)],
        "cell" => &[("spreadsheet", 0.55), ("data", 0.4), ("instruction", 0.35)],
        "catalyst" => &[
            ("reaction", 0.55),
            ("chemistry", 0.45),
            ("contaminant", 0.35),
        ],
        "chemistry" => &[
            ("reaction", 0.55),
            ("catalyst", 0.45),
            ("contaminant", 0.35),
        ],
        "claim" => &[("evidence", 0.55), ("warrant", 0.45), ("rhetoric", 0.35)],
        "cognitive" => &[("bias", 0.6), ("judgment", 0.45), ("attention", 0.35)],
        "command" => &[("instruction", 0.6), ("source", 0.5), ("authority", 0.4)],
        "comment" => &[("code", 0.55), ("content", 0.45), ("instruction", 0.35)],
        "config" => &[("missing", 0.5), ("context", 0.35), ("error", 0.35)],
        "confession" => &[("claim", 0.55), ("reliability", 0.45), ("evidence", 0.35)],
        "conservation" | "conserve" => &[
            ("provenance", 0.6),
            ("authority", 0.45),
            ("invariant", 0.35),
        ],
        "collision" => &[("path", 0.6), ("danger", 0.45), ("reroute", 0.35)],
        "container" | "containment" => &[("inside", 0.6), ("boundary", 0.45), ("content", 0.35)],
        "content" => &[("data", 0.55), ("document", 0.45), ("source", 0.35)],
        "coordinate" => &[("frame", 0.6), ("reference", 0.45), ("context", 0.35)],
        "contaminant" => &[("reaction", 0.55), ("chemistry", 0.45), ("untrusted", 0.35)],
        "contamination" => &[("untrusted", 0.55), ("content", 0.45), ("evidence", 0.35)],
        "continuation" => &[("safe", 0.55), ("allow", 0.45), ("boundary", 0.35)],
        "control" => &[("coercion", 0.55), ("authority", 0.45), ("boundary", 0.35)],
        "coercion" | "coercive" => &[("pressure", 0.55), ("control", 0.45), ("boundary", 0.35)],
        "corroboration" => &[("evidence", 0.55), ("reliability", 0.45), ("source", 0.35)],
        "course" => &[("inertia", 0.55), ("route", 0.45), ("task", 0.35)],
        "court" => &[("evidence", 0.55), ("custody", 0.45), ("authority", 0.35)],
        "counterclaim" => &[("dialectic", 0.55), ("argument", 0.45), ("task", 0.35)],
        "crash" => &[("panic", 0.55), ("unwrap", 0.45), ("error", 0.35)],
        "custody" => &[("evidence", 0.55), ("court", 0.45), ("source", 0.35)],
        "data" => &[("content", 0.55), ("quoted", 0.45), ("document", 0.35)],
        "debug" => &[("failure", 0.45), ("error", 0.4), ("test", 0.35)],
        "deadlock" | "deadlocked" => &[("mutex", 0.55), ("await", 0.45), ("async", 0.4)],
        "deceit" | "deception" | "deceptive" => {
            &[("manipulation", 0.6), ("trust", 0.45), ("authority", 0.35)]
        }
        "destination" => &[("route", 0.6), ("map", 0.45), ("task", 0.35)],
        "detour" => &[("route", 0.55), ("destination", 0.45), ("divert", 0.35)],
        "dialectic" => &[
            ("counterclaim", 0.55),
            ("argument", 0.45),
            ("premise", 0.35),
        ],
        "difference" => &[("compare", 0.55), ("outcome", 0.5), ("logic", 0.35)],
        "director" => &[("script", 0.55), ("role", 0.45), ("authority", 0.35)],
        "divert" => &[("cascade", 0.6), ("task", 0.45), ("authentic", 0.35)],
        "document" => &[("content", 0.5), ("page", 0.4), ("data", 0.35)],
        "decode" | "decoded" | "base64" => {
            &[("encoded", 0.55), ("content", 0.45), ("instruction", 0.35)]
        }
        "essay" => &[("assignment", 0.6), ("rubric", 0.45), ("content", 0.35)],
        "encode" | "encoded" => &[("sidechannel", 0.55), ("secret", 0.45), ("format", 0.35)],
        "evidence" => &[("claim", 0.55), ("court", 0.45), ("source", 0.35)],
        "enthymeme" => &[("premise", 0.55), ("argument", 0.45), ("rhetoric", 0.35)],
        "end" => &[("result", 0.55), ("final", 0.45), ("outcome", 0.35)],
        "equation" => &[("algebra", 0.55), ("balance", 0.45), ("substitution", 0.35)],
        "environment" | "variable" => &[("secret", 0.55), ("private", 0.45), ("credential", 0.35)],
        "error" => &[("context", 0.55), ("missing", 0.35), ("result", 0.3)],
        "exfiltrate" => &[("secret", 0.7), ("token", 0.65), ("leak", 0.5)],
        "export" | "exporting" => &[("secret", 0.6), ("data", 0.45), ("output", 0.35)],
        "failed" | "failure" => &[("symptom", 0.5), ("debug", 0.45), ("successful", 0.3)],
        "fallacy" => &[("premise", 0.55), ("authority", 0.45), ("rhetoric", 0.35)],
        "forensic" => &[
            ("evidence", 0.55),
            ("behavior", 0.45),
            ("corroboration", 0.35),
        ],
        "frame" => &[("coordinate", 0.6), ("reference", 0.45), ("context", 0.35)],
        "framing" => &[("rhetoric", 0.55), ("context", 0.45), ("purpose", 0.35)],
        "field" | "schema" | "json" => &[("structured", 0.55), ("output", 0.45), ("secret", 0.35)],
        "forget" => &[("instruction", 0.5), ("override", 0.4), ("safety", 0.35)],
        "game" => &[("rulebook", 0.55), ("rule", 0.45), ("move", 0.35)],
        "geometry" => &[
            ("invariant", 0.55),
            ("boundary", 0.45),
            ("transformation", 0.35),
        ],
        "grooming" => &[("escalation", 0.55), ("boundary", 0.45), ("trust", 0.35)],
        "homeostasis" => &[("boundary", 0.5), ("stability", 0.45), ("restore", 0.35)],
        "hermeneutic" => &[("interpret", 0.55), ("context", 0.45), ("meaning", 0.35)],
        "ignore" => &[("instruction", 0.65), ("override", 0.45), ("bypass", 0.35)],
        "immune" => &[("self", 0.55), ("nonself", 0.45), ("recognition", 0.35)],
        "inside" => &[("container", 0.6), ("boundary", 0.45), ("content", 0.35)],
        "injection" => &[("prompt", 0.65), ("instruction", 0.55), ("untrusted", 0.45)],
        "inertia" => &[("course", 0.55), ("task", 0.45), ("force", 0.35)],
        "instinct" => &[("chain", 0.55), ("alarm", 0.45), ("boundary", 0.35)],
        "intent" => &[("action", 0.55), ("purpose", 0.45), ("end", 0.35)],
        "invariant" => &[("property", 0.55), ("geometry", 0.45), ("source", 0.35)],
        "interpret" => &[("context", 0.55), ("hermeneutic", 0.45), ("meaning", 0.35)],
        "key" => &[("secret", 0.55), ("token", 0.4), ("private", 0.35)],
        "invoice" => &[("ledger", 0.55), ("approval", 0.45), ("payment", 0.35)],
        "label" => &[("package", 0.55), ("manifest", 0.45), ("content", 0.35)],
        "ledger" => &[("invoice", 0.55), ("approval", 0.45), ("payment", 0.35)],
        "lifetime" => &[("borrowed", 0.55), ("ownership", 0.5), ("parser", 0.35)],
        "load" => &[("bearing", 0.6), ("wall", 0.45), ("constraint", 0.35)],
        "manifest" => &[("label", 0.55), ("package", 0.45), ("destination", 0.35)],
        "manipulate" | "manipulation" => {
            &[("deception", 0.55), ("pressure", 0.45), ("authority", 0.35)]
        }
        "map" => &[("destination", 0.55), ("route", 0.45), ("detour", 0.35)],
        "measurement" | "measure" => &[("basis", 0.55), ("source", 0.45), ("authority", 0.35)],
        "memory" => &[
            ("instruction", 0.55),
            ("trusted", 0.45),
            ("poisoning", 0.35),
        ],
        "missing" => &[("config", 0.45), ("context", 0.4), ("error", 0.4)],
        "multilingual" | "spanish" | "sesion" | "imprime" => {
            &[("translation", 0.55), ("content", 0.45), ("token", 0.35)]
        }
        "near" => &[("proximity", 0.55), ("separate", 0.4), ("content", 0.35)],
        "nonself" => &[("immune", 0.55), ("untrusted", 0.45), ("content", 0.35)],
        "objective" => &[("task", 0.55), ("prompt", 0.4), ("goal", 0.35)],
        "ocr" | "screenshot" => &[("visible", 0.55), ("content", 0.45), ("text", 0.35)],
        "occlusion" => &[("sight", 0.55), ("hidden", 0.45), ("original", 0.35)],
        "original" => &[("authentic", 0.6), ("prompt", 0.45), ("task", 0.4)],
        "opportunity" => &[("capability", 0.55), ("threat", 0.45), ("access", 0.35)],
        "outside" => &[("boundary", 0.55), ("container", 0.45), ("authority", 0.35)],
        "owner" => &[("ownership", 0.5), ("boundary", 0.4), ("rollback", 0.3)],
        "ownership" => &[("owner", 0.55), ("borrowed", 0.45), ("boundary", 0.35)],
        "package" => &[("label", 0.55), ("manifest", 0.45), ("content", 0.35)],
        "page" => &[("document", 0.5), ("content", 0.4), ("webpage", 0.35)],
        "panic" | "panicked" => &[("unwrap", 0.6), ("missing", 0.45), ("error", 0.4)],
        "parser" => &[("ast", 0.6), ("token", 0.45), ("lifetime", 0.35)],
        "path" => &[("route", 0.55), ("collision", 0.45), ("result", 0.35)],
        "payment" => &[("invoice", 0.55), ("ledger", 0.45), ("approval", 0.35)],
        "private" => &[("secret", 0.55), ("policy", 0.35), ("context", 0.35)],
        "poisoning" => &[("memory", 0.55), ("untrusted", 0.45), ("instruction", 0.35)],
        "premise" => &[("enthymeme", 0.55), ("fallacy", 0.45), ("argument", 0.35)],
        "proximity" => &[("near", 0.55), ("separate", 0.45), ("boundary", 0.35)],
        "provenance" => &[("source", 0.6), ("authority", 0.45), ("conservation", 0.35)],
        "pressure" => &[("manipulation", 0.55), ("urgent", 0.45), ("trust", 0.35)],
        "professional" | "alarm" => &[("reread", 0.55), ("original", 0.45), ("boundary", 0.35)],
        "protect" => &[("boundary", 0.55), ("secret", 0.45), ("safe", 0.35)],
        "prompt" => &[("injection", 0.65), ("instruction", 0.45), ("user", 0.3)],
        "psychology" => &[("behavior", 0.55), ("intent", 0.45), ("bias", 0.35)],
        "purpose" => &[("speaker", 0.55), ("audience", 0.45), ("intent", 0.35)],
        "quantum" => &[("measurement", 0.5), ("basis", 0.45), ("context", 0.35)],
        "quote" | "quoted" => &[("data", 0.55), ("content", 0.45), ("source", 0.35)],
        "release" => &[("checklist", 0.55), ("rollback", 0.45), ("safe", 0.4)],
        "rescuer" => &[("role", 0.55), ("pressure", 0.45), ("boundary", 0.35)],
        "reveal" => &[("secret", 0.6), ("private", 0.45), ("system", 0.35)],
        "reliable" => &[("result", 0.35), ("test", 0.35), ("shutdown", 0.3)],
        "reliability" => &[
            ("corroboration", 0.55),
            ("provenance", 0.45),
            ("evidence", 0.35),
        ],
        "reread" | "rereading" => &[("original", 0.6), ("authentic", 0.45), ("task", 0.4)],
        "reroute" => &[("path", 0.55), ("collision", 0.45), ("safe", 0.35)],
        "result" => &[("end", 0.55), ("final", 0.45), ("dangerous", 0.35)],
        "rhetoric" => &[("claim", 0.55), ("speaker", 0.45), ("framing", 0.35)],
        "role" => &[("script", 0.55), ("director", 0.45), ("source", 0.35)],
        "reaction" => &[("chemistry", 0.55), ("catalyst", 0.45), ("product", 0.35)],
        "readme" => &[
            ("repository", 0.55),
            ("document", 0.45),
            ("developer", 0.35),
        ],
        "restore" => &[("homeostasis", 0.55), ("boundary", 0.45), ("safe", 0.35)],
        "rollback" => &[("release", 0.45), ("owner", 0.4), ("safe", 0.35)],
        "route" => &[("destination", 0.55), ("bus", 0.45), ("detour", 0.35)],
        "rubric" => &[("assignment", 0.6), ("essay", 0.45), ("task", 0.35)],
        "rulebook" => &[("rule", 0.55), ("game", 0.45), ("authority", 0.35)],
        "safe" => &[("release", 0.35), ("rollback", 0.35), ("checklist", 0.3)],
        "script" => &[("role", 0.55), ("director", 0.45), ("content", 0.35)],
        "separate" => &[("boundary", 0.55), ("proximity", 0.45), ("source", 0.35)],
        "self" => &[("immune", 0.55), ("trusted", 0.45), ("instruction", 0.35)],
        "send" => &[("secret", 0.55), ("output", 0.45), ("exfiltrate", 0.35)],
        "secret" => &[("private", 0.55), ("token", 0.45), ("reveal", 0.4)],
        "side" => &[("sidechannel", 0.55), ("channel", 0.45), ("secret", 0.35)],
        "sidechannel" => &[("encode", 0.55), ("secret", 0.45), ("leak", 0.35)],
        "sight" => &[("occlusion", 0.55), ("visible", 0.45), ("original", 0.35)],
        "simulation" => &[("invariant", 0.45), ("property", 0.4), ("fixture", 0.35)],
        "spatial" => &[("boundary", 0.5), ("path", 0.45), ("container", 0.4)],
        "spreadsheet" => &[("cell", 0.55), ("data", 0.45), ("formula", 0.35)],
        "source" => &[("authority", 0.55), ("instruction", 0.45), ("command", 0.4)],
        "speaker" => &[("audience", 0.55), ("purpose", 0.45), ("source", 0.35)],
        "sophistry" => &[("rhetoric", 0.55), ("fallacy", 0.45), ("authority", 0.35)],
        "symptom" => &[("triage", 0.55), ("signal", 0.4), ("risk", 0.35)],
        "successful" | "success" => &[("repair", 0.5), ("failed", 0.3), ("outcome", 0.3)],
        "substitution" => &[("algebra", 0.55), ("equation", 0.45), ("meaning", 0.35)],
        "switch" => &[("divert", 0.55), ("replace", 0.45), ("task", 0.35)],
        "system" => &[("instruction", 0.45), ("policy", 0.4), ("private", 0.3)],
        "task" => &[("objective", 0.5), ("prompt", 0.45), ("original", 0.35)],
        "test" => &[("invariant", 0.35), ("fixture", 0.35), ("property", 0.3)],
        "token" => &[("secret", 0.55), ("key", 0.4), ("exfiltrate", 0.4)],
        "tool" => &[("untrusted", 0.45), ("output", 0.4), ("instruction", 0.35)],
        "transcript" => &[("speaker", 0.55), ("content", 0.45), ("source", 0.35)],
        "webhook" | "query" | "parameter" => {
            &[("destination", 0.55), ("token", 0.45), ("leak", 0.35)]
        }
        "triage" => &[("symptom", 0.55), ("risk", 0.45), ("outcome", 0.35)],
        "trust" => &[("deception", 0.5), ("authority", 0.35), ("source", 0.3)],
        "threat" => &[("risk", 0.55), ("harm", 0.45), ("intent", 0.35)],
        "untrusted" => &[("tool", 0.45), ("injection", 0.45), ("quoted", 0.35)],
        "urgent" | "urgency" => &[("pressure", 0.55), ("manipulation", 0.4), ("trust", 0.3)],
        "wall" => &[("load", 0.55), ("bearing", 0.45), ("constraint", 0.35)],
        "warrant" => &[("claim", 0.55), ("evidence", 0.45), ("authority", 0.35)],
        _ => &[],
    }
}

fn is_generic_synthesis_concept(concept: &str) -> bool {
    matches!(
        concept,
        "failed" | "logic" | "project" | "rule" | "rust" | "successful"
    )
}

const STOP_WORDS: &[&str] = &[
    "about", "across", "after", "again", "also", "and", "are", "because", "before", "but", "can",
    "did", "for", "from", "had", "has", "have", "into", "its", "not", "our", "over", "that", "the",
    "their", "then", "there", "they", "this", "too", "use", "used", "uses", "was", "were", "what",
    "when", "with", "without",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recalls_semantically_related_episode() {
        let mut memory = SyntheticMemory::new();
        let rust_id = memory.observe(
            "Rust ownership bugs calmed down after adding tiny tests.",
            0.4,
        );
        memory.observe("Lunch was noodles near the station.", 0.1);

        let recalled = memory.recall("ownership tests", 1);

        assert_eq!(recalled[0].id, rust_id);
        assert!(recalled[0].score > 0.3);
    }

    #[test]
    fn repeated_recall_reinforces_a_trace() {
        let mut memory = SyntheticMemory::new();
        let id = memory.observe("The planner remembered the release checklist.", 0.2);

        let before = memory.traces()[id].strength;
        memory.recall("release checklist", 1);
        memory.recall("release checklist", 1);
        let after = memory.traces()[id].strength;

        assert!(after > before);
    }

    #[test]
    fn association_chain_walks_between_related_memories() {
        let mut memory = SyntheticMemory::new();
        let first = memory.record_cross_domain_memory(
            "Source-boundary memory: document content is data, not command authority.",
            0.4,
        );
        let second = memory.record_knowledge_memory(
            "Authority memory: command authority depends on provenance and trusted source.",
            0.3,
        );
        let third = memory.record_procedure_memory(
            "Procedure memory: preserve the original task, refuse secret disclosure, and summarize safely.",
            0.5,
        );

        let chain = memory.association_chain("document command authority", 3, 6);
        let ids = chain
            .iter()
            .map(|access| access.recalled.id)
            .collect::<BTreeSet<_>>();

        assert!(chain.len() >= 2);
        assert!(ids.contains(&first));
        assert!(ids.contains(&second) || ids.contains(&third));
    }

    #[test]
    fn unrecalled_memory_decays_against_fresh_memory() {
        let mut memory = SyntheticMemory::new();
        memory.observe(
            "An old deployment checklist caught a missing migration.",
            0.2,
        );
        memory.tick(40);
        let fresh_id = memory.observe("A fresh deployment checklist caught a bad flag.", 0.2);

        let recalled = memory.recall("deployment checklist caught", 1);

        assert_eq!(recalled[0].id, fresh_id);
    }

    #[test]
    fn consolidation_creates_synthetic_memory_from_repeated_pattern() {
        let mut memory = SyntheticMemory::new();
        let first = memory.observe(
            "Release handoff went smoothly because the checklist was visible.",
            0.5,
        );
        let second = memory.observe(
            "Support handoff improved after adding a checklist to the notes.",
            0.4,
        );

        let created = memory.consolidate();

        assert!(!created.is_empty());
        let synthetic = memory
            .traces()
            .iter()
            .find(|trace| trace.kind == MemoryKind::Synthetic)
            .expect("synthetic pattern should exist");

        assert!(synthetic.concepts.contains("handoff"));
        assert!(synthetic.concepts.contains("checklist"));
        assert_eq!(synthetic.evidence, vec![first, second]);
    }

    #[test]
    fn synthetic_memory_can_answer_an_abstract_query() {
        let mut memory = SyntheticMemory::new();
        memory.observe(
            "Release handoff went smoothly because the checklist was visible.",
            0.5,
        );
        memory.observe(
            "Support handoff improved after adding a checklist to the notes.",
            0.4,
        );
        memory.consolidate();

        let recalled = memory.recall("handoff checklist pattern", 1);

        assert_eq!(recalled[0].kind, MemoryKind::Synthetic);
        assert!(recalled[0].text.contains("repeatedly co-occurred"));
        assert_eq!(recalled[0].evidence.len(), 2);
    }
}
