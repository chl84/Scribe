use std::path::Path;
use std::time::Instant;

use crate::application::commands::{
    CreateViewport, DocumentSnapshot, EditDocument, EditResult, PerformanceTelemetry, SaveDocument,
    ScrollViewport, ViewportLine, ViewportSnapshot,
};
use crate::application::state::{DocumentEntry, DocumentStore, ViewportStore};
use crate::domain::document::{
    Document, DocumentError, DocumentSessionId, RevisionId, TextOffset, TextRange,
    ViewportSessionId,
};
use crate::infrastructure::filesystem::FileSystem;

#[derive(Debug)]
pub enum EditorServiceError {
    Document(DocumentError),
    DocumentSessionNotFound(DocumentSessionId),
    ViewportSessionNotFound(ViewportSessionId),
    MissingDocumentPath(DocumentSessionId),
    StaleRevision {
        document_session_id: DocumentSessionId,
        expected: RevisionId,
        actual: RevisionId,
    },
    FileSystem(String),
}

impl std::fmt::Display for EditorServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document(error) => write!(f, "{error}"),
            Self::DocumentSessionNotFound(document_session_id) => {
                write!(
                    f,
                    "document session {} is not open",
                    document_session_id.value()
                )
            }
            Self::ViewportSessionNotFound(viewport_session_id) => {
                write!(
                    f,
                    "viewport session {} is not open",
                    viewport_session_id.value()
                )
            }
            Self::MissingDocumentPath(document_session_id) => {
                write!(
                    f,
                    "document session {} has no file path",
                    document_session_id.value()
                )
            }
            Self::StaleRevision {
                document_session_id,
                expected,
                actual,
            } => write!(
                f,
                "stale revision for document session {}: expected {}, actual {}",
                document_session_id.value(),
                expected.value(),
                actual.value()
            ),
            Self::FileSystem(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for EditorServiceError {}

impl From<DocumentError> for EditorServiceError {
    fn from(value: DocumentError) -> Self {
        Self::Document(value)
    }
}

pub struct EditorService<F: FileSystem> {
    store: DocumentStore,
    viewport_store: ViewportStore,
    filesystem: F,
}

impl<F: FileSystem> EditorService<F> {
    pub fn new(filesystem: F) -> Self {
        Self {
            store: DocumentStore::default(),
            viewport_store: ViewportStore::default(),
            filesystem,
        }
    }

    pub fn create_document(&mut self, text: impl Into<String>) -> DocumentSnapshot {
        let id = self.store.next_document_id();
        let document = Document::open(id, text);
        self.store.insert(DocumentEntry::new(document, None));
        let session_id = self
            .store
            .open_session(id)
            .expect("document session should exist after insert");
        let entry = self
            .store
            .get_mut(id)
            .expect("inserted document should exist");
        let snapshot = DocumentSnapshot::from_document(entry.document(), session_id, None);
        entry.cache_snapshot(snapshot.clone());

        log::info!("editor.create document_id={}", snapshot.document_id.value());

        snapshot
    }

    pub fn open_document(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<DocumentSnapshot, EditorServiceError> {
        let path = path.as_ref().to_path_buf();
        let contents = self
            .filesystem
            .read_to_string(&path)
            .map_err(|error| EditorServiceError::FileSystem(error.to_string()))?;
        let id = self.store.next_document_id();
        let document = Document::open(id, contents);
        self.store
            .insert(DocumentEntry::new(document, Some(path.clone())));
        let session_id = self
            .store
            .open_session(id)
            .expect("document session should exist after insert");
        let entry = self
            .store
            .get_mut(id)
            .expect("inserted document should exist");
        let snapshot = DocumentSnapshot::from_document(entry.document(), session_id, Some(path));
        entry.cache_snapshot(snapshot.clone());

        log::info!(
            "editor.open document_id={} path={}",
            snapshot.document_id.value(),
            snapshot
                .path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_default()
        );

        Ok(snapshot)
    }

    pub fn get_document(
        &mut self,
        document_session_id: DocumentSessionId,
    ) -> Result<DocumentSnapshot, EditorServiceError> {
        let document_id = self.store.resolve_session(document_session_id).ok_or(
            EditorServiceError::DocumentSessionNotFound(document_session_id),
        )?;
        let entry =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    document_session_id,
                ))?;
        let snapshot_started_at = Instant::now();

        if let Some(snapshot) = entry.cached_snapshot() {
            return Ok(snapshot
                .clone()
                .with_document_session_id(document_session_id)
                .with_telemetry(PerformanceTelemetry {
                    document_operation_nanos: None,
                    snapshot_build_nanos: Some(0),
                }));
        }

        let snapshot = DocumentSnapshot::from_document(
            entry.document(),
            document_session_id,
            entry.path().cloned(),
        );
        let build_nanos = snapshot_started_at.elapsed().as_nanos() as u64;
        entry.cache_snapshot(snapshot.clone());

        Ok(snapshot.with_telemetry(PerformanceTelemetry {
            document_operation_nanos: None,
            snapshot_build_nanos: Some(build_nanos),
        }))
    }

    pub fn edit_document(
        &mut self,
        command: EditDocument,
    ) -> Result<EditResult, EditorServiceError> {
        let document_id = self
            .store
            .resolve_session(command.document_session_id)
            .ok_or(EditorServiceError::DocumentSessionNotFound(
                command.document_session_id,
            ))?;
        let document =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    command.document_session_id,
                ))?;
        ensure_revision_matches(
            command.document_session_id,
            command.expected_revision,
            document.document().revision(),
        )?;
        let operation_started_at = Instant::now();

        let change = document.document_mut().apply_edit(command.edit)?;
        document.clear_snapshot_cache();
        let telemetry = PerformanceTelemetry {
            document_operation_nanos: Some(operation_started_at.elapsed().as_nanos() as u64),
            snapshot_build_nanos: None,
        };
        log::info!(
            "editor.edit document_id={} revision={} changes=1",
            document_id.value(),
            document.document().revision().value()
        );

        Ok(EditResult {
            document_session_id: command.document_session_id,
            document_id,
            changes: vec![change],
            telemetry,
        })
    }

    pub fn undo_document(
        &mut self,
        document_session_id: DocumentSessionId,
        expected_revision: Option<RevisionId>,
    ) -> Result<EditResult, EditorServiceError> {
        let document_id = self.store.resolve_session(document_session_id).ok_or(
            EditorServiceError::DocumentSessionNotFound(document_session_id),
        )?;
        let document =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    document_session_id,
                ))?;
        ensure_revision_matches(
            document_session_id,
            expected_revision,
            document.document().revision(),
        )?;
        let operation_started_at = Instant::now();
        let changes = document.document_mut().undo()?.unwrap_or_default();
        document.clear_snapshot_cache();
        let telemetry = PerformanceTelemetry {
            document_operation_nanos: Some(operation_started_at.elapsed().as_nanos() as u64),
            snapshot_build_nanos: None,
        };
        log::info!(
            "editor.undo document_id={} changes={}",
            document_id.value(),
            changes.len()
        );

        Ok(EditResult {
            document_session_id,
            document_id,
            changes,
            telemetry,
        })
    }

    pub fn redo_document(
        &mut self,
        document_session_id: DocumentSessionId,
        expected_revision: Option<RevisionId>,
    ) -> Result<EditResult, EditorServiceError> {
        let document_id = self.store.resolve_session(document_session_id).ok_or(
            EditorServiceError::DocumentSessionNotFound(document_session_id),
        )?;
        let document =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    document_session_id,
                ))?;
        ensure_revision_matches(
            document_session_id,
            expected_revision,
            document.document().revision(),
        )?;
        let operation_started_at = Instant::now();
        let changes = document.document_mut().redo()?.unwrap_or_default();
        document.clear_snapshot_cache();
        let telemetry = PerformanceTelemetry {
            document_operation_nanos: Some(operation_started_at.elapsed().as_nanos() as u64),
            snapshot_build_nanos: None,
        };
        log::info!(
            "editor.redo document_id={} changes={}",
            document_id.value(),
            changes.len()
        );

        Ok(EditResult {
            document_session_id,
            document_id,
            changes,
            telemetry,
        })
    }

    pub fn save_document(
        &mut self,
        command: SaveDocument,
    ) -> Result<DocumentSnapshot, EditorServiceError> {
        let document_id = self
            .store
            .resolve_session(command.document_session_id)
            .ok_or(EditorServiceError::DocumentSessionNotFound(
                command.document_session_id,
            ))?;
        let entry =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    command.document_session_id,
                ))?;
        ensure_revision_matches(
            command.document_session_id,
            command.expected_revision,
            entry.document().revision(),
        )?;
        let path = command.path.or_else(|| entry.path().cloned()).ok_or(
            EditorServiceError::MissingDocumentPath(command.document_session_id),
        )?;

        self.filesystem
            .write_string(&path, &entry.document().text())
            .map_err(|error| EditorServiceError::FileSystem(error.to_string()))?;
        entry.set_path(Some(path.clone()));
        let snapshot = DocumentSnapshot::from_document(
            entry.document(),
            command.document_session_id,
            Some(path),
        );
        entry.cache_snapshot(snapshot.clone());
        log::info!(
            "editor.save document_id={} path={}",
            document_id.value(),
            snapshot
                .path
                .as_ref()
                .expect("saved snapshot path")
                .display()
        );

        Ok(snapshot)
    }

    pub fn close_document(
        &mut self,
        document_session_id: DocumentSessionId,
    ) -> Result<(), EditorServiceError> {
        let (document_id, should_remove_document) =
            self.store.close_session(document_session_id).ok_or(
                EditorServiceError::DocumentSessionNotFound(document_session_id),
            )?;

        if should_remove_document {
            self.store.remove(document_id);
        }

        log::info!(
            "editor.close document_session_id={} document_id={}",
            document_session_id.value(),
            document_id.value()
        );

        Ok(())
    }

    pub fn create_viewport(
        &mut self,
        command: CreateViewport,
    ) -> Result<ViewportSnapshot, EditorServiceError> {
        self.store
            .resolve_session(command.document_session_id)
            .ok_or(EditorServiceError::DocumentSessionNotFound(
                command.document_session_id,
            ))?;

        let viewport_session_id = self.viewport_store.create_viewport(
            command.document_session_id,
            command.top_line,
            command.visible_line_count,
        );

        self.get_viewport(viewport_session_id)
    }

    pub fn get_viewport(
        &mut self,
        viewport_session_id: ViewportSessionId,
    ) -> Result<ViewportSnapshot, EditorServiceError> {
        let viewport = self
            .viewport_store
            .get(viewport_session_id)
            .ok_or(EditorServiceError::ViewportSessionNotFound(
                viewport_session_id,
            ))?
            .clone();
        let started_at = Instant::now();
        let document_id = self
            .store
            .resolve_session(viewport.document_session_id())
            .ok_or(EditorServiceError::DocumentSessionNotFound(
                viewport.document_session_id(),
            ))?;
        let entry =
            self.store
                .get_mut(document_id)
                .ok_or(EditorServiceError::DocumentSessionNotFound(
                    viewport.document_session_id(),
                ))?;
        let snapshot = build_viewport_snapshot(
            entry.document(),
            viewport_session_id,
            viewport.document_session_id(),
            viewport.top_line(),
            viewport.visible_line_count(),
        )?;

        Ok(snapshot.with_telemetry(PerformanceTelemetry {
            document_operation_nanos: None,
            snapshot_build_nanos: Some(started_at.elapsed().as_nanos() as u64),
        }))
    }

    pub fn scroll_viewport(
        &mut self,
        command: ScrollViewport,
    ) -> Result<ViewportSnapshot, EditorServiceError> {
        let viewport = self
            .viewport_store
            .get_mut(command.viewport_session_id)
            .ok_or(EditorServiceError::ViewportSessionNotFound(
                command.viewport_session_id,
            ))?;
        viewport.set_top_line(command.top_line);

        self.get_viewport(command.viewport_session_id)
    }
}

