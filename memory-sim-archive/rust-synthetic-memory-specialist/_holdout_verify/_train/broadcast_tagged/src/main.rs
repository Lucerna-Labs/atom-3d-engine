use rust_ordo_ext::Mesh;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut mesh = Mesh::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "link" => mesh.link(parts[1], parts[2]),
            "tag" => mesh.tag(parts[1], parts[2]),
            "broadcast" => {
                let d = mesh.broadcast(parts[1], parts[2]);
                let deliveries = if d.deliveries.is_empty() {
                    "none".to_string()
                } else {
                    d.deliveries.join(",")
                };
                println!("tag={} deliveries={} count={}", d.tag, deliveries, d.count);
            }
            _ => {}
        }
    }
}
