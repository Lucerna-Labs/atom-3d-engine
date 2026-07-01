use rust_ordo_ext::Graph;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut graph = Graph::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "link" => graph.link(parts[1], parts[2]),
            "degree" => {
                let degree = graph.degree(parts[1]);
                println!("out={} in={}", degree.out_degree, degree.in_degree);
            }
            _ => {}
        }
    }
}
