use std::io::Write;
use math_evaluator::eval;
use math_evaluator::state::State;

fn main() {
    let mut state = State {
        variables: std::collections::HashMap::new(),
    };
    loop {
        print!("\n> ");
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Unable to flash stdout: {}", e);
            continue;
        }
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Unable to read line: {}", e);
            continue;
        };
        let input = input.trim();
        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        let res = eval(input.chars(), &mut state);
        println!("{:?}", res);
    }
}