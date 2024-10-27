use std::{env::args, io::Result};
use tui::App;

mod tui;

#[derive(Debug)]
enum ArgInput {
    Help,
    InvalidArgLength,
}

impl ArgInput {
    pub fn value(&self) -> &str {
        match self {
            Self::InvalidArgLength => "Invalid argument length. Use --help (-h) to see docs.",
            Self::Help => "Usage:\n\ttuirc <nickname> <host> <port> <optional user description>",
        }
    }
}

#[allow(dead_code)]
struct UserArgs<'a> {
    nickname: &'a str,
    host: &'a str,
    port: &'a str,
    usr_desc: Option<&'a str>,
}

fn main() -> Result<()> {
    // tuirc <nickname> <host> <port> <optional user description>
    // tuirc makefolduh irc.freenode.net 6667
    // tuirc somebody_else irc.eu.libera.chat 6697 "My Optional Description"
    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        if args.len() == 2
            && (args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()))
        {
            let msg = ArgInput::Help.value();
            println!("{}", msg);
            return Ok(());
        }
        println!("arg length: {}", args.len());
        let msg = ArgInput::InvalidArgLength.value();
        panic!("{}", msg);
    }

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
