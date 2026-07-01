use std::io::Read;

use rust_ordo_ext::CycleRuntime;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let mut runtime = CycleRuntime::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = match tokens.first() {
            Some(cmd) => *cmd,
            None => continue,
        };

        match command {
            "connect" => {
                if tokens.len() >= 3 {
                    runtime.connect(tokens[1], tokens[2]);
                }
            }
            "walk" => {
                if tokens.len() >= 2 {
                    let start = tokens[1];
                    let trace = runtime.walk(start);
                    let cycle = if trace.cycle { "yes" } else { "no" };
                    let path = trace.path.join(">");
                    println!(
                        "cycle={} repeated={} path={}",
                        cycle, trace.repeated, path
                    );
                }
            }
            _ => {}
        }
    }
}
