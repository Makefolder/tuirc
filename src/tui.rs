use ratatui::layout::Constraint;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        style::Color,
    },
    layout::{Alignment, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use std::io::Result;

const CLIENT_NAME: &str = "[ TUI IRC Client ]";
const USER_NICKNAME: &str = "makefolder";
const MESSAGE_PLACEHOLDER: &str = "Enter your message...";

#[derive(Default, Debug)]
#[allow(unused)]
enum ActiveWin {
    #[default]
    Chat,
    Input,
    Explorer,
}

#[derive(Default, Debug)]
enum AppMode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug, Default)]
pub struct App {
    active: ActiveWin,
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
        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(100 - 25),
            ])
            .split(frame.area());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(95), Constraint::Percentage(5)])
            .split(horizontal_layout[1]);
        frame.render_widget(self.make_explorer(), horizontal_layout[0]);
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

    // Keybindings & navigation
    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            // Quit client
            // TODO change/remove ltr
            KeyCode::Char('q') => match self.mode {
                AppMode::Normal => self.exit = true,
                _ => {}
            },
            // Enter INSERT mode
            KeyCode::Char('i') => match self.mode {
                AppMode::Normal => self.mode = AppMode::Insert,
                _ => {}
            },
            // Next component
            KeyCode::Tab => match self.mode {
                AppMode::Normal => match self.active {
                    ActiveWin::Chat => self.active = ActiveWin::Input,
                    ActiveWin::Input => self.active = ActiveWin::Explorer,
                    ActiveWin::Explorer => self.active = ActiveWin::Chat,
                },
                _ => {}
            },
            // Previous component
            KeyCode::BackTab => match self.mode {
                AppMode::Normal => match self.active {
                    ActiveWin::Input => self.active = ActiveWin::Chat,
                    ActiveWin::Explorer => self.active = ActiveWin::Input,
                    ActiveWin::Chat => self.active = ActiveWin::Explorer,
                },
                _ => {}
            },
            KeyCode::Esc => self.mode = AppMode::Normal,
            _ => match self.mode {
                AppMode::Insert => todo!(), // Keyboard input
                _ => {}
            },
        };
    }

    fn make_explorer(&self) -> Paragraph {
        let is_active = match self.mode {
            AppMode::Normal => match self.active {
                ActiveWin::Explorer => true,
                _ => false,
            },
            _ => false,
        };
        Paragraph::new(if is_active {
            Text::from("Channel #1").bold()
        } else {
            Text::from("Channel #1")
        })
        .block(
            Block::new()
                .title_top(if is_active {
                    "[ Channels: 1 ]".bold()
                } else {
                    "[ Channels: 1 ]".not_bold()
                })
                .style(Style::default().bg(Color::Black.into()))
                .padding(Padding::symmetric(1, 0))
                .borders(Borders::ALL)
                .border_type(if is_active {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                }),
        )
    }

    // Creates chat window
    fn make_chatw(&self) -> Paragraph {
        let is_active = match self.mode {
            AppMode::Normal => match self.active {
                ActiveWin::Chat => true,
                _ => false,
            },
            _ => false,
        };
        let chat_content = Text::from(vec![Line::from(vec![
            "12:47:53 ".to_string().italic().gray(),
            "author: ".to_string().bold().white(),
            "This text would be their long long message."
                .to_string()
                .white(),
        ])]);
        Paragraph::new(chat_content).block(
            Block::new()
                // contents of main window
                .title_style(Style::new().white())
                .title_alignment(Alignment::Right)
                .title_top(if is_active {
                    Line::from(vec![
                        "[ ".bold().into(),
                        USER_NICKNAME.bold().italic().into(),
                        " ]".bold().into(),
                    ])
                } else {
                    Line::from(vec![
                        "[ ".into(),
                        USER_NICKNAME.italic().into(),
                        " ]".into(),
                    ])
                })
                .title_top(match self.active {
                    ActiveWin::Chat => match self.mode {
                        AppMode::Normal => CLIENT_NAME.bold(),
                        _ => CLIENT_NAME.into(),
                    },
                    _ => CLIENT_NAME.into(),
                })
                // Main window
                .style(Style::default().bg(Color::Black.into()))
                .padding(Padding::symmetric(1, 0))
                .borders(Borders::ALL)
                .border_type(if is_active {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                }),
        )
    }

    // Creates input field
    fn make_inputf(&self) -> Paragraph {
        let is_active = match self.mode {
            AppMode::Insert => true,
            _ => match self.active {
                ActiveWin::Input => true,
                _ => false,
            },
        };
        let block = Block::new()
            .style(Style::default().bg(Color::Black.into()))
            .padding(Padding::symmetric(1, 0))
            .borders(Borders::ALL)
            .border_type(if is_active {
                BorderType::Thick
            } else {
                BorderType::Plain
            });
        let placeholder = match self.mode {
            AppMode::Insert => "",
            _ => MESSAGE_PLACEHOLDER,
        };
        let inner_text = Text::from(match self.mode {
            AppMode::Normal => match self.active {
                ActiveWin::Input => placeholder.white().bold(),
                _ => placeholder.gray(),
            },
            _ => placeholder.white(),
        })
        .white()
        .alignment(Alignment::Center);
        Paragraph::new(inner_text).block(block)
    }
}
