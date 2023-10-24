mod editor;
mod terminal;
pub use editor::Editor;
pub use terminal::Terminal;

use std::io;

fn main() -> io::Result<()> {
    Editor::default().run();
    Ok(())
}
