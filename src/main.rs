use std::io::Result;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        style::Color,
    },
    layout::Alignment,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.make_chatw(), frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Left => match self.counter.checked_sub(1) {
                Some(n) => self.counter = n,
                None => self.counter = 0,
            },
            KeyCode::Right => match self.counter.checked_add(1) {
                Some(n) => self.counter = n,
                None => self.counter = 255,
            },
            _ => {}
        };
    }

    // Creates chat window
    fn make_chatw(&self) -> Paragraph {
        let block = Block::new()
            // contents of main window
            .title_style(Style::new().white())
            .title_top(" TUI IRC Client ")
            .title_alignment(Alignment::Center)
            // Main window
            .style(Style::default().bg(Color::Black.into()))
            .padding(Padding::symmetric(2, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        // counter with the value
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".to_string().white(),
            self.counter.to_string().white(),
        ])]);
        Paragraph::new(counter_text).block(block)
    }

    // Creates input field
    fn make_inputf(&self) {
        todo!()
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
