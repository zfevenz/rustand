mod editor;
mod terminal;
mod document;
mod row;

pub use editor::Editor;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;
use std::io;

fn main() -> io::Result<()> {
    Editor::default().run();
    Ok(())
}
