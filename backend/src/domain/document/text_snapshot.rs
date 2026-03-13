use super::{DocumentError, TextOffset, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSnapshot {
    text: String,
}

impl TextSnapshot {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len_bytes(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn slice(&self, range: TextRange) -> Result<&str, DocumentError> {
        if range.end().value() > self.text.len() {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.text.len(),
                start: range.start(),
                end: range.end(),
            });
        }

        if !self.text.is_char_boundary(range.start().value()) {
            return Err(DocumentError::InvalidUtf8Boundary {
                offset: range.start(),
            });
        }

        if !self.text.is_char_boundary(range.end().value()) {
            return Err(DocumentError::InvalidUtf8Boundary {
                offset: range.end(),
            });
        }

        Ok(&self.text[range.start().value()..range.end().value()])
    }

    pub fn char_column(&self, line_start: TextOffset, offset: TextOffset) -> Result<usize, DocumentError> {
        let range = TextRange::new(line_start, offset)?;
        Ok(self.slice(range)?.chars().count())
    }
}
