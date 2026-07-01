use rust_ordo_ext::PriorityMerge;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = PriorityMerge::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "first" => {
                let priority: i64 = parts[2].parse().unwrap_or(0);
                rt.first(parts[1], priority);
            }
            "second" => {
                let priority: i64 = parts[2].parse().unwrap_or(0);
                rt.second(parts[1], priority);
            }
            "merge" => {
                let result = rt.merge();
                let order = if result.order.is_empty() {
                    "none".to_string()
                } else {
                    result.order.join(">")
                };
                let signals = if result.signals.is_empty() {
                    "none".to_string()
                } else {
                    result.signals.join(",")
                };
                println!("order={} signals={}", order, signals);
            }
            _ => {}
        }
    }
}
