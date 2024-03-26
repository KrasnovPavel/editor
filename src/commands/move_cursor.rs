use crate::commands::Command;
use crate::text_document::TextDocument;
use crate::text_view::TextView;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    LineStart,
    To(usize, usize)
}

pub struct MoveCursor {
    direction: Direction,
}

impl MoveCursor {
    pub fn new(direction: Direction) -> MoveCursor {
        MoveCursor {
            direction
        }
    }
}

impl Command for MoveCursor {
    fn execute(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        view.move_cursor(self.direction, doc)
    }

    fn undo(&self, doc: &mut dyn TextDocument, view: &mut dyn TextView) -> anyhow::Result<()> {
        todo!()
    }
}