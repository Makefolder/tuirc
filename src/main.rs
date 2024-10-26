use std::io::Result;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        style::Color,
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Default, Debug)]
enum AppMode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug, Default)]
pub struct App {
    mode: AppMode,
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(95), Constraint::Percentage(5)])
            .split(frame.area());
        frame.render_widget(self.make_chatw(), layout[0]);
        frame.render_widget(self.make_inputf(), layout[1]);
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
            KeyCode::Char('q') => match self.mode {
                AppMode::Normal => self.exit = true,
                _ => {}
            },
            KeyCode::Char('i') => match self.mode {
                AppMode::Normal => self.mode = AppMode::Insert,
                _ => {}
            },
            KeyCode::Esc => self.mode = AppMode::Normal,
            _ => {}
        };
    }

    // Creates chat window
    fn make_chatw(&self) -> Paragraph {
        let block = Block::new()
            // contents of main window
            .title_style(Style::new().white())
            .title_top("| TUI IRC Client |")
            .title_alignment(Alignment::Center)
            // Main window
            .style(Style::default().bg(Color::Black.into()))
            .padding(Padding::symmetric(1, 0))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        // counter with the value
        let counter_text = Text::from(vec![Line::from(vec![
            "author:".to_string().bold(),
            "their message.".to_string().white(),
        ])]);
        Paragraph::new(counter_text).block(block)
    }

    // Creates input field
    fn make_inputf(&self) -> Paragraph {
        let block = Block::new()
            .style(Style::default().bg(Color::Black.into()))
            .padding(Padding::symmetric(1, 0))
            .borders(Borders::ALL)
            .border_type(match self.mode {
                AppMode::Insert => BorderType::Thick,
                _ => BorderType::Plain,
            });
        let placeholder = match self.mode {
            AppMode::Normal => "Enter your message...",
            _ => "",
        };
        let inner_text = Text::from(placeholder).white().alignment(Alignment::Center);
        Paragraph::new(inner_text).block(block)
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
