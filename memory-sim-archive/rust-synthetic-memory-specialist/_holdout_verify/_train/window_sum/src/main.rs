use rust_ordo_ext::WindowSum;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = WindowSum::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "window" => {
                let window: usize = parts[1].parse().unwrap_or(0);
                rt.set_window(window);
            }
            "push" => {
                let value: i64 = parts[2].parse().unwrap_or(0);
                rt.push(parts[1], value);
            }
            "report" => {
                let report = rt.report(parts[1]);
                let used = if report.used.is_empty() {
                    "none".to_string()
                } else {
                    report
                        .used
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                };
                println!("node={} sum={} used={}", report.node, report.sum, used);
            }
            _ => {}
        }
    }
}
