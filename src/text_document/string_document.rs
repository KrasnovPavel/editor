use std::cmp::min;
use crate::text_document::TextDocument;
use anyhow::{Error, Result};
use crate::cursor_position::CursorPosition;

pub struct StringDocument
{
    rows: Vec<String>,
}

impl StringDocument {
    pub fn new() -> Self {
        StringDocument {
            rows: vec![],
        }
    }
}

impl TextDocument for StringDocument
{
    fn amount_of_lines(&self) -> usize {
        self.rows.len()
    }

    fn get_line(&self, row: usize) -> Result<String> {
        self.rows
            .get(row)
            .ok_or(Error::msg("Line not found"))
            .cloned()
    }

    fn get_line_length(&self, row: usize) -> Result<usize> {
        self.rows
            .get(row)
            .ok_or(Error::msg("Line not found"))
            .map(|l| l.len())
    }

    fn get_lines(&self, first_row: usize, mut last_row: usize) -> Result<Vec<String>> {
        if first_row > self.rows.len() {
            return Ok(vec![]);
        }
        last_row = min(last_row, self.rows.len());
        if first_row > last_row {
            return Ok(vec![]);
        }
        Ok(self.rows[first_row..last_row].to_vec())
    }

    fn add_char(&mut self, pos: CursorPosition, ch: char) -> Result<()> {
        if pos.row < self.rows.len() {
            let line = self.rows.get_mut(pos.row).unwrap();
            line.insert(pos.column, ch);
            return Ok(());
        }
        if pos.row == self.rows.len() {
            self.rows.push(ch.to_string());
            return Ok(());
        }
        Err(Error::msg("Can't add char"))
    }

    fn add_line(&mut self, pos: CursorPosition) -> Result<()> {
        let begin = self.rows.get(pos.row).ok_or(Error::msg(""))?[..pos.column].to_string();
        let end = self.rows.get(pos.row).ok_or(Error::msg(""))?[pos.column..].to_string();
        *self.rows.get_mut(pos.row).ok_or(Error::msg(""))? = begin;

        if pos.row < self.rows.len() - 1 {
            self.rows.insert(pos.row + 1, end);
            return Ok(());
        }
        if pos.row == self.rows.len() - 1 {
            self.rows.push(end);
            return Ok(());
        }

        Err(Error::msg("Can't add line"))
    }

    fn delete_char(&mut self, pos: CursorPosition) -> Result<()> {
        let current_row_len = self.rows.get(pos.row).map(|s| s.len()).unwrap_or(0);
        if pos.column == current_row_len {
            if pos.row < self.rows.len() - 1
            {
                let next_line = self.rows.remove(pos.row + 1);
                self.rows.get_mut(pos.row).unwrap().push_str(next_line.as_str());
                return Ok(());
            }
            return Ok(());
        }

        self.rows.get_mut(pos.row).unwrap().remove(pos.column);
        Ok(())
    }
}