use std::path::Path;
use std::time::Instant;

use crate::application::commands::{
    DocumentSnapshot, EditDocument, EditResult, PerformanceTelemetry, SaveDocument,
};
use crate::application::state::{DocumentEntry, DocumentStore};
use crate::domain::document::{Document, DocumentError, DocumentId, RevisionId};
use crate::infrastructure::filesystem::FileSystem;

#[derive(Debug)]
pub enum EditorServiceError {
    Document(DocumentError),
    DocumentNotFound(DocumentId),
    MissingDocumentPath(DocumentId),
    StaleRevision {
        document_id: DocumentId,
        expected: RevisionId,
        actual: RevisionId,
    },
    FileSystem(String),
}

impl std::fmt::Display for EditorServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document(error) => write!(f, "{error}"),
            Self::DocumentNotFound(document_id) => {
                write!(f, "document {} is not open", document_id.value())
            }
            Self::MissingDocumentPath(document_id) => {
                write!(f, "document {} has no file path", document_id.value())
            }
            Self::StaleRevision {
                document_id,
                expected,
                actual,
            } => write!(
                f,
                "stale revision for document {}: expected {}, actual {}",
                document_id.value(),
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
    filesystem: F,
}

impl<F: FileSystem> EditorService<F> {
    pub fn new(filesystem: F) -> Self {
        Self {
            store: DocumentStore::default(),
            filesystem,
        }
    }

    pub fn create_document(&mut self, text: impl Into<String>) -> DocumentSnapshot {
        let id = self.store.next_document_id();
        let document = Document::open(id, text);
        let snapshot = DocumentSnapshot::from_document(&document, None);
        let mut entry = DocumentEntry::new(document, None);
        entry.cache_snapshot(snapshot.clone());

        self.store.insert(entry);
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
        let snapshot = DocumentSnapshot::from_document(&document, Some(path.clone()));
        let mut entry = DocumentEntry::new(document, Some(path));
        entry.cache_snapshot(snapshot.clone());

        self.store.insert(entry);
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
        document_id: DocumentId,
    ) -> Result<DocumentSnapshot, EditorServiceError> {
        let entry = self
            .store
            .get_mut(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?;
        let snapshot_started_at = Instant::now();

        if let Some(snapshot) = entry.cached_snapshot() {
            return Ok(snapshot.clone().with_telemetry(PerformanceTelemetry {
                document_operation_nanos: None,
                snapshot_build_nanos: Some(0),
            }));
        }

        let snapshot = DocumentSnapshot::from_document(entry.document(), entry.path().cloned());
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
        let document = self
            .store
            .get_mut(command.document_id)
            .ok_or(EditorServiceError::DocumentNotFound(command.document_id))?;
        ensure_revision_matches(
            command.document_id,
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
            command.document_id.value(),
            document.document().revision().value()
        );

        Ok(EditResult {
            document_id: command.document_id,
            changes: vec![change],
            telemetry,
        })
    }

    pub fn undo_document(
        &mut self,
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
    ) -> Result<EditResult, EditorServiceError> {
        let document = self
            .store
            .get_mut(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?;
        ensure_revision_matches(
            document_id,
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
            document_id,
            changes,
            telemetry,
        })
    }

    pub fn redo_document(
        &mut self,
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
    ) -> Result<EditResult, EditorServiceError> {
        let document = self
            .store
            .get_mut(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?;
        ensure_revision_matches(
            document_id,
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
            document_id,
            changes,
            telemetry,
        })
    }

    pub fn save_document(
        &mut self,
        command: SaveDocument,
    ) -> Result<DocumentSnapshot, EditorServiceError> {
        let entry = self
            .store
            .get_mut(command.document_id)
            .ok_or(EditorServiceError::DocumentNotFound(command.document_id))?;
        ensure_revision_matches(
            command.document_id,
            command.expected_revision,
            entry.document().revision(),
        )?;
        let path = command
            .path
            .or_else(|| entry.path().cloned())
            .ok_or(EditorServiceError::MissingDocumentPath(command.document_id))?;

        self.filesystem
            .write_string(&path, &entry.document().text())
            .map_err(|error| EditorServiceError::FileSystem(error.to_string()))?;
        entry.set_path(Some(path.clone()));
        let snapshot = DocumentSnapshot::from_document(entry.document(), Some(path));
        entry.cache_snapshot(snapshot.clone());
        log::info!(
            "editor.save document_id={} path={}",
            command.document_id.value(),
            snapshot
                .path
                .as_ref()
                .expect("saved snapshot path")
                .display()
        );

        Ok(snapshot)
    }

    pub fn close_document(&mut self, document_id: DocumentId) -> Result<(), EditorServiceError> {
        self.store
            .remove(document_id)
            .map(|_| {
                log::info!("editor.close document_id={}", document_id.value());
            })
            .ok_or(EditorServiceError::DocumentNotFound(document_id))
    }
}

fn ensure_revision_matches(
    document_id: DocumentId,
    expected_revision: Option<RevisionId>,
    actual_revision: RevisionId,
) -> Result<(), EditorServiceError> {
    if let Some(expected_revision) = expected_revision {
        if expected_revision != actual_revision {
            return Err(EditorServiceError::StaleRevision {
                document_id,
                expected: expected_revision,
                actual: actual_revision,
            });
        }
    }

    Ok(())
}
