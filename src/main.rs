#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod fil;
mod edit;
mod term;

pub use crate::fil::file::File;
pub use crate::fil::filetype::FileType;
pub use crate::fil::filetype::HighlightingOptions;

use crate::edit::editor::Editor;
pub use crate::edit::editor::Position;
pub use crate::edit::row::Row;
pub use crate::term::terminal::Terminal;
pub use crate::term::terminal::Mode;
pub use crate::edit::editor::SearchDirection;


fn main() {
    Editor::default().run();
}