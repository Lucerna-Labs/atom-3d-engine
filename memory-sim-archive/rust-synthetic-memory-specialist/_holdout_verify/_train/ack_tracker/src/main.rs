use rust_ordo_ext::AckTracker;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = AckTracker::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "send" => rt.send(parts[1]),
            "ack" => rt.ack(parts[1]),
            "pending" => {
                let report = rt.pending();
                let pending = if report.pending.is_empty() {
                    "none".to_string()
                } else {
                    report.pending.join(">")
                };
                println!("pending={} count={}", pending, report.count);
            }
            _ => {}
        }
    }
}
