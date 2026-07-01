use rust_ordo_ext::PriorityDrain;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = PriorityDrain::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "enqueue" => {
                let priority: i64 = parts[2].parse().unwrap_or(0);
                rt.enqueue(parts[1], priority);
            }
            "drain" => {
                let trace = rt.drain();
                let order = if trace.order.is_empty() {
                    "none".to_string()
                } else {
                    trace.order.join(">")
                };
                let signals = if trace.signals.is_empty() {
                    "none".to_string()
                } else {
                    trace.signals.join(",")
                };
                println!("order={} signals={}", order, signals);
            }
            _ => {}
        }
    }
}
