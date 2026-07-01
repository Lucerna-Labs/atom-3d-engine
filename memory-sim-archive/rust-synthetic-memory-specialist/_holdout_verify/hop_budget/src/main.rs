use std::io::Read;

use rust_ordo_ext::HopRuntime;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut runtime = HopRuntime::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "connect" => {
                if tokens.len() >= 3 {
                    runtime.connect(tokens[1], tokens[2]);
                }
            }
            "flow" => {
                if tokens.len() >= 3 {
                    let start = tokens[1];
                    let budget: usize = tokens[2].parse().unwrap_or(0);
                    let trace = runtime.flow(start, budget);
                    let path = trace.path.join(">");
                    let signals = if trace.signals.is_empty() {
                        "none".to_string()
                    } else {
                        trace.signals.join(",")
                    };
                    println!("path={} signals={}", path, signals);
                }
            }
            _ => {}
        }
    }
}
