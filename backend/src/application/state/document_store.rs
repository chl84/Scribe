use std::collections::HashMap;
use std::path::PathBuf;

use crate::application::commands::DocumentSnapshot;
use crate::domain::document::{Document, DocumentId};

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
    documents: HashMap<DocumentId, DocumentEntry>,
}

impl DocumentStore {
    pub fn next_document_id(&mut self) -> DocumentId {
        self.next_id += 1;
        DocumentId::new(self.next_id)
    }

    pub fn insert(&mut self, entry: DocumentEntry) {
        self.documents.insert(entry.document().id(), entry);
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
}
