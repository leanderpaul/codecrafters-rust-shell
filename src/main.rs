use std::io::{self, Write};

mod commands;
mod state;
mod utils;

fn execute_command(command: String, args: &[&str]) {
  let mut command = std::process::Command::new(command);
  command.args(args);
  let output = command.output().expect("Failed to execute command");
  if output.status.success() {
    print!("{}", String::from_utf8_lossy(&output.stdout));
  } else {
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
  }
}

fn collect_args(input: &str) -> Vec<String> {
  let mut parts = Vec::new();
  let mut current = String::new();
  let mut quote_char = None;

  for c in input.chars() {
    match c {
      '\'' | '"' => match quote_char {
        None => quote_char = Some(c),
        Some(q) if q == c => quote_char = None,
        Some(_) => current.push(c),
      },
      ' ' if quote_char.is_none() => {
        if !current.is_empty() {
          parts.push(current.clone());
          current.clear();
        }
      }
      _ => current.push(c),
    }
  }

  if !current.is_empty() {
    parts.push(current);
  }

  return parts;
}

fn main() {
  loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input_args = collect_args(&input.trim());
    let args: Vec<&str> = input_args.iter().map(|s| s.as_str()).collect();
    let cmd = args[0];
    let cmd_args = &args[1..];
    match cmd {
      "exit" => commands::cmd_exit::execute(cmd_args),
      "echo" => commands::cmd_echo::execute(cmd_args),
      "type" => commands::cmd_type::execute(cmd_args),
      "pwd" => commands::cmd_pwd::execute(cmd_args),
      "cd" => commands::cmd_cd::execute(cmd_args),
      _ => match utils::find_command_in_path(cmd) {
        Some(entry) => execute_command(entry.file_name().into_string().unwrap(), cmd_args),
        None => println!("{}: command not found", input.trim()),
      },
    }
  }
}
