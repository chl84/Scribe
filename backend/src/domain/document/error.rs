use std::error::Error;
use std::fmt::{Display, Formatter};

use super::TextOffset;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentError {
    InvalidRange {
        start: TextOffset,
        end: TextOffset,
    },
    RangeOutOfBounds {
        len: usize,
        start: TextOffset,
        end: TextOffset,
    },
    PositionOutOfBounds {
        line: usize,
        column: usize,
    },
    InvalidUtf8Boundary {
        offset: TextOffset,
    },
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange { start, end } => {
                write!(
                    f,
                    "invalid range: start offset {} is greater than end offset {}",
                    start.value(),
                    end.value()
                )
            }
            Self::RangeOutOfBounds { len, start, end } => {
                write!(
                    f,
                    "range {}..{} is out of bounds for document length {}",
                    start.value(),
                    end.value(),
                    len
                )
            }
            Self::PositionOutOfBounds { line, column } => {
                write!(f, "position {}:{} is out of bounds", line, column)
            }
            Self::InvalidUtf8Boundary { offset } => {
                write!(
                    f,
                    "offset {} is not on a UTF-8 character boundary",
                    offset.value()
                )
            }
        }
    }
}

impl Error for DocumentError {}
