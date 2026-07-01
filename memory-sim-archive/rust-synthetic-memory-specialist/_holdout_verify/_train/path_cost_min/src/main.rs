use std::io::Read;

use rust_ordo_ext::CostGraph;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut graph = CostGraph::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "connect" => {
                if tokens.len() >= 4 {
                    let cost: i32 = tokens[3].parse().unwrap_or(0);
                    graph.connect(tokens[1], tokens[2], cost);
                }
            }
            "cheapest" => {
                if tokens.len() >= 2 {
                    let source = tokens[1];
                    let candidates: Vec<&str> = tokens[2..].to_vec();
                    let pick = graph.cheapest(source, &candidates);
                    let signals = if pick.signals.is_empty() {
                        "none".to_string()
                    } else {
                        pick.signals.join(",")
                    };
                    println!("target={} cost={} signals={}", pick.target, pick.cost, signals);
                }
            }
            _ => {}
        }
    }
}