fn ensure_revision_matches(
    document_session_id: DocumentSessionId,
    expected_revision: Option<RevisionId>,
    actual_revision: RevisionId,
) -> Result<(), EditorServiceError> {
    if let Some(expected_revision) = expected_revision {
        if expected_revision != actual_revision {
            return Err(EditorServiceError::StaleRevision {
                document_session_id,
                expected: expected_revision,
                actual: actual_revision,
            });
        }
    }

    Ok(())
}

fn build_viewport_snapshot(
    document: &Document,
    viewport_session_id: ViewportSessionId,
    document_session_id: DocumentSessionId,
    top_line: usize,
    visible_line_count: usize,
) -> Result<ViewportSnapshot, EditorServiceError> {
    let document_line_count = document.line_count();
    let clamped_top_line = if document_line_count == 0 {
        0
    } else {
        top_line.min(document_line_count.saturating_sub(1))
    };
    let last_line = (clamped_top_line + visible_line_count).min(document_line_count);
    let document_snapshot = document.snapshot();
    let mut lines = Vec::with_capacity(last_line.saturating_sub(clamped_top_line));

    for line_number in clamped_top_line..last_line {
        let line_start = document
            .line_start_offset(line_number)
            .map_err(EditorServiceError::Document)?;
        let line_end = if line_number + 1 < document_line_count {
            document
                .line_start_offset(line_number + 1)
                .map_err(EditorServiceError::Document)?
        } else {
            TextOffset::new(document.len_bytes())
        };
        let line_text = document_snapshot
            .slice(TextRange::new(line_start, line_end).map_err(EditorServiceError::Document)?)
            .map_err(EditorServiceError::Document)?
            .to_string();

        lines.push(ViewportLine {
            line_number,
            text: line_text,
        });
    }

    Ok(ViewportSnapshot {
        viewport_session_id,
        document_session_id,
        document_id: document.id(),
        revision: document.revision(),
        top_line: clamped_top_line,
        visible_line_count,
        document_line_count,
        lines,
        telemetry: None,
    })
}
