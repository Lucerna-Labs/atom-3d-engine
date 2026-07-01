use rust_ordo_ext::Broker;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = Broker::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "publish" => {
                rt.publish(parts[1]);
                let pub_entry = rt.last();
                println!("thread={} msg={}", pub_entry.thread, pub_entry.payload);
            }
            "log" => {
                let log = rt.log();
                let entries = if log.entries.is_empty() {
                    "none".to_string()
                } else {
                    log.entries
                        .iter()
                        .map(|e| format!("{}>{}", e.thread, e.payload))
                        .collect::<Vec<String>>()
                        .join(",")
                };
                println!("entries={} count={}", entries, log.count);
            }
            _ => {}
        }
    }
}
