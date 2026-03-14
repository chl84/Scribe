use std::path::Path;

use crate::application::commands::{DocumentSnapshot, EditDocument, EditResult, SaveDocument};
use crate::application::state::{DocumentEntry, DocumentStore};
use crate::domain::document::{Document, DocumentError, DocumentId};
use crate::infrastructure::filesystem::FileSystem;

#[derive(Debug)]
pub enum EditorServiceError {
    Document(DocumentError),
    DocumentNotFound(DocumentId),
    MissingDocumentPath(DocumentId),
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

        self.store.insert(DocumentEntry::new(document, None));
        log::info!("editor.create document_id={}", snapshot.document_id.value());

        snapshot
    }

    pub fn open_document(&mut self, path: impl AsRef<Path>) -> Result<DocumentSnapshot, EditorServiceError> {
        let path = path.as_ref().to_path_buf();
        let contents = self
            .filesystem
            .read_to_string(&path)
            .map_err(|error| EditorServiceError::FileSystem(error.to_string()))?;
        let id = self.store.next_document_id();
        let document = Document::open(id, contents);
        let snapshot = DocumentSnapshot::from_document(&document, Some(path.clone()));

        self.store.insert(DocumentEntry::new(document, Some(path)));
        log::info!(
            "editor.open document_id={} path={}",
            snapshot.document_id.value(),
            snapshot.path.as_ref().map(|path| path.display().to_string()).unwrap_or_default()
        );

        Ok(snapshot)
    }

    pub fn get_document(&self, document_id: DocumentId) -> Result<DocumentSnapshot, EditorServiceError> {
        let entry = self
            .store
            .get(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?;

        Ok(DocumentSnapshot::from_document(entry.document(), entry.path().cloned()))
    }

    pub fn edit_document(&mut self, command: EditDocument) -> Result<EditResult, EditorServiceError> {
        let document = self
            .store
            .get_mut(command.document_id)
            .ok_or(EditorServiceError::DocumentNotFound(command.document_id))?
            .document_mut();

        let change = document.apply_edit(command.edit)?;
        log::info!(
            "editor.edit document_id={} revision={} changes=1",
            command.document_id.value(),
            document.revision().value()
        );

        Ok(EditResult {
            document_id: command.document_id,
            changes: vec![change],
        })
    }

    pub fn undo_document(&mut self, document_id: DocumentId) -> Result<EditResult, EditorServiceError> {
        let document = self
            .store
            .get_mut(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?
            .document_mut();
        let changes = document.undo()?.unwrap_or_default();
        log::info!(
            "editor.undo document_id={} changes={}",
            document_id.value(),
            changes.len()
        );

        Ok(EditResult {
            document_id,
            changes,
        })
    }

    pub fn redo_document(&mut self, document_id: DocumentId) -> Result<EditResult, EditorServiceError> {
        let document = self
            .store
            .get_mut(document_id)
            .ok_or(EditorServiceError::DocumentNotFound(document_id))?
            .document_mut();
        let changes = document.redo()?.unwrap_or_default();
        log::info!(
            "editor.redo document_id={} changes={}",
            document_id.value(),
            changes.len()
        );

        Ok(EditResult {
            document_id,
            changes,
        })
    }

    pub fn save_document(&mut self, command: SaveDocument) -> Result<DocumentSnapshot, EditorServiceError> {
        let entry = self
            .store
            .get_mut(command.document_id)
            .ok_or(EditorServiceError::DocumentNotFound(command.document_id))?;
        let path = command
            .path
            .or_else(|| entry.path().cloned())
            .ok_or(EditorServiceError::MissingDocumentPath(command.document_id))?;

        self.filesystem
            .write_string(&path, &entry.document().text())
            .map_err(|error| EditorServiceError::FileSystem(error.to_string()))?;
        entry.set_path(Some(path.clone()));
        log::info!(
            "editor.save document_id={} path={}",
            command.document_id.value(),
            path.display()
        );

        Ok(DocumentSnapshot::from_document(entry.document(), Some(path)))
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
