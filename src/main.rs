#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
  // Uncomment this block to pass the first stage
  loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let args: Vec<&str> = input.trim().split(' ').collect();
    match args[0] {
      "exit" => {
        let code = args.get(1).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
        std::process::exit(code);
      }
      _ => println!("{}: command not found", input.trim()),
    }
  }
}
