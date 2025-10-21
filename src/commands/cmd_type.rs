use crate::utils;

pub fn execute(args: &[&str]) {
  let cmds = vec!["exit", "echo", "type", "pwd", "cd"];
  let cmd = args.get(0).expect("No command provided");

  if cmds.contains(&cmd) {
    println!("{} is a shell builtin", cmd);
    return;
  }

  match utils::find_command_in_path(cmd) {
    Some(entry) => println!("{} is {}", cmd, entry.path().display()),
    None => println!("{}: not found", cmd),
  }
}
