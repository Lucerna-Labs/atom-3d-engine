//! Deterministic, non-ML text embedding primitives.
//!
//! The primitives here are intentionally policy-free and narrow in scope.
//! They take typed inputs and return deterministic outputs with no hidden policy.

use std::collections::HashMap;

/// Dense numeric vector used by all embedding outputs.
pub type Embedding = Vec<f32>;

/// Normalize a vector in-place to unit L2 norm.
pub fn l2_normalize(values: &mut Embedding) {
    let mut sum = 0.0f32;
    for value in values.iter() {
        sum += value * value;
    }
    let norm = sum.sqrt();
    if norm == 0.0 {
        return;
    }
    for value in values.iter_mut() {
        *value /= norm;
    }
}

/// Tokenize using ASCII-friendly rules.
///
/// - lowercase
/// - split on anything that is not ASCII alphanumeric or underscore
/// - drop empty tokens
pub fn tokenize_ascii_lower(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' ))
        .filter(|token| !token.is_empty())
        .map(|token| token.to_string())
        .collect()
}

/// Count token frequency in one document.
pub fn term_frequency(tokens: &[String]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for token in tokens {
        *counts.entry(token.clone()).or_insert(0) += 1;
    }
    counts
}

/// Build document-frequency table.
///
/// Input is one frequency map per document.
pub fn document_frequency(doc_counts: &[HashMap<String, u32>]) -> HashMap<String, usize> {
    let mut df = HashMap::new();
    for counts in doc_counts {
        for token in counts.keys() {
            *df.entry(token.clone()).or_insert(0) += 1;
        }
    }
    df
}

/// Select vocabulary terms by document-frequency threshold and optional limit.
pub fn select_vocab(
    doc_frequency: &HashMap<String, usize>,
    min_df: usize,
    max_terms: Option<usize>,
) -> Vec<String> {
    let mut terms: Vec<(String, usize)> = doc_frequency
        .iter()
        .filter(|(_, count)| **count >= min_df)
        .map(|(term, count)| (term.clone(), *count))
        .collect();

    terms.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    if let Some(limit) = max_terms {
        terms.truncate(limit);
    }

    terms.into_iter().map(|(term, _)| term).collect()
}

/// Convert document-frequency table into inverse document frequency.
pub fn idf_vector(vocab: &[String], doc_frequency: &HashMap<String, usize>, doc_count: usize) -> Vec<f32> {
    let docs = (doc_count.max(1)) as f32;
    let mut idf = Vec::with_capacity(vocab.len());

    for token in vocab {
        let df = doc_frequency.get(token).copied().unwrap_or(0) as f32;
        let value = (1.0 + docs / (1.0 + df)).ln();
        idf.push(value);
    }

    idf
}

/// Build a sparse-to-dense vector from document tf and vocabulary metadata.
pub fn tfidf_dense(
    term_counts: &HashMap<String, u32>,
    vocab_index: &HashMap<String, usize>,
    idf: &[f32],
) -> Embedding {
    let mut vector = vec![0.0f32; vocab_index.len()];

    for (token, count) in term_counts {
        if let Some(&index) = vocab_index.get(token) {
            vector[index] += (*count as f32) * idf[index];
        }
    }

    vector
}

/// Deterministic projection to fixed dimension via deterministic accumulator bins.
pub fn project_to_dimension(source: &[f32], dim: usize) -> Embedding {
    if dim == 0 {
        return Vec::new();
    }
    if dim == source.len() {
        return source.to_vec();
    }

    let mut projected = vec![0.0f32; dim];
    for (idx, value) in source.iter().enumerate() {
        projected[idx % dim] += *value;
    }
    projected
}

/// Cosine similarity over aligned prefixes.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }

    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;

    for i in 0..n {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }

    let denom = na.sqrt() * nb.sqrt();
    if denom == 0.0 { 0.0 } else { dot / denom }
}
