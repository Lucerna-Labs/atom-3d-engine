/// Core Embedding Engine Trait for Ordo
/// 
/// This trait defines the interface for text embedding engines.
/// All embedding primitives must implement this trait.

use std::error::Error;
use std::fmt::Debug;

/// Result type for embedding operations
pub type EmbeddingResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// A single embedding vector (dense representation)
pub type Embedding = Vec<f32>;

/// Batch of embeddings
pub type Embeddings = Vec<Embedding>;

/// Sparse embedding representation (for lexical retrieval)
#[derive(Debug, Clone)]
pub struct SparseEmbedding {
    /// Non-zero indices
    pub indices: Vec<u32>,
    /// Corresponding values
    pub values: Vec<f32>,
}

/// Multi-vector representation (for ColBERT-style late interaction)
#[derive(Debug, Clone)]
pub struct MultiVectorEmbedding {
    /// Token-level embeddings [num_tokens, dim]
    pub vectors: Vec<Embedding>,
}

/// Pooling strategy for converting token embeddings to sentence embeddings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoolingStrategy {
    /// Use [CLS] token embedding (default for BERT-style models)
    Cls,
    /// Mean pooling over all tokens with attention masking
    Mean,
    /// Max pooling over all tokens
    Max,
}

/// Precision format for embeddings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmbeddingPrecision {
    /// Full 32-bit float (default)
    Float32,
    /// 16-bit float (reduced memory)
    Float16,
    /// 8-bit integer quantization
    Int8,
    /// Binary quantization (1 bit per dimension)
    Binary,
}

/// Configuration for embedding generation
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    /// Output dimension (for Matryoshka-style truncation)
    pub dimensions: Option<usize>,
    /// Pooling strategy
    pub pooling: PoolingStrategy,
    /// Output precision
    pub precision: EmbeddingPrecision,
    /// Normalize embeddings to unit length
    pub normalize: bool,
    /// Maximum sequence length
    pub max_length: Option<usize>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            dimensions: None,
            pooling: PoolingStrategy::Cls,
            precision: EmbeddingPrecision::Float32,
            normalize: true,
            max_length: None,
        }
    }
}

/// Input types for embedding (query vs document)
#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddingInputType {
    /// Query embedding (may use different prompt/instruction)
    Query,
    /// Document/passages embedding
    Document,
}

/// Main embedding engine trait
pub trait EmbeddingEngine: Debug + Send + Sync {
    /// Get the model name/identifier
    fn model_name(&self) -> &str;
    
    /// Get the native embedding dimension
    fn dimension(&self) -> usize;
    
    /// Get the maximum sequence length
    fn max_length(&self) -> usize;
    
    /// Encode a single text into an embedding
    fn encode_one(&self, text: &str, config: &EmbeddingConfig) -> EmbeddingResult<Embedding>;
    
    /// Encode multiple texts into embeddings (batched)
    fn encode_batch(&self, texts: &[&str], config: &EmbeddingConfig) -> EmbeddingResult<Embeddings> {
        texts.iter()
            .map(|&text| self.encode_one(text, config))
            .collect()
    }
    
    /// Encode with input type differentiation (query vs document)
    fn encode_with_type(
        &self,
        text: &str,
        input_type: EmbeddingInputType,
        config: &EmbeddingConfig,
    ) -> EmbeddingResult<Embedding> {
        // Default implementation ignores input_type
        // Models that support different prompts for query/doc should override
        self.encode_one(text, config)
    }
    
    /// Generate sparse embeddings (lexical retrieval)
    /// Only implemented by models supporting multi-functionality (e.g., BGE-M3)
    fn encode_sparse(&self, _text: &str) -> EmbeddingResult<SparseEmbedding> {
        Err(Box::new(UnsupportedOperationError::SparseEmbedding))
    }
    
    /// Generate multi-vector embeddings (ColBERT-style)
    /// Only implemented by models supporting multi-functionality (e.g., BGE-M3)
    fn encode_multi_vector(&self, _text: &str) -> EmbeddingResult<MultiVectorEmbedding> {
        Err(Box::new(UnsupportedOperationError::MultiVectorEmbedding))
    }
    
    /// Compute cosine similarity between two embeddings
    fn cosine_similarity(a: &Embedding, b: &Embedding) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
    
    /// Compute similarity matrix between two sets of embeddings
    fn similarity_matrix(&self, embeddings_a: &Embeddings, embeddings_b: &Embeddings) -> Vec<Vec<f32>> {
        embeddings_a
            .iter()
            .map(|a| {
                embeddings_b
                    .iter()
                    .map(|b| Self::cosine_similarity(a, b))
                    .collect()
            })
            .collect()
    }
}

/// Error type for unsupported operations
#[derive(Debug)]
pub enum UnsupportedOperationError {
    SparseEmbedding,
    MultiVectorEmbedding,
    QuantizationNotSupported,
}

impl std::fmt::Display for UnsupportedOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SparseEmbedding => write!(f, "Sparse embeddings not supported by this model"),
            Self::MultiVectorEmbedding => write!(f, "Multi-vector embeddings not supported"),
            Self::QuantizationNotSupported => write!(f, "Quantization not supported"),
        }
    }
}

impl Error for UnsupportedOperationError {}

/// Metadata about an embedding model
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    /// Hugging Face model ID
    pub model_id: String,
    /// Model description
    pub description: String,
    /// Supported languages (ISO 639-1 codes)
    pub languages: Vec<String>,
    /// License
    pub license: String,
    /// Architecture (e.g., "BERT", "Gemma")
    pub architecture: String,
    /// Parameter count (approximate)
    pub parameters: Option<u64>,
    /// Release date
    pub release_date: Option<String>,
    /// Paper URL
    pub paper_url: Option<String>,
    /// MTEB average score
    pub mteb_score: Option<f32>,
}
