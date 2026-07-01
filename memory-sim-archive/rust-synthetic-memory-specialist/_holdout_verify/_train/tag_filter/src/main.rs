use rust_ordo_ext::Catalog;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut catalog = Catalog::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "register" => catalog.register(parts[1], parts[2]),
            "select" => {
                let m = catalog.select(parts[1]);
                let nodes = if m.nodes.is_empty() {
                    "none".to_string()
                } else {
                    m.nodes.join(",")
                };
                println!("tag={} nodes={} count={}", m.tag, nodes, m.count);
            }
            _ => {}
        }
    }
}
