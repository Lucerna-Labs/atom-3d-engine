use rust_ordo_ext::SignalLog;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut log = SignalLog::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "collect" => {
                if parts.len() > 1 {
                    log.collect(parts[1]);
                }
            }
            "report" => {
                let r = log.report();
                let signals = if r.signals.is_empty() {
                    "none".to_string()
                } else {
                    r.signals.join(",")
                };
                println!("signals={} count={}", signals, r.count);
            }
            _ => {}
        }
    }
}
