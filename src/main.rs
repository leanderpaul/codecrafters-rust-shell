use std::io::{self, Write};
use std::path::Path;

fn cmd_exit(args: &[&str]) {
  let code: i32 = args.get(0).unwrap_or(&"0").parse().expect("Invalid exit code");
  std::process::exit(code);
}

fn cmd_echo(args: &[&str]) {
  println!("{}", args.join(" "));
}

fn cmd_type(args: &[&str]) {
  let cmds = vec!["exit", "echo", "type"];
  let cmd = args.get(0).expect("No command provided");

  if cmds.contains(&cmd) {
    println!("{} is a shell builtin", cmd);
    return;
  }

  let path = std::env::var("PATH").unwrap_or_default();
  let paths: Vec<&str> = path.split(':').collect();
  for p in paths {
    let path = Path::new(p);
    if !path.is_dir() {
      continue;
    }

    for file in path.read_dir().unwrap() {
      let file = file.unwrap();
      if file.file_name().eq(cmd) {
        println!("{} is {}", cmd, file.path().display());
        return;
      }
    }
  }

  println!("{}: not found", cmd);
}

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
      "exit" => cmd_exit(cmd_args),
      "echo" => cmd_echo(cmd_args),
      "type" => cmd_type(cmd_args),
      _ => println!("{}: command not found", input.trim()),
    }
  }
}
