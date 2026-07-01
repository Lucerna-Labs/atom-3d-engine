use non_ml_embedder_orchestrator::{NonMlEmbedder, NonMlEmbedderConfig};

#[test]
fn orchestrator_fit_embed_search_is_stable() {
    let docs = [
        "apple banana apple",
        "banana fruit salad",
        "orange and fruit",
        "apple orange",
    ];

    let mut embedder = NonMlEmbedder::new(NonMlEmbedderConfig {
        min_df: 1,
        max_terms: 32,
        dim: 4,
        normalize: true,
    });

    let report = embedder.fit(&docs).expect("fit should succeed");
    assert_eq!(report.documents, docs.len());
    assert!(report.kept_terms >= 3);
    assert_eq!(report.effective_dimension, 4);

    let vectors = embedder.embed_many(&docs).expect("embed_many should work after fit");
    assert_eq!(vectors.len(), docs.len());
    assert_eq!(vectors[0].len(), 4);

    let results = embedder.search("banana salad", &vectors, 2).expect("search should work");
    assert!(!results.is_empty());
    assert_eq!(results[0].0, 1);
}

#[test]
fn orchestrator_requires_fit_before_use() {
    let embedder = NonMlEmbedder::new(NonMlEmbedderConfig::default());
    assert!(embedder.embed("test").is_err());
}
