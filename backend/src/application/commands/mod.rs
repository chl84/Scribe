use std::path::PathBuf;

use crate::domain::document::{ChangeSet, Document, DocumentId, Edit, NewlineMode, RevisionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSnapshot {
    pub document_id: DocumentId,
    pub revision: RevisionId,
    pub text: String,
    pub line_count: usize,
    pub newline_mode: NewlineMode,
    pub path: Option<PathBuf>,
}

impl DocumentSnapshot {
    pub fn from_document(document: &Document, path: Option<PathBuf>) -> Self {
        Self {
            document_id: document.id(),
            revision: document.revision(),
            text: document.text(),
            line_count: document.line_count(),
            newline_mode: document.newline_policy().preferred_mode(),
            path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditResult {
    pub document_id: DocumentId,
    pub changes: Vec<ChangeSet>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveDocument {
    pub document_id: DocumentId,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditDocument {
    pub document_id: DocumentId,
    pub edit: Edit,
}
