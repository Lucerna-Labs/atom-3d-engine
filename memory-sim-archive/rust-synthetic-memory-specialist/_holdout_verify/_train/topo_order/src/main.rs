use std::io::Read;

use rust_ordo_ext::TopoRuntime;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut runtime = TopoRuntime::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "edge" => {
                if tokens.len() >= 3 {
                    runtime.edge(tokens[1], tokens[2]);
                }
            }
            "order" => {
                let result = runtime.order();
                let cycle = if result.cycle { "yes" } else { "no" };
                let order = if result.order.is_empty() {
                    "none".to_string()
                } else {
                    result.order.join(">")
                };
                let remaining = if result.remaining.is_empty() {
                    "none".to_string()
                } else {
                    result.remaining.join(",")
                };
                println!("cycle={} order={} remaining={}", cycle, order, remaining);
            }
            _ => {}
        }
    }
}
