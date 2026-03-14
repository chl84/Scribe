use std::path::PathBuf;

use crate::domain::document::{ChangeSet, Document, DocumentId, Edit, NewlineMode, RevisionId};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PerformanceTelemetry {
    pub document_operation_nanos: Option<u64>,
    pub snapshot_build_nanos: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSnapshot {
    pub document_id: DocumentId,
    pub revision: RevisionId,
    pub text: String,
    pub line_count: usize,
    pub newline_mode: NewlineMode,
    pub path: Option<PathBuf>,
    pub telemetry: Option<PerformanceTelemetry>,
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
            telemetry: None,
        }
    }

    pub fn with_telemetry(mut self, telemetry: PerformanceTelemetry) -> Self {
        self.telemetry = Some(telemetry);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditResult {
    pub document_id: DocumentId,
    pub changes: Vec<ChangeSet>,
    pub telemetry: PerformanceTelemetry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveDocument {
    pub document_id: DocumentId,
    pub expected_revision: Option<RevisionId>,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditDocument {
    pub document_id: DocumentId,
    pub expected_revision: Option<RevisionId>,
    pub edit: Edit,
}
