use crate::input_processor::{InputProcessor, ProcessorOutput};
use crate::text_document::TextDocument;
use crate::text_view::TextView;
use std::ops::{Deref, DerefMut};

pub struct Editor {
    document: Box<dyn TextDocument>,
    view: Box<dyn TextView>,
    processor: Box<dyn InputProcessor>,
}

impl Editor {
    pub fn new(
        document: Box<dyn TextDocument>,
        view: Box<dyn TextView>,
        processor: Box<dyn InputProcessor>,
    ) -> Editor {
        Editor {
            document,
            view,
            processor,
        }
    }

    pub fn loop_input(&mut self) -> anyhow::Result<()> {
        loop {
            let output = self.processor.process_char(self.view.deref())?;
            match output {
                ProcessorOutput::Command(command) => {
                    command.execute(self.document.deref_mut(), self.view.deref_mut())?;
                }
                ProcessorOutput::Nothing => {}
                ProcessorOutput::Exit => return Ok(()),
            }
        }
    }
}
