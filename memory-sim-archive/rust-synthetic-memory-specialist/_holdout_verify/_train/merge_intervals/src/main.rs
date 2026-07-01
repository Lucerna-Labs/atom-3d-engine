use rust_ordo_ext::MergeIntervals;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = MergeIntervals::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "add" => {
                let start: i32 = parts[1].parse().unwrap_or(0);
                let end: i32 = parts[2].parse().unwrap_or(0);
                rt.add(start, end);
            }
            "merge" => {
                let merged = rt.merge();
                let line = if merged.is_empty() {
                    "none".to_string()
                } else {
                    merged
                        .iter()
                        .map(|iv| format!("{},{}", iv.start, iv.end))
                        .collect::<Vec<String>>()
                        .join(">")
                };
                println!("merged={}", line);
            }
            _ => {}
        }
    }
}
