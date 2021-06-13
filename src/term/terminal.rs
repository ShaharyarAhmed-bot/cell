use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal}; 
use termion::color;

use crate::Position;

pub enum Mode {
    InsertMode(bool),
    CommandMode(bool)
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
    pub mode: Mode
}

impl Terminal {

    /// # Errors
    /// It will return a error if the terminal fails to switch into Raw Mode
    /// 
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },

            _stdout: stdout().into_raw_mode()?,
            mode: Mode::CommandMode(true),
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() { 
        print!("{}", termion::clear::All);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    #[allow(clippy::pattern_type_mismatch, clippy::as_conversions, clippy::cast_possible_truncation)]
    pub fn set_cursor_position(position: &Position) {
        let Position{ mut x, mut y } = position;

        x = x.saturating_add(1);
        y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;
        
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn switch_to_command_mode(&mut self) {
        print!("{}", termion::cursor::BlinkingBlock);
        
        self.mode = Mode::CommandMode(true);
    }

    pub fn switch_to_insert_mode(&mut self) {
        print!("{}", termion::cursor::BlinkingBar);

        self.mode = Mode::InsertMode(true);

    }

    /// # Errors
    /// If output stream flush failed it will return a Error
    /// 
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// # Errors
    /// Returns a error if there is some issue in reading the keys
    /// 
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
