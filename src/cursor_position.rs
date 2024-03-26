use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CursorPosition {
    pub row: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(row: usize, column: usize) -> CursorPosition {
        CursorPosition {
            row,
            column,
        }
    }
}

impl ops::Add<CursorPosition> for CursorPosition {
    type Output = CursorPosition;

    fn add(self, rhs: CursorPosition) -> CursorPosition {
        CursorPosition {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl ops::Sub<CursorPosition> for CursorPosition {
    type Output = CursorPosition;

    fn sub(self, rhs: CursorPosition) -> CursorPosition {
        CursorPosition {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

impl From<(u16, u16)> for CursorPosition {
    fn from((column, row): (u16, u16)) -> Self {
        CursorPosition {
            row: row as usize,
            column: column as usize,
        }
    }
}