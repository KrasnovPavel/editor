mod string_document;

pub use string_document::StringDocument;
use anyhow::Result;
use crate::cursor_position::CursorPosition;

pub trait TextDocument
{
    fn amount_of_lines(&self) -> usize;
    fn get_line(&self, row: usize) -> Result<String>;
    fn get_line_length(&self, row: usize) -> Result<usize>;
    fn get_lines(&self, first_row: usize, last_row: usize) -> Result<Vec<String>>;
    fn add_char(&mut self, pos: CursorPosition, ch: char) -> Result<()>;
    fn add_line(&mut self, pos: CursorPosition) -> Result<()>;
    fn delete_char(&mut self, pos: CursorPosition) -> Result<()>;
}