use std::io::stdout;

use crossterm::event::KeyCode::{self, Char};
use crossterm::event::{read, Event, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::enable_raw_mode;
use crossterm::ExecutableCommand;

#[derive(Debug)]
pub struct Editor {}

impl Editor {
    fn check_if_ctrl_char(&self, key_event: KeyEvent, c: char) -> bool {
        return key_event.code == Char(c) && key_event.modifiers == KeyModifiers::CONTROL;
    }

    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read().unwrap() {
                Event::Key(key_event) => {
                    if self.check_if_ctrl_char(key_event, 'c') {
                        break;
                    }
                    if let KeyCode::Char(c) = key_event.code {
                        stdout().execute(Print(c));
                    }
                }
                _ => {}
            }
        }
    }
}
