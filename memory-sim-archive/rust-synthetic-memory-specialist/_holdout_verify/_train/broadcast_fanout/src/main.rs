use rust_ordo_ext::Fabric;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut fabric = Fabric::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "link" => fabric.link(parts[1], parts[2]),
            "broadcast" => {
                let fanout = fabric.broadcast(parts[1]);
                let deliveries = if fanout.deliveries.is_empty() {
                    "none".to_string()
                } else {
                    fanout.deliveries.join(",")
                };
                println!("deliveries={} count={}", deliveries, fanout.count);
            }
            _ => {}
        }
    }
}
