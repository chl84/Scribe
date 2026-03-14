use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::application::commands::DocumentSnapshot;
use crate::domain::document::{Document, DocumentId, DocumentSessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentEntry {
    document: Document,
    path: Option<PathBuf>,
    cached_snapshot: Option<DocumentSnapshot>,
}

impl DocumentEntry {
    pub fn new(document: Document, path: Option<PathBuf>) -> Self {
        Self {
            document,
            path,
            cached_snapshot: None,
        }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn cached_snapshot(&self) -> Option<&DocumentSnapshot> {
        self.cached_snapshot.as_ref()
    }

    pub fn cache_snapshot(&mut self, snapshot: DocumentSnapshot) {
        self.cached_snapshot = Some(snapshot);
    }

    pub fn clear_snapshot_cache(&mut self) {
        self.cached_snapshot = None;
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
        self.clear_snapshot_cache();
    }
}

#[derive(Debug, Default)]
pub struct DocumentStore {
    next_id: u64,
    next_session_id: u64,
    documents: HashMap<DocumentId, DocumentEntry>,
    document_sessions: HashMap<DocumentSessionId, DocumentId>,
    sessions_by_document: HashMap<DocumentId, HashSet<DocumentSessionId>>,
}

impl DocumentStore {
    pub fn next_document_id(&mut self) -> DocumentId {
        self.next_id += 1;
        DocumentId::new(self.next_id)
    }

    pub fn insert(&mut self, entry: DocumentEntry) {
        self.documents.insert(entry.document().id(), entry);
    }

    pub fn open_session(&mut self, document_id: DocumentId) -> Option<DocumentSessionId> {
        if !self.documents.contains_key(&document_id) {
            return None;
        }

        self.next_session_id += 1;
        let session_id = DocumentSessionId::new(self.next_session_id);

        self.document_sessions.insert(session_id, document_id);
        self.sessions_by_document
            .entry(document_id)
            .or_default()
            .insert(session_id);

        Some(session_id)
    }

    pub fn resolve_session(&self, document_session_id: DocumentSessionId) -> Option<DocumentId> {
        self.document_sessions.get(&document_session_id).copied()
    }

    pub fn get(&self, document_id: DocumentId) -> Option<&DocumentEntry> {
        self.documents.get(&document_id)
    }

    pub fn get_mut(&mut self, document_id: DocumentId) -> Option<&mut DocumentEntry> {
        self.documents.get_mut(&document_id)
    }

    pub fn remove(&mut self, document_id: DocumentId) -> Option<DocumentEntry> {
        self.documents.remove(&document_id)
    }

    pub fn close_session(
        &mut self,
        document_session_id: DocumentSessionId,
    ) -> Option<(DocumentId, bool)> {
        let document_id = self.document_sessions.remove(&document_session_id)?;
        let sessions = self.sessions_by_document.get_mut(&document_id)?;
        sessions.remove(&document_session_id);

        let should_remove_document = sessions.is_empty();

        if should_remove_document {
            self.sessions_by_document.remove(&document_id);
        }

        Some((document_id, should_remove_document))
    }
}
