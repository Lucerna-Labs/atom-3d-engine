use rust_ordo_ext::Graph;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut graph = Graph::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "link" => graph.link(parts[1], parts[2]),
            "depth" => {
                let report = graph.depth(parts[1]);
                let signals = if report.signals.is_empty() {
                    "none".to_string()
                } else {
                    report.signals.join(",")
                };
                println!(
                    "hops={} path={} signals={}",
                    report.hops,
                    report.deepest.join(">"),
                    signals
                );
            }
            _ => {}
        }
    }
}
