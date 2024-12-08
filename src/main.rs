#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let exit: bool = false;
    // Uncomment this block to pass the first stage
    while !exit {
        
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    
    println!("{}: not found", input.trim());
    }
}
