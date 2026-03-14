use std::path::PathBuf;

use crate::application::commands::{EditDocument, SaveDocument};
use crate::application::services::{EditorService, EditorServiceError};
use crate::domain::document::{Edit, TextOffset};

use super::support::WritableMemoryFileSystem;

#[test]
fn unsaved_edits_remain_in_backend_state_until_explicit_save() {
    let path = PathBuf::from("/tmp/recovery.txt");
    let filesystem = WritableMemoryFileSystem::with_file(&path, "alpha");
    let mut service = EditorService::new(filesystem);
    let snapshot = service.open_document(&path).unwrap();

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " beta".to_string(),
            },
        })
        .unwrap();

    let current = service.get_document(snapshot.document_session_id).unwrap();

    assert_eq!(current.text, "alpha beta");
    assert_eq!(current.path, Some(path.clone()));
}

#[test]
fn explicit_save_persists_current_backend_state_to_filesystem() {
    let path = PathBuf::from("/tmp/recovery-save.txt");
    let filesystem = WritableMemoryFileSystem::with_file(&path, "draft");
    let mut service = EditorService::new(filesystem);
    let snapshot = service.open_document(&path).unwrap();

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " updated".to_string(),
            },
        })
        .unwrap();

    let saved = service
        .save_document(SaveDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            path: None,
        })
        .unwrap();

    assert_eq!(saved.text, "draft updated");
    assert_eq!(
        service
            .get_document(snapshot.document_session_id)
            .unwrap()
            .text,
        "draft updated"
    );
}

#[test]
fn explicit_save_without_path_stays_an_application_error() {
    let mut service = EditorService::new(WritableMemoryFileSystem::default());
    let snapshot = service.create_document("draft");

    let error = service
        .save_document(SaveDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            path: None,
        })
        .unwrap_err();

    assert!(matches!(
        error,
        EditorServiceError::MissingDocumentPath(document_session_id)
        if document_session_id == snapshot.document_session_id
    ));
}
