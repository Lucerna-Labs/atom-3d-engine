use rust_ordo_ext::Buffer;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut buffer = Buffer::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "add" => buffer.add(parts[1], parts[2].parse().unwrap()),
            "sweep" => {
                let s = buffer.alive(parts[1].parse().unwrap());
                let alive = if s.alive.is_empty() {
                    "none".to_string()
                } else {
                    s.alive.join(">")
                };
                println!("now={} alive={} count={}", s.now, alive, s.count);
            }
            _ => {}
        }
    }
}
