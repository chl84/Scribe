mod change_set;
mod document;
mod edit;
mod error;
mod ids;
mod newline;
mod position;

pub use change_set::ChangeSet;
pub use document::Document;
pub use edit::Edit;
pub use error::DocumentError;
pub use ids::{DocumentId, RevisionId};
pub use newline::{NewlineMode, NewlinePolicy};
pub use position::{Position, TextOffset, TextRange};

#[cfg(test)]
mod tests;
