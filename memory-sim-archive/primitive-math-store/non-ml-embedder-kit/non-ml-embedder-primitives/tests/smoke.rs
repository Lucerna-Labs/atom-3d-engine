use non_ml_embedder_primitives::{
    cosine_similarity,
    document_frequency,
    idf_vector,
    l2_normalize,
    project_to_dimension,
    select_vocab,
    term_frequency,
    tfidf_dense,
    tokenize_ascii_lower,
};

#[test]
fn tokenize_and_vocab_flow() {
    let tokens = tokenize_ascii_lower("Hello, world! Hello Rust.");
    assert_eq!(tokens, vec!["hello", "world", "hello", "rust"]);

    let tf = term_frequency(&tokens);
    assert_eq!(tf.get("hello"), Some(&2));

    let df = document_frequency(&[tf.clone()]);
    let vocab = select_vocab(&df, 1, Some(3));
    assert_eq!(vocab, vec!["hello", "rust", "world"]);

    let idf = idf_vector(&vocab, &df, 1);
    assert_eq!(idf.len(), 3);
    assert!(idf.iter().all(|v| *v > 0.0));

    let index: std::collections::HashMap<String, usize> =
        vocab.iter().enumerate().map(|(i, t)| (t.clone(), i)).collect();
    let dense = tfidf_dense(&tf, &index, &idf);
    assert_eq!(dense.len(), 3);

    let projected = project_to_dimension(&dense, 2);
    assert_eq!(projected.len(), 2);

    let mut normed = projected.clone();
    l2_normalize(&mut normed);
    let norm: f32 = normed.iter().map(|v| v * v).sum();
    assert!((norm - 1.0).abs() < 1e-5);

    let mut identical = dense.clone();
    l2_normalize(&mut identical);
    let self_similarity = cosine_similarity(&identical, &identical);
    assert!((self_similarity - 1.0).abs() < 1e-6);
}

#[test]
fn cosine_distance_for_orthogonal_vectors() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![0.0, 1.0, 0.0];
    let value = cosine_similarity(&a, &b);
    assert!(value.abs() < 1e-6);
}
