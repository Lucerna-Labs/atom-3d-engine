use std::io::Read;

use rust_ordo_ext::RoundRobin;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut runtime = RoundRobin::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "add" => {
                if tokens.len() >= 2 {
                    runtime.add(tokens[1]);
                }
            }
            "assign" => {
                if tokens.len() >= 2 {
                    let count: usize = tokens[1].parse().unwrap_or(0);
                    let rotation = runtime.assign(count);
                    let assigned = if rotation.assignments.is_empty() {
                        "none".to_string()
                    } else {
                        rotation.assignments.join(">")
                    };
                    let signals = if rotation.signals.is_empty() {
                        "none".to_string()
                    } else {
                        rotation.signals.join(",")
                    };
                    println!("assigned={} signals={}", assigned, signals);
                }
            }
            _ => {}
        }
    }
}
