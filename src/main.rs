mod input_processor;
mod text_document;
mod text_view;
mod commands;
mod cursor_position;
mod editor;

use crate::editor::Editor;
use crate::input_processor::{BasicMode};
use crate::text_document::StringDocument;
use crate::text_view::TerminalView;

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::new(
        Box::new(StringDocument::new()),
        Box::new(TerminalView::new()?),
        Box::new(BasicMode{}),
    );
    
    editor.loop_input()
}
