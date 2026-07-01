use rust_ordo_ext::ThrottleWindow;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = ThrottleWindow::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "limit" => {
                let limit: u32 = parts[1].parse().unwrap_or(0);
                rt.set_limit(limit);
            }
            "event" => {
                let plan = rt.event(parts[1]);
                println!(
                    "node={} count={} signals={}",
                    plan.node,
                    plan.count,
                    plan.signals.join(",")
                );
            }
            _ => {}
        }
    }
}
