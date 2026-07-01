use rust_ordo_ext::Merger;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut merger = Merger::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "flow" => merger.flow(&parts[1..]),
            "merge" => {
                let path = merger.merge();
                let nodes = if path.nodes.is_empty() {
                    "none".to_string()
                } else {
                    path.nodes.join(">")
                };
                let signals = if path.signals.is_empty() {
                    "none".to_string()
                } else {
                    path.signals.join(",")
                };
                println!("path={} signals={}", nodes, signals);
            }
            _ => {}
        }
    }
}
