use rust_ordo_ext::{Relay, Trace};

#[test]
fn relay_walks_and_signals() {
    let mut r = Relay::new();
    r.connect("a", "b");
    r.connect("b", "c");
    assert_eq!(
        r.route("a"),
        Trace { path: vec!["a".to_string(), "b".to_string(), "c".to_string()], signals: vec!["end:c".to_string()] }
    );
}
