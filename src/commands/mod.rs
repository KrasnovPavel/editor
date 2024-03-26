mod add_char;
mod add_line;
mod delete_char;
mod move_cursor;
mod composite_command;
mod delete_char_under_cursor;

use crate::text_document::TextDocument;
use crate::text_view::TextView;
use anyhow::Result;

pub use add_char::AddChar;
pub use add_line::AddLine;
pub use delete_char::DeleteChar;
pub use move_cursor::*;
pub use composite_command::CompositeCommand;
pub use delete_char_under_cursor::DeleteCharUnderCursor;

pub trait Command
{
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> Result<()>;
    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> Result<()>;
}