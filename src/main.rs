use std::io::Result;

use ratatui::{
    crossterm::event::{read, Event, KeyCode, KeyEventKind},
    style::Stylize,
    widgets, DefaultTerminal,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = widgets::Paragraph::new("Hello, Ratatui! (press 'q' to quit)")
                .black()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;
        if let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
