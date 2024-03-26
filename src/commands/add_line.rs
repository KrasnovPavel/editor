use crate::commands::Command;
use crate::cursor_position::CursorPosition;
use crate::text_document::TextDocument;
use crate::text_view::TextView;

pub struct AddLine {
    pos: CursorPosition,
}

impl AddLine {
    pub fn new(pos: CursorPosition) -> Self {
        AddLine {
            pos
        }
    }
}

impl Command for AddLine {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        doc.add_line(self.pos)?;
        view.draw(doc)?;
        Ok(())
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        todo!()
    }
}