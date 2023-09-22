use std::{
    io::{stdout, Stdout},
    error::Error,
};
use ratatui::{prelude::*, widgets::{Paragraph, BorderType, Block, Borders}};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn setup_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}
  
pub fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn render_border_type(
    paragraph: &Paragraph,
    title: &String,
    border_type: BorderType,
    frame: &mut Frame,
    area: Rect,
  ) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(border_type)
        .title(title.clone());
    frame.render_widget(paragraph.clone().block(block), area);
}
