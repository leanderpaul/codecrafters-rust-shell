use crate::state::STATE;

pub fn execute(_args: &[&str]) {
  println!("{}", STATE.read().unwrap().cwd.display());
}
