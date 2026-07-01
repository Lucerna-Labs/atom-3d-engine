use rust_ordo_ext::SignalBus;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut bus = SignalBus::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "collect" => bus.collect(parts[1]),
            "filter" => {
                let result = bus.filter(parts[1]);
                let signals = if result.signals.is_empty() {
                    "none".to_string()
                } else {
                    result.signals.join(">")
                };
                println!("prefix={} signals={} count={}", result.prefix, signals, result.count);
            }
            _ => {}
        }
    }
}
