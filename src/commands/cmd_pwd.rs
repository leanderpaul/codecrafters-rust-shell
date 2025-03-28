pub fn execute(_args: &[&str]) {
  let current_dir = std::env::current_dir().unwrap();
  println!("{}", current_dir.display());
}
