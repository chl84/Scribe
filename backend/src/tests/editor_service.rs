use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::application::commands::{EditDocument, SaveDocument};
use crate::application::services::EditorService;
use crate::domain::document::{DocumentId, Edit, TextOffset, TextRange};
use crate::infrastructure::filesystem::FileSystem;

#[derive(Default)]
struct MemoryFileSystem {
    files: HashMap<PathBuf, String>,
}

impl MemoryFileSystem {
}

impl FileSystem for MemoryFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing file"))
    }

    fn write_string(&self, _path: &Path, _contents: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "immutable memory filesystem",
        ))
    }
}

#[derive(Default)]
struct WritableMemoryFileSystem {
    files: std::sync::Mutex<HashMap<PathBuf, String>>,
}

impl WritableMemoryFileSystem {
    fn with_file(path: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        let mut files = HashMap::new();
        files.insert(path.into(), contents.into());
        Self {
            files: std::sync::Mutex::new(files),
        }
    }

}

impl FileSystem for WritableMemoryFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "lock poisoned"))?
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing file"))
    }

    fn write_string(&self, path: &Path, contents: &str) -> io::Result<()> {
        self.files
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "lock poisoned"))?
            .insert(path.to_path_buf(), contents.to_string());
        Ok(())
    }
}

#[test]
fn application_service_can_open_edit_and_save_document() {
    let path = PathBuf::from("/tmp/example.txt");
    let filesystem = WritableMemoryFileSystem::with_file(&path, "hello");
    let mut service = EditorService::new(filesystem);

    let snapshot = service.open_document(&path).unwrap();
    assert_eq!(snapshot.text, "hello");

    let result = service
        .edit_document(EditDocument {
            document_id: snapshot.document_id,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " world".to_string(),
            },
        })
        .unwrap();
    assert_eq!(result.changes.len(), 1);

    let saved = service
        .save_document(SaveDocument {
            document_id: snapshot.document_id,
            path: None,
        })
        .unwrap();
    assert_eq!(saved.text, "hello world");
}

#[test]
fn application_service_handles_large_document_round_trip() {
    let path = PathBuf::from("/tmp/large.txt");
    let original = "row-0123456789abcdef\n".repeat(2048);
    let filesystem = WritableMemoryFileSystem::with_file(&path, original.clone());
    let mut service = EditorService::new(filesystem);

    let snapshot = service.open_document(&path).unwrap();
    assert_eq!(snapshot.text.len(), original.len());

    service
        .edit_document(EditDocument {
            document_id: snapshot.document_id,
            edit: Edit::Insert {
                offset: TextOffset::new(snapshot.text.len()),
                text: "tail".to_string(),
            },
        })
        .unwrap();

    let saved = service
        .save_document(SaveDocument {
            document_id: snapshot.document_id,
            path: None,
        })
        .unwrap();

    assert!(saved.text.ends_with("tail"));
}

#[test]
fn application_service_supports_undo_and_redo() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("hello");

    service
        .edit_document(EditDocument {
            document_id: snapshot.document_id,
            edit: Edit::Replace {
                range: TextRange::new(TextOffset::new(0), TextOffset::new(5)).unwrap(),
                text: "scribe".to_string(),
            },
        })
        .unwrap();

    let undone = service.undo_document(snapshot.document_id).unwrap();
    assert_eq!(undone.changes.len(), 1);
    assert_eq!(
        service.get_document(snapshot.document_id).unwrap().text,
        "hello"
    );

    let redone = service.redo_document(snapshot.document_id).unwrap();
    assert_eq!(redone.changes.len(), 1);
    assert_eq!(
        service.get_document(snapshot.document_id).unwrap().text,
        "scribe"
    );
}

#[test]
fn application_service_can_close_documents() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("");

    service.close_document(snapshot.document_id).unwrap();

    assert!(service.get_document(DocumentId::new(1)).is_err());
}
