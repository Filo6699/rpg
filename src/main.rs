#[forbid(unsafe_code)]
use std::error::Error;

mod tui;
mod game;
use crate::tui::utils::*;
use crate::tui::tui::run;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut terminal = setup_terminal()?;
  let result = run(&mut terminal);
  restore_terminal(terminal)?;

  if let Err(err) = result {
      eprintln!("{err:?}");
  }
  Ok(())
}
