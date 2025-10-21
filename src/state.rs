use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct State {
  pub cwd: PathBuf,
}

impl State {
  pub fn new() -> Self {
    State {
      cwd: std::env::current_dir().unwrap(),
    }
  }
}

lazy_static! {
  pub static ref STATE: Arc<RwLock<State>> = Arc::new(RwLock::new(State::new()));
}
