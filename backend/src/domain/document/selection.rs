use super::{DocumentError, TextOffset, TextRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    anchor: TextOffset,
    active: TextOffset,
}

impl Selection {
    pub const fn caret(offset: TextOffset) -> Self {
        Self {
            anchor: offset,
            active: offset,
        }
    }

    pub const fn new(anchor: TextOffset, active: TextOffset) -> Self {
        Self { anchor, active }
    }

    pub const fn anchor(self) -> TextOffset {
        self.anchor
    }

    pub const fn active(self) -> TextOffset {
        self.active
    }

    pub const fn is_caret(self) -> bool {
        self.anchor.value() == self.active.value()
    }

    pub fn range(self) -> Result<TextRange, DocumentError> {
        let start = if self.anchor <= self.active {
            self.anchor
        } else {
            self.active
        };
        let end = if self.anchor <= self.active {
            self.active
        } else {
            self.anchor
        };

        TextRange::new(start, end)
    }

    pub fn collapse_to_start(self) -> Result<Self, DocumentError> {
        Ok(Self::caret(self.range()?.start()))
    }

    pub fn collapse_to_end(self) -> Result<Self, DocumentError> {
        Ok(Self::caret(self.range()?.end()))
    }
}
