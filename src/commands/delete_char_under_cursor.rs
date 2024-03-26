use crate::commands::Command;
use crate::text_document::TextDocument;
use crate::text_view::TextView;

pub struct DeleteCharUnderCursor {}

impl Command for DeleteCharUnderCursor {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        let pos = view.get_document_pos();
        doc.delete_char(pos)?;
        view.draw(doc)?;
        Ok(())
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        todo!()
    }
}