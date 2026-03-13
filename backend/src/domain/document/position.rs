use super::DocumentError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextOffset(usize);

impl TextOffset {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn value(self) -> usize {
        self.0
    }

    pub const fn checked_add(self, delta: usize) -> Self {
        Self(self.0 + delta)
    }
}

impl From<usize> for TextOffset {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: TextOffset,
    end: TextOffset,
}

impl TextRange {
    pub fn new(start: TextOffset, end: TextOffset) -> Result<Self, DocumentError> {
        if start > end {
            return Err(DocumentError::InvalidRange { start, end });
        }

        Ok(Self { start, end })
    }

    pub const fn empty_at(offset: TextOffset) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub const fn start(self) -> TextOffset {
        self.start
    }

    pub const fn end(self) -> TextOffset {
        self.end
    }

    pub const fn len(self) -> usize {
        self.end.value() - self.start.value()
    }

    pub const fn is_empty(self) -> bool {
        self.start.value() == self.end.value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub const fn line(self) -> usize {
        self.line
    }

    pub const fn column(self) -> usize {
        self.column
    }
}
