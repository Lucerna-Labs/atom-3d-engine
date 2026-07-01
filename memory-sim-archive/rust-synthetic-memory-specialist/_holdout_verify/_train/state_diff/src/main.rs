use std::io::Read;

use rust_ordo_ext::DiffFabric;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut fabric = DiffFabric::new();

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let cmd = match tokens.next() {
            Some(c) => c,
            None => continue,
        };
        match cmd {
            "before" => {
                let nodes: Vec<&str> = tokens.collect();
                fabric.before(&nodes);
            }
            "after" => {
                let nodes: Vec<&str> = tokens.collect();
                fabric.after(&nodes);
            }
            "diff" => {
                let result = fabric.diff();
                let added = if result.added.is_empty() {
                    "none".to_string()
                } else {
                    result.added.join(",")
                };
                let removed = if result.removed.is_empty() {
                    "none".to_string()
                } else {
                    result.removed.join(",")
                };
                println!("added={} removed={}", added, removed);
            }
            _ => {}
        }
    }
}
