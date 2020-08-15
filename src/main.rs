use std::env;
use std::process;
use diceroller::*;

fn main() {
    let rolls = parse_roll(env::args()).unwrap_or_else( |err| {
        eprintln!("Program exited with error: {}", err);
        process::exit(1);
    }); 

    throw_rolls(rolls);
}