use serde::{Deserialize, Serialize};

use crate::application::commands::{
    DocumentSnapshot, EditResult, PerformanceTelemetry, ViewportLine, ViewportSnapshot,
};
use crate::domain::document::{ChangeSet, Edit, NewlineMode, TextOffset, TextRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenDocumentRequest {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDocumentRequest {
    pub document_session_id: u64,
    pub expected_revision: Option<u64>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    pub document_session_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionedDocumentReference {
    pub document_session_id: u64,
    pub expected_revision: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateViewportRequest {
    pub document_session_id: u64,
    pub top_line: usize,
    pub visible_line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportReference {
    pub viewport_session_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollViewportRequest {
    pub viewport_session_id: u64,
    pub top_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EditCommandDto {
    Insert {
        offset: usize,
        text: String,
    },
    Delete {
        start: usize,
        end: usize,
    },
    Replace {
        start: usize,
        end: usize,
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditDocumentRequest {
    pub document_session_id: u64,
    pub expected_revision: Option<u64>,
    pub edit: EditCommandDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentSnapshotDto {
    pub document_session_id: u64,
    pub document_id: u64,
    pub revision: u64,
    pub text: String,
    pub line_count: usize,
    pub newline_mode: &'static str,
    pub path: Option<String>,
    pub telemetry: Option<PerformanceTelemetryDto>,
}

impl From<DocumentSnapshot> for DocumentSnapshotDto {
    fn from(value: DocumentSnapshot) -> Self {
        Self {
            document_session_id: value.document_session_id.value(),
            document_id: value.document_id.value(),
            revision: value.revision.value(),
            text: value.text,
            line_count: value.line_count,
            newline_mode: match value.newline_mode {
                NewlineMode::Lf => "lf",
                NewlineMode::Crlf => "crlf",
            },
            path: value.path.map(|path| path.display().to_string()),
            telemetry: value.telemetry.map(PerformanceTelemetryDto::from),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeSetDto {
    pub revision_before: u64,
    pub revision_after: u64,
    pub range_before: RangeDto,
    pub range_after: RangeDto,
    pub inserted_text: String,
    pub removed_text: String,
}

impl From<ChangeSet> for ChangeSetDto {
    fn from(value: ChangeSet) -> Self {
        Self {
            revision_before: value.revision_before().value(),
            revision_after: value.revision_after().value(),
            range_before: RangeDto::from(value.range_before()),
            range_after: RangeDto::from(value.range_after()),
            inserted_text: value.inserted_text().to_string(),
            removed_text: value.removed_text().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditResultDto {
    pub document_session_id: u64,
    pub document_id: u64,
    pub changes: Vec<ChangeSetDto>,
    pub telemetry: PerformanceTelemetryDto,
}

impl From<EditResult> for EditResultDto {
    fn from(value: EditResult) -> Self {
        Self {
            document_session_id: value.document_session_id.value(),
            document_id: value.document_id.value(),
            changes: value.changes.into_iter().map(ChangeSetDto::from).collect(),
            telemetry: PerformanceTelemetryDto::from(value.telemetry),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceTelemetryDto {
    pub document_operation_nanos: Option<u64>,
    pub snapshot_build_nanos: Option<u64>,
}

impl From<PerformanceTelemetry> for PerformanceTelemetryDto {
    fn from(value: PerformanceTelemetry) -> Self {
        Self {
            document_operation_nanos: value.document_operation_nanos,
            snapshot_build_nanos: value.snapshot_build_nanos,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewportLineDto {
    pub line_number: usize,
    pub text: String,
}

impl From<ViewportLine> for ViewportLineDto {
    fn from(value: ViewportLine) -> Self {
        Self {
            line_number: value.line_number,
            text: value.text,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewportSnapshotDto {
    pub viewport_session_id: u64,
    pub document_session_id: u64,
    pub document_id: u64,
    pub revision: u64,
    pub top_line: usize,
    pub visible_line_count: usize,
    pub document_line_count: usize,
    pub lines: Vec<ViewportLineDto>,
    pub telemetry: Option<PerformanceTelemetryDto>,
}

impl From<ViewportSnapshot> for ViewportSnapshotDto {
    fn from(value: ViewportSnapshot) -> Self {
        Self {
            viewport_session_id: value.viewport_session_id.value(),
            document_session_id: value.document_session_id.value(),
            document_id: value.document_id.value(),
            revision: value.revision.value(),
            top_line: value.top_line,
            visible_line_count: value.visible_line_count,
            document_line_count: value.document_line_count,
            lines: value.lines.into_iter().map(ViewportLineDto::from).collect(),
            telemetry: value.telemetry.map(PerformanceTelemetryDto::from),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RangeDto {
    pub start: usize,
    pub end: usize,
}

impl From<TextRange> for RangeDto {
    fn from(value: TextRange) -> Self {
        Self {
            start: value.start().value(),
            end: value.end().value(),
        }
    }
}

impl TryFrom<EditCommandDto> for Edit {
    type Error = String;

    fn try_from(value: EditCommandDto) -> Result<Self, Self::Error> {
        match value {
            EditCommandDto::Insert { offset, text } => Ok(Edit::Insert {
                offset: TextOffset::new(offset),
                text,
            }),
            EditCommandDto::Delete { start, end } => Ok(Edit::Delete {
                range: TextRange::new(TextOffset::new(start), TextOffset::new(end))
                    .map_err(|error| error.to_string())?,
            }),
            EditCommandDto::Replace { start, end, text } => Ok(Edit::Replace {
                range: TextRange::new(TextOffset::new(start), TextOffset::new(end))
                    .map_err(|error| error.to_string())?,
                text,
            }),
        }
    }
}
