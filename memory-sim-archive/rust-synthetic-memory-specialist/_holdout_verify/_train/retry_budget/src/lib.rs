//! retry_budget: each candidate is registered with an integer retry budget and
//! an ordered list of per-attempt outcomes ("ok" or "fail"). A candidate is
//! given up to `budget + 1` attempts total (one initial attempt plus `budget`
//! retries). It SUCCEEDS the first time an "ok" outcome is reached within that
//! attempt allowance; otherwise it FAILS (it ran out of budget, or every listed
//! outcome was "fail", or it had no outcomes at all).
//!
//! `resolve` scans the candidates in insertion order and reports the first one
//! that succeeds. Along the way it emits retry signals describing how many
//! retries each *visited* candidate consumed before its success or failure:
//!   - "retry:<name>:<n>" when candidate <name> consumed n >= 1 retries.
//! Candidates after the first success are not visited and emit no signals. If no
//! candidate succeeds, the winner is reported as "none".

/// Outcome of resolving the candidate list: the winning candidate (or "none")
/// and the retry signals emitted for each visited candidate, in visit order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolution {
    pub winner: String,
    pub signals: Vec<String>,
}

/// A registered candidate: its name, retry budget, and ordered attempt outcomes
/// (each outcome is the literal "ok" or "fail").
#[derive(Clone, Debug, PartialEq, Eq)]
struct Candidate {
    name: String,
    budget: i32,
    outcomes: Vec<String>,
}

/// Runtime holding candidates in insertion order and resolving them under their
/// individual retry budgets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryRuntime {
    candidates: Vec<Candidate>,
}

impl RetryRuntime {
    pub fn new() -> RetryRuntime {
        RetryRuntime {
            candidates: Vec::new(),
        }
    }

    /// Register a candidate with the given retry budget, preserving insertion
    /// order. Outcomes are appended later via `attempt`.
    pub fn add(&mut self, name: &str, budget: i32) {
        self.candidates.push(Candidate {
            name: name.to_string(),
            budget,
            outcomes: Vec::new(),
        });
    }

    /// Append one attempt outcome ("ok" or "fail") to the named candidate, in
    /// order. Outcomes for an unknown candidate are ignored.
    pub fn attempt(&mut self, name: &str, outcome: &str) {
        if let Some(c) = self.candidates.iter_mut().find(|c| c.name == name) {
            c.outcomes.push(outcome.to_string());
        }
    }

    /// Number of retries a candidate consumes before it succeeds or gives up.
    /// A candidate gets `budget + 1` attempts; the first attempt is the initial
    /// try (zero retries) and each subsequent attempt costs one retry. Returns
    /// (succeeded, retries_consumed).
    fn evaluate(c: &Candidate) -> (bool, i32) {
        let max_attempts = if c.budget < 0 { 1 } else { c.budget + 1 };
        let mut attempt_index = 0;
        while attempt_index < max_attempts {
            match c.outcomes.get(attempt_index as usize) {
                Some(o) if o == "ok" => return (true, attempt_index),
                Some(_) => attempt_index += 1,
                None => break,
            }
        }
        // Failed: retries consumed is the number of retries used, capped by both
        // the attempts actually made and the budget.
        let attempts_made = attempt_index.min(c.outcomes.len() as i32);
        let retries = (attempts_made - 1).max(0);
        (false, retries)
    }

    /// Scan candidates in insertion order and report the first that succeeds
    /// within its retry budget, emitting "retry:<name>:<n>" for each visited
    /// candidate that consumed at least one retry. Reports winner "none" if no
    /// candidate succeeds.
    pub fn resolve(&self) -> Resolution {
        let mut signals: Vec<String> = Vec::new();
        for c in &self.candidates {
            let (succeeded, retries) = RetryRuntime::evaluate(c);
            if retries >= 1 {
                signals.push(format!("retry:{}:{}", c.name, retries));
            }
            if succeeded {
                return Resolution {
                    winner: c.name.clone(),
                    signals,
                };
            }
        }
        Resolution {
            winner: "none".to_string(),
            signals,
        }
    }
}

impl Default for RetryRuntime {
    fn default() -> RetryRuntime {
        RetryRuntime::new()
    }
}
