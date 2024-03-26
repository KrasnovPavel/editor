pub mod basic_mode;

pub use basic_mode::BasicMode;

use anyhow::Result;
use crate::commands::Command;
use crate::text_view::TextView;

pub enum ProcessorOutput
{
    Command(Box<dyn Command>),
    Nothing,
    Exit,
}

impl<T> From<Box<T>> for ProcessorOutput where T : Command + 'static {
    fn from(value: Box<T>) -> Self {
        ProcessorOutput::Command(value)
    }
}

pub trait InputProcessor
{
    fn process_char(&self, view: &dyn TextView) -> Result<ProcessorOutput>;
}
