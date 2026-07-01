use rust_ordo_ext::StateMachine;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut machine = StateMachine::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "event" => machine.event(parts[1], parts[2]),
            "state" => {
                let result = machine.state(parts[1]);
                println!("{}={}", result.node, result.state);
            }
            _ => {}
        }
    }
}
