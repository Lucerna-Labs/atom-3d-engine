use rust_ordo_ext::LaneScheduler;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = LaneScheduler::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "node" => {
                let priority: u32 = parts[2].parse().unwrap_or(0);
                rt.add_node(parts[1], priority);
            }
            "schedule" => {
                let candidates: Vec<&str> = parts[1..].to_vec();
                if let Some(plan) = rt.schedule(&candidates) {
                    println!(
                        "node={} priority={} signals={}",
                        plan.node,
                        plan.priority,
                        plan.signals.join(",")
                    );
                }
            }
            _ => {}
        }
    }
}
