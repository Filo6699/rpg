use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
};
use std::{
    error::Error,
    io::{stdout, Stdout},
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

pub fn render_border_type(paragraph: &Paragraph, title: &String, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title.clone());
    frame.render_widget(paragraph.clone().block(block), area);
}

pub fn render_message(message: &String, frame: &mut Frame) {
    let half = (std::cmp::max(message.len() / 2, 5)) as u16;
    let area = Rect {
        x: frame.size().width / 2 - half - 1,
        y: frame.size().height / 2 - 2,
        width: std::cmp::max(message.len() as u16 + 2, 13),
        height: 4,
    };
    let paragraph = Paragraph::new(message.clone());
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(String::from("Message"))
        .title(
            Title::from("Press Enter")
                .position(Position::Bottom)
                .alignment(Alignment::Center),
        );
    frame.render_widget(paragraph.clone().block(block), area);
}
