use std::io::Read;

use rust_ordo_ext::RetryRuntime;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut runtime = RetryRuntime::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "add" => {
                if tokens.len() >= 3 {
                    let budget: i32 = tokens[2].parse().unwrap_or(0);
                    runtime.add(tokens[1], budget);
                }
            }
            "attempt" => {
                if tokens.len() >= 3 {
                    runtime.attempt(tokens[1], tokens[2]);
                }
            }
            "resolve" => {
                let resolution = runtime.resolve();
                let signals = if resolution.signals.is_empty() {
                    "none".to_string()
                } else {
                    resolution.signals.join(",")
                };
                println!("winner={} signals={}", resolution.winner, signals);
            }
            _ => {}
        }
    }
}
