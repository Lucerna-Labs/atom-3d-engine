use std::io::Read;

use rust_ordo_ext::TraceLog;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut log = TraceLog::new();

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let cmd = match tokens.next() {
            Some(c) => c,
            None => continue,
        };
        match cmd {
            "record" => {
                let node = tokens.next().unwrap_or("");
                let kind = tokens.next().unwrap_or("");
                log.record(node, kind);
                println!("recorded={}@{}", kind, node);
            }
            "filter" => {
                let kind = tokens.next().unwrap_or("");
                let report = log.of_kind(kind);
                let traces = if report.items.is_empty() {
                    "none".to_string()
                } else {
                    report.items.join(",")
                };
                println!("filter={} traces={}", report.kind, traces);
            }
            _ => {}
        }
    }
}
