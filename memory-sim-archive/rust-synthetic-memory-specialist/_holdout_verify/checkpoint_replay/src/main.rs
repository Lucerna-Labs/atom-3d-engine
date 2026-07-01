use std::io::Read;

use rust_ordo_ext::ReplayLog;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut log = ReplayLog::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(&c) => c,
            None => continue,
        };

        match command {
            "record" => {
                let kind = tokens.get(1).copied().unwrap_or("");
                let node = tokens.get(2).copied().unwrap_or("");
                log.record(kind, node);
            }
            "checkpoint" => {
                log.checkpoint();
            }
            "replay" => {
                let events = log.replay();
                let body = if events.is_empty() {
                    "none".to_string()
                } else {
                    events
                        .iter()
                        .map(|e| format!("{}:{}@{}", e.index, e.kind, e.node))
                        .collect::<Vec<String>>()
                        .join(",")
                };
                println!("replay={}", body);
            }
            _ => {}
        }
    }
}
