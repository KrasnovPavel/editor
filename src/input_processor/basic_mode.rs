use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::input_processor::{InputProcessor, ProcessorOutput};
use anyhow::{Error, Result};
use crate::commands::{AddChar, AddLine, CompositeCommand, MoveCursor, DeleteCharUnderCursor};
use crate::commands::Direction::{Down, Left, LineStart, Right, Up};
use crate::cursor_position::CursorPosition;
use crate::text_view::TextView;

pub struct BasicMode;

impl InputProcessor for BasicMode
{
    fn process_char(&self, view: &dyn TextView) -> Result<ProcessorOutput> {
        let pos = view.get_document_pos();
        match event::read() {
            Ok(Event::Key(KeyEvent {code: KeyCode::Char('q'), kind: KeyEventKind::Press, modifiers: KeyModifiers::CONTROL, .. })) => {
                Ok(ProcessorOutput::Exit)
            }
            Ok(Event::Key(KeyEvent { code: KeyCode::Char(ch), kind: KeyEventKind::Press, .. })) => {
                let command = CompositeCommand::new(vec![
                    Box::new(AddChar::new(pos, ch)),
                    Box::new(MoveCursor::new(Right(1))),
                ]);
                Ok(Box::new(command).into())
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Press, .. })) => {
                let command = CompositeCommand::new(vec![
                    Box::new(AddLine::new(pos)),
                    Box::new(MoveCursor::new(Down(1))),
                    Box::new(MoveCursor::new(LineStart)),
                ]);
                Ok(Box::new(command).into())
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Backspace, kind: KeyEventKind::Press, .. })) => {
                if view.get_document_pos() == CursorPosition::new(0,0) {
                    return Ok(ProcessorOutput::Nothing);
                }
                let command = CompositeCommand::new(vec![
                    Box::new(MoveCursor::new(Left(1))),
                    Box::new(DeleteCharUnderCursor{}),
                ]);
                Ok(Box::new(command).into())
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Delete, kind: KeyEventKind::Press, .. })) => {
                Ok(Box::new(DeleteCharUnderCursor{}).into())
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Left, kind: KeyEventKind::Press, .. })) =>
                Ok(Box::new(MoveCursor::new(Left(1))).into()),
            Ok(Event::Key(KeyEvent { code: KeyCode::Right, kind: KeyEventKind::Press, .. })) =>
                Ok(Box::new(MoveCursor::new(Right(1))).into()),
            Ok(Event::Key(KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. })) =>
                Ok(Box::new(MoveCursor::new(Up(1))).into()),
            Ok(Event::Key(KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. })) =>
                Ok(Box::new(MoveCursor::new(Down(1))).into()),
            Err(e) => Err(Error::from(e)),
            _ => Ok(ProcessorOutput::Nothing),
        }
    }
}