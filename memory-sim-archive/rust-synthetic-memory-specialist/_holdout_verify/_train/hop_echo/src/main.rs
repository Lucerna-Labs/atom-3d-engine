use rust_ordo_ext::Walk;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut walk = Walk::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "link" => walk.link(parts[1], parts[2]),
            "hop" => {
                let echo = walk.hop(parts[1]);
                println!("terminal={} path={}", echo.terminal, echo.path.join(">"));
            }
            _ => {}
        }
    }
}
