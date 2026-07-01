use rust_ordo_ext::Broker;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut broker = Broker::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "subscribe" => broker.subscribe(parts[1], parts[2]),
            "publish" => {
                let result = broker.publish(parts[1]);
                let subscribers = if result.subscribers.is_empty() {
                    "none".to_string()
                } else {
                    result.subscribers.join(",")
                };
                println!("topic={} subscribers={}", result.topic, subscribers);
            }
            _ => {}
        }
    }
}
