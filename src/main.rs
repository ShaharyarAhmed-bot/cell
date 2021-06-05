#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod file;
mod editor;
mod highlighting;
mod row;
mod terminal;

pub use file::File;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;
pub use editor::SearchDirection;


fn main() {
    Editor::default().run();
}