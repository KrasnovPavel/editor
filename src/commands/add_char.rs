use crate::commands::Command;
use crate::text_document::TextDocument;
use crate::text_view::TextView;
use anyhow::Result;
use crate::cursor_position::CursorPosition;

pub struct AddChar {
    pos: CursorPosition,
    ch: char,
}

impl AddChar {
    pub fn new(pos: CursorPosition, ch: char) -> AddChar {
        AddChar {
            pos,
            ch,
        }
    }
}

impl Command for AddChar {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> Result<()> {
        doc.add_char(self.pos, self.ch)?;
        view.draw(doc)?;
        Ok(())
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> Result<()> {
        todo!()
    }
}