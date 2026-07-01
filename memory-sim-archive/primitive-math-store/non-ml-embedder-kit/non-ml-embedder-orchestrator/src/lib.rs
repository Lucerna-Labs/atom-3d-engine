//! Orchestrator layer for a non-ML embedding stack.
//!
//! The orchestrator owns policy (feature selection thresholds, dimension policy,
//! fitting order, query routing) and delegates mechanism to primitives.

use std::collections::HashMap;

use non_ml_embedder_primitives as primitives;

#[derive(Debug, Clone)]
pub struct NonMlEmbedderConfig {
    /// Drop tokens that appear in fewer than this many documents.
    pub min_df: usize,
    /// Cap vocabulary size. 0 means keep every term that passes min_df.
    pub max_terms: usize,
    /// Output dimension after projection. 0 means keep natural vocabulary dimension.
    pub dim: usize,
    /// Apply L2 normalization.
    pub normalize: bool,
}

impl Default for NonMlEmbedderConfig {
    fn default() -> Self {
        Self {
            min_df: 1,
            max_terms: 2048,
            dim: 0,
            normalize: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FitReport {
    pub documents: usize,
    pub kept_terms: usize,
    pub raw_terms: usize,
    pub effective_dimension: usize,
}

#[derive(Debug)]
pub enum EmbedderError {
    NotFitted,
    EmptyCorpus,
}

pub struct NonMlEmbedder {
    config: NonMlEmbedderConfig,
    vocab: Vec<String>,
    vocab_index: HashMap<String, usize>,
    idf: Vec<f32>,
    is_fitted: bool,
}

impl NonMlEmbedder {
    pub fn new(config: NonMlEmbedderConfig) -> Self {
        Self {
            config,
            vocab: Vec::new(),
            vocab_index: HashMap::new(),
            idf: Vec::new(),
            is_fitted: false,
        }
    }

    pub fn fit(&mut self, corpus: &[&str]) -> Result<FitReport, EmbedderError> {
        if corpus.is_empty() {
            return Err(EmbedderError::EmptyCorpus);
        }

        let mut per_doc_counts = Vec::with_capacity(corpus.len());
        let mut raw_term_count = 0usize;

        for text in corpus {
            let tokens = primitives::tokenize_ascii_lower(text);
            raw_term_count += tokens.len();
            per_doc_counts.push(primitives::term_frequency(&tokens));
        }

        let doc_frequency = primitives::document_frequency(&per_doc_counts);
        let max_terms = if self.config.max_terms == 0 { None } else { Some(self.config.max_terms) };
        let vocab = primitives::select_vocab(&doc_frequency, self.config.min_df, max_terms);

        let mut vocab_index = HashMap::with_capacity(vocab.len());
        for (idx, token) in vocab.iter().enumerate() {
            vocab_index.insert(token.clone(), idx);
        }

        let idf = primitives::idf_vector(&vocab, &doc_frequency, corpus.len());

        self.vocab = vocab;
        self.vocab_index = vocab_index;
        self.idf = idf;
        self.is_fitted = true;

        let effective_dimension = if self.config.dim == 0 {
            self.vocab.len()
        } else {
            self.config.dim
        };

        Ok(FitReport {
            documents: corpus.len(),
            kept_terms: self.vocab.len(),
            raw_terms: raw_term_count,
            effective_dimension,
        })
    }

    pub fn embed(&self, text: &str) -> Result<primitives::Embedding, EmbedderError> {
        if !self.is_fitted {
            return Err(EmbedderError::NotFitted);
        }

        let tokens = primitives::tokenize_ascii_lower(text);
        let freq = primitives::term_frequency(&tokens);
        let mut vector = primitives::tfidf_dense(&freq, &self.vocab_index, &self.idf);

        if self.config.dim != 0 {
            vector = primitives::project_to_dimension(&vector, self.config.dim);
        }

        if self.config.normalize {
            primitives::l2_normalize(&mut vector);
        }

        Ok(vector)
    }

    pub fn embed_many<'a>(&self, corpus: &[&'a str]) -> Result<Vec<primitives::Embedding>, EmbedderError> {
        let mut vectors = Vec::with_capacity(corpus.len());
        for doc in corpus {
            vectors.push(self.embed(doc)?);
        }
        Ok(vectors)
    }

    pub fn search(
        &self,
        query: &str,
        corpus: &[primitives::Embedding],
        top_k: usize,
    ) -> Result<Vec<(usize, f32)>, EmbedderError> {
        let query_vec = self.embed(query)?;
        let mut scored: Vec<(usize, f32)> = corpus
            .iter()
            .enumerate()
            .map(|(idx, vector)| (idx, primitives::cosine_similarity(&query_vec, vector)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut results = scored;
        if results.len() > top_k {
            results.truncate(top_k);
        }
        Ok(results)
    }

    pub fn vocab(&self) -> &[String] {
        &self.vocab
    }
}
