/// Embedding Primitives Module
/// 
/// Core traits and implementations for text embedding engines.

pub mod trait_def;
pub mod bge_m3;
pub mod mxbai_large;

// Re-export main trait
pub use trait_def::*;

// Re-export implementations
pub use bge_m3::{BgeM3Engine, bge_m3_metadata};
pub use mxbai_large::{MxbaiLargeEngine, mxbai_large_metadata, MXBAI_MATRYOSHKA_DIMS, MxbaiQuantization};

/// Factory function to create embedding engine by model ID
pub fn create_embedding_engine(model_id: &str) -> EmbeddingResult<Box<dyn EmbeddingEngine>> {
    match model_id {
        "BAAI/bge-m3" | "bge-m3" => {
            Ok(Box::new(BgeM3Engine::new(model_id)?))
        }
        "mixedbread-ai/mxbai-embed-large-v1" | "mxbai-large" => {
            Ok(Box::new(MxbaiLargeEngine::new(model_id)?))
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Unknown embedding model: {}", model_id),
        ))),
    }
}
