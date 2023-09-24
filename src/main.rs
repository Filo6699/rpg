#[forbid(unsafe_code)]
use std::error::Error;

mod game;
mod tui;
use crate::tui::tui::run;
use crate::tui::utils::*;

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
