use std::io::Read;

use rust_ordo_ext::MergeFabric;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut fabric = MergeFabric::new();

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let cmd = match tokens.next() {
            Some(c) => c,
            None => continue,
        };
        match cmd {
            "flow" => {
                let spec = tokens.next().unwrap_or("");
                let nodes: Vec<&str> = spec.split('>').filter(|s| !s.is_empty()).collect();
                let id = fabric.flow(&nodes);
                println!("thread={} route={}", id, nodes.join(">"));
            }
            "merge" => {
                let node = tokens.next().unwrap_or("");
                let trace = fabric.merge(node);
                let threads = if trace.threads.is_empty() {
                    "none".to_string()
                } else {
                    trace
                        .threads
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                };
                println!("merge={} threads={}", trace.node, threads);
            }
            _ => {}
        }
    }
}
