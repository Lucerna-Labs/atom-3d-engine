use std::io::Read;

use rust_ordo_ext::WeightRouter;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut router = WeightRouter::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "connect" => {
                if tokens.len() >= 4 {
                    let weight: i64 = tokens[3].parse().unwrap_or(0);
                    router.connect(tokens[1], tokens[2], weight);
                }
            }
            "route" => {
                if tokens.len() >= 2 {
                    let trace = router.route(tokens[1]);
                    let path = trace.path.join(">");
                    let signals = if trace.signals.is_empty() {
                        "none".to_string()
                    } else {
                        trace.signals.join(",")
                    };
                    println!("path={} total={} signals={}", path, trace.total, signals);
                }
            }
            _ => {}
        }
    }
}
