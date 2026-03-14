use std::path::PathBuf;

use crate::domain::document::{
    ChangeSet, Document, DocumentId, DocumentSessionId, Edit, NewlineMode, RevisionId,
    ViewportSessionId,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PerformanceTelemetry {
    pub document_operation_nanos: Option<u64>,
    pub snapshot_build_nanos: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSnapshot {
    pub document_session_id: DocumentSessionId,
    pub document_id: DocumentId,
    pub revision: RevisionId,
    pub text: String,
    pub line_count: usize,
    pub newline_mode: NewlineMode,
    pub path: Option<PathBuf>,
    pub telemetry: Option<PerformanceTelemetry>,
}

impl DocumentSnapshot {
    pub fn from_document(
        document: &Document,
        document_session_id: DocumentSessionId,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            document_session_id,
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

    pub fn with_document_session_id(mut self, document_session_id: DocumentSessionId) -> Self {
        self.document_session_id = document_session_id;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditResult {
    pub document_session_id: DocumentSessionId,
    pub document_id: DocumentId,
    pub changes: Vec<ChangeSet>,
    pub telemetry: PerformanceTelemetry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveDocument {
    pub document_session_id: DocumentSessionId,
    pub expected_revision: Option<RevisionId>,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditDocument {
    pub document_session_id: DocumentSessionId,
    pub expected_revision: Option<RevisionId>,
    pub edit: Edit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateViewport {
    pub document_session_id: DocumentSessionId,
    pub top_line: usize,
    pub visible_line_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollViewport {
    pub viewport_session_id: ViewportSessionId,
    pub top_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportLine {
    pub line_number: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportSnapshot {
    pub viewport_session_id: ViewportSessionId,
    pub document_session_id: DocumentSessionId,
    pub document_id: DocumentId,
    pub revision: RevisionId,
    pub top_line: usize,
    pub visible_line_count: usize,
    pub document_line_count: usize,
    pub lines: Vec<ViewportLine>,
    pub telemetry: Option<PerformanceTelemetry>,
}

impl ViewportSnapshot {
    pub fn with_telemetry(mut self, telemetry: PerformanceTelemetry) -> Self {
        self.telemetry = Some(telemetry);
        self
    }
}
