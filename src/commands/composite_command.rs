use crate::commands::Command;
use crate::text_document::TextDocument;
use crate::text_view::TextView;

pub struct CompositeCommand(Vec<Box<dyn Command>>);

impl CompositeCommand {
    pub fn new(commands: Vec<Box<dyn Command>>) -> CompositeCommand
    {
        CompositeCommand(commands)
    }
}

impl Command for CompositeCommand {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        for command in self.0.iter() {
            command.execute(doc, view)?
        }
        Ok(())
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        for command in self.0.iter().rev() {
            command.undo(doc, view)?
        }
        Ok(())
    }
}