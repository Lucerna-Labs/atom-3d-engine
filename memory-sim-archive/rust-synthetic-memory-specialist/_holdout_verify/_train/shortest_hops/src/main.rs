use std::io::Read;

use rust_ordo_ext::HopGraph;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut graph = HopGraph::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "link" => {
                if tokens.len() >= 3 {
                    graph.link(tokens[1], tokens[2]);
                }
            }
            "hops" => {
                if tokens.len() >= 3 {
                    let result = graph.shortest(tokens[1], tokens[2]);
                    let path = if result.path.is_empty() {
                        "none".to_string()
                    } else {
                        result.path.join(">")
                    };
                    println!("{} {} {}", result.reachable, path, result.hops);
                }
            }
            _ => {}
        }
    }
}
