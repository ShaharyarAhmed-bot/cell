mod file;
mod editor;
mod row;
mod terminal;

pub use file::File;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;


fn main() {
    Editor::default().run();
}