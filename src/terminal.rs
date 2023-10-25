use std::{fmt::Display, io::stdout, default};

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyEvent},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::editor::Position;

#[derive(Debug, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::window_size()?;
        Ok(Self {
            size: Size {
                width: size.columns,
                height: size.rows,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All))
            .expect("Error while clearing screen using crossterm");
    }

    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        execute!(stdout(), MoveTo(x as u16, y as u16))
            .expect("Error while moving cursor position using crossterm");
    }

    pub fn read_key() -> Result<KeyEvent, std::io::Error> {
        loop {
            if let Event::Key(pressed_key) = read()? {
                return Ok(pressed_key);
            }
        }
    }

    pub fn print<T: Display>(item: T) {
        let formatted_string = format!("{}", item);
        execute!(stdout(), Print(formatted_string))
            .expect("Error while trying to print to terminal using crossterm");
    }

    pub fn cursor_show() {
        execute!(stdout(), crossterm::cursor::Show).expect("Error while toggling on Show Cursor");
    }

    pub fn cursor_hide() {
        execute!(stdout(), crossterm::cursor::Hide).expect("Error while toggling on Hide Cursor");
    }

    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine))
            .expect("Error while clearing current line using crossterm");
    }

}
