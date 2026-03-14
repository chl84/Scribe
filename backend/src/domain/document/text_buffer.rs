use super::{DocumentError, TextOffset, TextRange, TextSnapshot};

pub trait TextBuffer: Clone {
    fn len_bytes(&self) -> usize;
    fn snapshot(&self) -> TextSnapshot;
    fn slice_string(&self, range: TextRange) -> Result<String, DocumentError>;
    fn is_char_boundary(&self, offset: TextOffset) -> bool;
    fn insert(&mut self, offset: TextOffset, text: &str) -> Result<(), DocumentError>;
    fn delete(&mut self, range: TextRange) -> Result<(), DocumentError>;
    fn replace(&mut self, range: TextRange, text: &str) -> Result<(), DocumentError>;
}
