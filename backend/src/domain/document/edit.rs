use super::{TextOffset, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edit {
    Insert { offset: TextOffset, text: String },
    Delete { range: TextRange },
    Replace { range: TextRange, text: String },
}
