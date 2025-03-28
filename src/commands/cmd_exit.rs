use std::process;

pub fn execute(args: &[&str]) {
  let code: i32 = args.get(0).unwrap_or(&"0").parse().expect("Invalid exit code");
  process::exit(code);
}
