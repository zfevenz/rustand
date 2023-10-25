use std::env;
use std::fmt::format;
use std::io::stdout;
use std::process::id;

use crate::document;
use crate::Document;
use crate::Row;
use crate::Terminal;

use crossterm::style::Print;
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    event::{
        KeyCode::{self, Char},
        KeyEvent, KeyModifiers,
    },
    ExecutableCommand,
};

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

fn number_length(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut len = 0;
    while n != 0 {
        len += 1;
        n /= 10;
    }

    len
}

impl Editor {
    fn check_if_ctrl_char(key_event: KeyEvent, c: char) -> bool {
        key_event.code == Char(c) && key_event.modifiers == KeyModifiers::CONTROL
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(&filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document: document,
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        self.draw_rows();
        Terminal::cursor_position(&self.cursor_position);
        Terminal::cursor_show();
        if self.should_quit {
            print!("Exiting RustAnd!");
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let terminal_height = self.terminal.size().height;
        for terminal_row_idx in 0..terminal_height {
            Terminal::cursor_position(&Position {
                x: 0,
                y: terminal_row_idx as usize,
            });
            Terminal::clear_current_line();

            let formatted_line_number = format!(
                "{:width$} ",
                terminal_row_idx,
                width = number_length(terminal_height as usize),
            );
            Terminal::print(&formatted_line_number);
            if let Some(row) = self.document.row(terminal_row_idx as usize) {
                self.draw_row(row);
            }
        }
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row_to_be_rendered = row.render(start, end);
        Terminal::print(row_to_be_rendered);
    }

    pub fn die(e: std::io::Error) {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        panic!("{}", e);
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        let key_event = pressed_key;

        match key_event.code {
            KeyCode::Char('c') if Editor::check_if_ctrl_char(key_event, 'c') => {
                self.should_quit = true;
            }
            KeyCode::Char('q') if Editor::check_if_ctrl_char(key_event, 'q') => {
                self.should_quit = true;
            }
            KeyCode::Char(c) => {
                stdout().execute(Print(c))?;
            }
            KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::End
            | KeyCode::Home => {
                self.move_cursor(key_event.code);
            }
            _ => {}
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;
        let terminal_size = self.terminal.size();
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < terminal_size.height.saturating_sub(1) as usize {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < terminal_size.width as usize {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = terminal_size.height.saturating_sub(1) as usize,
            KeyCode::Home => x = 0,
            KeyCode::End => x = terminal_size.width as usize,
            _ => {}
        }
        self.cursor_position = Position { x, y }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                panic!("{}", error);
            }

            if self.should_quit {
                Terminal::clear_screen();
                Terminal::cursor_position(&Position::default());
                Terminal::print("Exiting RustAnd");
                break;
            }

            if let Err(error) = self.process_keypress() {
                panic!("{}", error);
            }
        }
    }
}
