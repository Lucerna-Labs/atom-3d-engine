use rust_ordo_ext::Ledger;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut ledger = Ledger::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "record" => ledger.record(parts[1], parts[2].parse().unwrap()),
            "expire" => {
                let expiry = ledger.expire(parts[1].parse().unwrap());
                let expired = if expiry.expired.is_empty() {
                    "none".to_string()
                } else {
                    expiry.expired.join(",")
                };
                let active = if expiry.active.is_empty() {
                    "none".to_string()
                } else {
                    expiry.active.join(",")
                };
                println!("expired={} active={}", expired, active);
            }
            _ => {}
        }
    }
}
