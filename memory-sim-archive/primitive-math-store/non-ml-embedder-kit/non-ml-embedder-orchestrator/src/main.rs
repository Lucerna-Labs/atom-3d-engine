use non_ml_embedder_orchestrator::{FitReport, NonMlEmbedder, NonMlEmbedderConfig};

fn main() {
    let config = NonMlEmbedderConfig {
        min_df: 1,
        max_terms: 200,
        dim: 64,
        normalize: true,
    };

    let mut embedder = NonMlEmbedder::new(config);

    let corpus = [
        "the red fox jumps over the lazy dog",
        "a fast fox runs through the hills",
        "embeddings can be built without any neural network",
        "tfidf is a classic non-ml document embedding method",
        "search this string using cosine similarity",
    ];

    let report: FitReport = match embedder.fit(&corpus) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("fit failed: {:?}", error);
            return;
        }
    };

    println!(
        "fit: docs={}, kept_terms={}, raw_terms={}, dim={}",
        report.documents,
        report.kept_terms,
        report.raw_terms,
        report.effective_dimension
    );

    let corpus_embeddings = match embedder.embed_many(&corpus) {
        Ok(v) => v,
        Err(error) => {
            eprintln!("embedding failed: {:?}", error);
            return;
        }
    };

    let query = "non ml document embedding search";
    let top = embedder.search(query, &corpus_embeddings, 3).unwrap_or_else(|error| {
        eprintln!("search failed: {:?}", error);
        Vec::new()
    });

    println!("query: \"{}\"", query);
    for (rank, (doc_idx, score)) in top.into_iter().enumerate() {
        println!("rank {}: doc={} score={:.4}", rank + 1, doc_idx, score);
        println!("  text: {}", corpus[doc_idx]);
    }
}
