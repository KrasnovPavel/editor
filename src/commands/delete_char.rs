use crate::commands::Command;
use crate::cursor_position::CursorPosition;
use crate::text_document::TextDocument;
use crate::text_view::TextView;

pub struct DeleteChar {
    pos: CursorPosition,
}

impl DeleteChar {
    pub fn new(pos: CursorPosition) -> DeleteChar {
        DeleteChar {
            pos
        }
    }
}

impl Command for DeleteChar {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        doc.delete_char(self.pos)?;
        view.draw(doc)?;
        Ok(())
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        todo!()
    }
}