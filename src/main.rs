pub mod editor;

use std::io;

use editor::Editor;

fn main() -> io::Result<()> {
    let editor = Editor::default();
    editor.run();
    Ok(())
}
