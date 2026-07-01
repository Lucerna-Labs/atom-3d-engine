/// Reranker Primitives Module
/// 
/// Core traits and implementations for text reranking engines (cross-encoders).

pub mod trait_def;
pub mod bge_reranker_v2_m3;
pub mod bge_reranker_lightweight;

// Re-export main trait
pub use trait_def::*;

// Re-export implementations
pub use bge_reranker_v2_m3::{BgeRerankerV2M3, bge_reranker_v2_m3_metadata};
pub use bge_reranker_lightweight::{
    BgeRerankerGemma2Lightweight, 
    gemma2_lightweight_metadata, 
    Gemma2LightweightConfig
};

/// Factory function to create reranker engine by model ID
pub fn create_reranker_engine(model_id: &str) -> RerankerResult<Box<dyn RerankerEngine>> {
    match model_id {
        "BAAI/bge-reranker-v2-m3" | "bge-reranker-v2-m3" => {
            Ok(Box::new(BgeRerankerV2M3::new(model_id)?))
        }
        "BAAI/bge-reranker-v2.5-gemma2-lightweight" | "bge-reranker-gemma2-lightweight" => {
            Ok(Box::new(BgeRerankerGemma2Lightweight::new(model_id)?))
        }
        "BAAI/bge-reranker-large" | "bge-reranker-large" => {
            // TODO: Implement BGE Reranker Large
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotImplemented,
                "BGE Reranker Large not yet implemented",
            )))
        }
        "BAAI/bge-reranker-base" | "bge-reranker-base" => {
            // TODO: Implement BGE Reranker Base
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotImplemented,
                "BGE Reranker Base not yet implemented",
            )))
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Unknown reranker model: {}", model_id),
        ))),
    }
}
