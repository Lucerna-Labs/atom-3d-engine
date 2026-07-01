use rust_ordo_ext::DeadletterRuntime;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = DeadletterRuntime::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "connect" => rt.connect(parts[1], parts[2]),
            "target" => rt.add_target(parts[1]),
            "route" => {
                let d = rt.route(parts[1]);
                let signals = if d.signals.is_empty() {
                    "none".to_string()
                } else {
                    d.signals.join(",")
                };
                println!(
                    "path={} delivered={} signals={}",
                    d.path.join(">"),
                    d.delivered,
                    signals
                );
            }
            _ => {}
        }
    }
}
