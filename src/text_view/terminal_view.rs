use std::cmp::min;
use std::io;
use std::io::{Stdout, Write};
use crossterm::{cursor as term_cursor, execute, QueueableCommand, style, terminal};
use crate::text_document::TextDocument;
use crate::text_view::TextView;
use anyhow::{Error, Result};
use crossterm::terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crate::commands::Direction;
use crate::cursor_position::CursorPosition;

pub struct TerminalView {
    top_left: CursorPosition,
    cursor: CursorPosition,
    output: Stdout,
}

impl TerminalView {
    pub fn new() -> Result<TerminalView> {
        let mut output = io::stdout();
        execute!(output, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        execute!(
            output,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            term_cursor::MoveTo(0, 0)
        )?;
        Ok(TerminalView {
            top_left: CursorPosition::new(0,0),
            cursor: CursorPosition::new(0,0),
            output,
        })
    }
}

impl Drop for TerminalView {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

impl TextView for TerminalView
{
    fn get_document_pos(&self) -> CursorPosition {
        self.cursor
    }

    fn move_cursor(&mut self, direction: Direction, doc: &dyn TextDocument) -> Result<()> {
        match direction {
            Direction::Left(distance) => {
                if distance == 0  {
                    return Err(Error::msg("Distance is zero"))
                }
                if self.cursor.column < distance {
                    if self.cursor.row == 0 {
                        return self.move_cursor_to(CursorPosition::new(0, 0), doc);
                    }
                    return self.move_cursor_to(CursorPosition::new(self.cursor.row - 1, doc.get_line_length(self.cursor.row - 1)?), doc);
                }
                self.move_cursor_to(self.cursor - CursorPosition::new(0, distance), doc)
            }
            Direction::Right(distance) => {
                if distance == 0  {
                    return Err(Error::msg("Distance is zero"))
                }
                if self.cursor.column + distance > doc.get_line_length(self.cursor.row)? {
                    if self.cursor.row == doc.amount_of_lines() - 1 {
                        return self.move_cursor_to(CursorPosition::new(self.cursor.row, doc.get_line_length(self.cursor.row)?), doc);
                    }
                    return self.move_cursor_to(CursorPosition::new(self.cursor.row + 1, 0), doc);
                }
                self.move_cursor_to(self.cursor + CursorPosition::new(0, distance), doc)
            }
            Direction::Up(distance) => {
                if distance == 0  {
                    return Err(Error::msg("Distance is zero"))
                }
                if self.cursor.row < distance {
                    return self.move_cursor_to(CursorPosition::new(0, 0), doc);
                }
                let new_row = self.cursor.row - distance;
                let new_column = min(doc.get_line_length(new_row)?, self.cursor.column);
                self.move_cursor_to(CursorPosition::new(new_row, new_column), doc)
            }
            Direction::Down(distance) => {
                if distance == 0  {
                    return Err(Error::msg("Distance is zero"))
                }
                if self.cursor.row + distance >= doc.amount_of_lines() {
                    return self.move_cursor_to(CursorPosition::new(doc.amount_of_lines() - 1, doc.get_line_length(doc.amount_of_lines() - 1)?), doc);
                }
                let new_row = self.cursor.row + distance;
                let new_column = min(doc.get_line_length(new_row)?, self.cursor.column);
                self.move_cursor_to(CursorPosition::new(new_row, new_column), doc)
            }
            Direction::LineStart => {
                self.move_cursor_to(CursorPosition::new(self.cursor.row, 0), doc)
            }
            Direction::To(row, column) => self.move_cursor_to(CursorPosition::new(row, column), doc),
        }
    }

    fn move_cursor_to(&mut self, pos: CursorPosition, doc: &dyn TextDocument) -> Result<()> {
        let screen_height = terminal::size()?.1 as usize;
        let screen_width = terminal::size()?.0 as usize;

        if pos.row < self.top_left.row
        {
            self.top_left.row = pos.row;
            self.draw(doc)?;
            return Ok(());
        }
        if pos.column < self.top_left.column
        {
            self.top_left.column = pos.column;
            self.draw(doc)?;
            return Ok(());
        }
        if pos.row >= self.top_left.row + screen_height
        {
            self.top_left.row = pos.row - screen_height;
            self.draw(doc)?;
            return Ok(());
        }
        if pos.column >= self.top_left.column + screen_width
        {
            self.top_left.column = pos.column - screen_width;
            self.draw(doc)?;
            return Ok(());
        }

        let new_screen_pos = pos - self.top_left;
        execute!(self.output, term_cursor::MoveTo(new_screen_pos.column as u16, new_screen_pos.row as u16))?;
        self.cursor = pos;
        
        Ok(())
    }

    fn draw(&mut self, document: &dyn TextDocument) -> Result<()>
    {
        let screen_height = terminal::size()?.1 as usize;
        self.output.queue(terminal::Clear(ClearType::All))?;
        self.output.queue(term_cursor::MoveTo(0, 0))?;

        let lines = document.get_lines(self.top_left.row, screen_height + 1)?;
        if lines.is_empty() {
            return Ok(());
        }

        for line in lines[..lines.len() - 1].iter() {
            self.output.queue(style::Print(line))?;
            self.output.queue(style::Print('\n'))?;
        }
        self.output.queue(style::Print(lines.last().unwrap()))?;
        self.output.queue(term_cursor::MoveTo(self.cursor.column as u16, self.cursor.row as u16))?;
        self.output.flush()?;

        Ok(())
    }
}