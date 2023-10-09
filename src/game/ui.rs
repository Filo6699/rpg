use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    error::Error,
    io::{stdout, Stdout},
};

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

// pub fn render_border_type(paragraph: &Paragraph, title: &str, frame: &mut Frame, area: Rect) {
//     let block = Block::new()
//         .borders(Borders::ALL)
//         .border_type(BorderType::Rounded)
//         .title(Title::from(title.clone()).alignment(Alignment::Center));
//     frame.render_widget(paragraph.clone().block(block), area);
// }

// pub fn render_message(message: &String, frame: &mut Frame) {
//     let half = (std::cmp::max(message.len() / 2, 5)) as u16;
//     let area = Rect {
//         x: frame.size().width / 2 - half - 1,
//         y: frame.size().height / 2 - 2,
//         width: std::cmp::max(message.len() as u16 + 2, 15),
//         height: 4,
//     };
//     let paragraph = Paragraph::new(message.clone());
//     let block = Block::new()
//         .borders(Borders::ALL)
//         .border_type(BorderType::Rounded)
//         .title(Title::from("[Message]").alignment(Alignment::Center))
//         .title(
//             Title::from("[Press Enter]")
//                 .position(Position::Bottom)
//                 .alignment(Alignment::Center),
//         );
//     frame.render_widget(paragraph.clone().block(block), area);
// }

// pub fn run(terminal: &mut Terminal) -> Result<()> {
//     let mut game = Game::new();
//     loop {
//         if handle_events(&mut game)?.is_break() {
//             return Ok(());
//         }

//         terminal.draw(|f| ui(f, &mut game))?;
//     }
// }

// fn handle_events(game: &mut Game) -> Result<ControlFlow<()>> {
//     if event::poll(Duration::from_millis(100))? {
//         if let Event::Key(key) = event::read()? {
//             game.handle_key_press(key);
//             if let KeyCode::Char('q') = key.code {
//                 return Ok(ControlFlow::Break(()));
//             }
//         }
//     }
//     Ok(ControlFlow::Continue(()))
// }
