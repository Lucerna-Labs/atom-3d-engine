use rust_ordo_ext::CheckpointCounter;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = CheckpointCounter::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "record" => rt.record(parts[1]),
            "checkpoint" => rt.checkpoint(),
            "since" => {
                let report = rt.since();
                let events = if report.events.is_empty() {
                    "none".to_string()
                } else {
                    report.events.join(">")
                };
                println!("count={} events={}", report.count, events);
            }
            _ => {}
        }
    }
}
