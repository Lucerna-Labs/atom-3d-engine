/// Core Reranker Engine Trait for Ordo
/// 
/// This trait defines the interface for text reranking engines (cross-encoders).
/// Rerankers take a query-document pair and output a relevance score directly,
/// unlike embedding models which produce vectors for similarity computation.

use std::error::Error;
use std::fmt::Debug;

/// Result type for reranking operations
pub type RerankerResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// A single relevance score
pub type RelevanceScore = f32;

/// Query-document pair with score
#[derive(Debug, Clone)]
pub struct RankedResult {
    /// Original index in the input list
    pub index: usize,
    /// Relevance score (higher = more relevant)
    pub score: RelevanceScore,
    /// The document text
    pub document: String,
}

/// Configuration for reranking
#[derive(Debug, Clone)]
pub struct RerankerConfig {
    /// Maximum sequence length (query + document combined)
    pub max_length: Option<usize>,
    /// Batch size for processing multiple pairs
    pub batch_size: usize,
    /// Use fp16 for faster inference
    pub use_fp16: bool,
    /// Top-k results to return (None = return all)
    pub top_k: Option<usize>,
}

impl Default for RerankerConfig {
    fn default() -> Self {
        Self {
            max_length: None,
            batch_size: 32,
            use_fp16: true,
            top_k: None,
        }
    }
}

/// Main reranker engine trait
pub trait RerankerEngine: Debug + Send + Sync {
    /// Get the model name/identifier
    fn model_name(&self) -> &str;
    
    /// Get the maximum sequence length
    fn max_length(&self) -> usize;
    
    /// Score a single query-document pair
    fn score_pair(&self, query: &str, document: &str, config: &RerankerConfig) -> RerankerResult<RelevanceScore>;
    
    /// Score multiple query-document pairs (batched)
    fn score_batch(
        &self,
        pairs: &[(&str, &str)],
        config: &RerankerConfig,
    ) -> RerankerResult<Vec<RelevanceScore>> {
        pairs
            .iter()
            .map(|&(q, d)| self.score_pair(q, d, config))
            .collect()
    }
    
    /// Score and rank documents for a query
    fn rank(
        &self,
        query: &str,
        documents: &[&str],
        config: &RerankerConfig,
    ) -> RerankerResult<Vec<RankedResult>> {
        let pairs: Vec<(&str, &str)> = documents.iter().map(|&d| (query, d)).collect();
        let scores = self.score_batch(&pairs, config)?;
        
        let mut results: Vec<RankedResult> = documents
            .iter()
            .zip(scores.iter())
            .enumerate()
            .map(|(idx, (&doc, &score))| RankedResult {
                index: idx,
                score,
                document: doc.to_string(),
            })
            .collect();
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Apply top-k if specified
        if let Some(k) = config.top_k {
            results.truncate(k);
        }
        
        Ok(results)
    }
    
    /// Rerank existing search results (with metadata)
    fn rerank_with_metadata<T: Clone>(
        &self,
        query: &str,
        documents: &[(&str, T)],
        config: &RerankerConfig,
    ) -> RerankerResult<Vec<(RankedResult, T)>> {
        let texts: Vec<&str> = documents.iter().map(|(d, _)| *d).collect();
        let ranked = self.rank(query, &texts, config)?;
        
        let results = ranked
            .into_iter()
            .map(|r| {
                let metadata = documents[r.index].1.clone();
                (r, metadata)
            })
            .collect();
        
        Ok(results)
    }
}

/// Error type for reranking operations
#[derive(Debug)]
pub enum RerankerError {
    /// Input too long for model's context window
    InputTooLong { actual: usize, max: usize },
    /// Empty input
    EmptyInput,
    /// Model loading failed
    ModelLoadFailed(String),
    /// Inference error
    InferenceFailed(String),
}

impl std::fmt::Display for RerankerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooLong { actual, max } => {
                write!(f, "Input length {} exceeds maximum {}", actual, max)
            }
            Self::EmptyInput => write!(f, "Input cannot be empty"),
            Self::ModelLoadFailed(msg) => write!(f, "Model load failed: {}", msg),
            Self::InferenceFailed(msg) => write!(f, "Inference failed: {}", msg),
        }
    }
}

impl Error for RerankerError {}

/// Metadata about a reranker model
#[derive(Debug, Clone)]
pub struct RerankerMetadata {
    /// Hugging Face model ID
    pub model_id: String,
    /// Model description
    pub description: String,
    /// Supported languages (ISO 639-1 codes)
    pub languages: Vec<String>,
    /// License
    pub license: String,
    /// Architecture (e.g., "BERT", "Gemma", "MiniCPM")
    pub architecture: String,
    /// Parameter count (approximate)
    pub parameters: Option<u64>,
    /// Release date
    pub release_date: Option<String>,
    /// Paper URL
    pub paper_url: Option<String>,
    /// BEIR average score
    pub beir_score: Option<f32>,
    /// C-MTEB retrieval score
    pub cmteb_score: Option<f32>,
    /// Supports token compression
    pub supports_token_compression: bool,
    /// Supports layerwise selection
    pub supports_layerwise: bool,
}
