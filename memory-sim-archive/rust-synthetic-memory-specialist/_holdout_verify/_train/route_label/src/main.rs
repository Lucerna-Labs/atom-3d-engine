use rust_ordo_ext::RouteBook;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut book = RouteBook::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "assign" => {
                let label = parts[1];
                let stops: Vec<&str> = parts[2..].to_vec();
                book.assign(label, &stops);
            }
            "lookup" => {
                let entry = book.lookup(parts[1]);
                let route = if entry.stops.is_empty() {
                    "none".to_string()
                } else {
                    entry.stops.join(">")
                };
                println!("{}={}", entry.label, route);
            }
            _ => {}
        }
    }
}
