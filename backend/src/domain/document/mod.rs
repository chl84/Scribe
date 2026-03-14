mod change_set;
mod cursor;
mod document;
mod edit;
mod error;
mod history;
mod ids;
mod line_index;
mod newline;
mod piece_table;
mod position;
mod selection;
mod text_buffer;
mod text_snapshot;

pub use change_set::ChangeSet;
pub use cursor::{CursorMove, CursorMoveRules};
pub use document::Document;
pub use edit::Edit;
pub use error::DocumentError;
pub use history::{EditTransaction, UndoManager};
pub use ids::{DocumentId, RevisionId};
pub use line_index::LineIndex;
pub use newline::{NewlineMode, NewlinePolicy};
pub use piece_table::PieceTable;
pub use position::{Position, TextOffset, TextRange};
pub use selection::Selection;
pub use text_buffer::TextBuffer;
pub use text_snapshot::TextSnapshot;

#[cfg(test)]
mod tests;
