use rust_ordo_ext::Relay;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut relay = Relay::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "connect" => relay.connect(parts[1], parts[2]),
            "route" => {
                let trace = relay.route(parts[1]);
                let signals = if trace.signals.is_empty() {
                    "none".to_string()
                } else {
                    trace.signals.join(",")
                };
                println!("path={} signals={}", trace.path.join(">"), signals);
            }
            _ => {}
        }
    }
}
