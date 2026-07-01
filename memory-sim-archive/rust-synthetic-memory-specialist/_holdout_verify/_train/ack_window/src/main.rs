use rust_ordo_ext::AckWindow;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = AckWindow::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "send" => rt.send(parts[1], parts[2].parse().unwrap()),
            "ack" => rt.ack(parts[1], parts[2].parse().unwrap()),
            "report" => {
                let report = rt.report(parts[1].parse().unwrap());
                let unacked = if report.unacked.is_empty() {
                    "none".to_string()
                } else {
                    report.unacked.join(">")
                };
                println!("unacked={} count={}", unacked, report.count);
            }
            _ => {}
        }
    }
}
