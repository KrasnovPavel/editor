mod terminal_view;

pub use terminal_view::TerminalView;
use crate::text_document::TextDocument;
use anyhow::Result;
use crate::commands::Direction;
use crate::cursor_position::CursorPosition;

pub trait TextView {
    fn get_document_pos(&self) -> CursorPosition;
    fn move_cursor(&mut self, direction: Direction, doc: &dyn TextDocument) -> Result<()>;
    fn move_cursor_to(&mut self, pos: CursorPosition, doc: &dyn TextDocument) -> Result<()>;
    fn draw(&mut self, document: &dyn TextDocument) -> Result<()>;
}