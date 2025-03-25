#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
  loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let args: Vec<&str> = input.trim().split(' ').collect();
    let cmd = args[0];
    let cmd_args = &args[1..];
    match cmd {
      "exit" => {
        let code: i32 = cmd_args.get(0).unwrap_or(&"0").parse().expect("Invalid exit code");
        std::process::exit(code);
      }

      "echo" => {
        println!("{}", cmd_args.join(" "));
      }

      _ => println!("{}: command not found", input.trim()),
    }
  }
}
