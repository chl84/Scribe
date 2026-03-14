use std::path::PathBuf;

use crate::application::commands::{EditDocument, SaveDocument};
use crate::application::services::{EditorService, EditorServiceError};
use crate::domain::document::{DocumentSessionId, Edit, TextOffset};

use super::support::{FailingWriteFileSystem, MemoryFileSystem};

#[test]
fn open_document_returns_filesystem_error_for_missing_file() {
    let path = PathBuf::from("/tmp/missing.txt");
    let mut service = EditorService::new(MemoryFileSystem::default());

    let error = service.open_document(&path).unwrap_err();

    assert!(matches!(error, EditorServiceError::FileSystem(_)));
    assert!(error.to_string().contains("missing file"));
}

#[test]
fn save_document_without_known_path_is_rejected() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("draft");

    let error = service
        .save_document(SaveDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            path: None,
        })
        .unwrap_err();

    assert_eq!(
        error.to_string(),
        format!(
            "document session {} has no file path",
            snapshot.document_session_id.value()
        )
    );
}

#[test]
fn save_document_surfaces_filesystem_failure_without_dropping_document_state() {
    let path = PathBuf::from("/tmp/example.txt");
    let mut service = EditorService::new(FailingWriteFileSystem::with_file(&path, "hello"));
    let snapshot = service.open_document(&path).unwrap();

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " world".to_string(),
            },
        })
        .unwrap();

    let error = service
        .save_document(SaveDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            path: None,
        })
        .unwrap_err();

    assert!(matches!(error, EditorServiceError::FileSystem(_)));
    assert_eq!(
        service
            .get_document(snapshot.document_session_id)
            .unwrap()
            .text,
        "hello world"
    );
}

#[test]
fn unknown_document_operations_return_document_session_not_found() {
    let missing_id = DocumentSessionId::new(42);
    let mut service = EditorService::new(MemoryFileSystem::default());

    let get_error = service.get_document(missing_id).unwrap_err();
    let close_error = service.close_document(missing_id).unwrap_err();
    let edit_error = service
        .edit_document(EditDocument {
            document_session_id: missing_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(0),
                text: "x".to_string(),
            },
        })
        .unwrap_err();

    assert!(matches!(
        get_error,
        EditorServiceError::DocumentSessionNotFound(document_session_id)
            if document_session_id == missing_id
    ));
    assert!(matches!(
        close_error,
        EditorServiceError::DocumentSessionNotFound(document_session_id)
            if document_session_id == missing_id
    ));
    assert!(matches!(
        edit_error,
        EditorServiceError::DocumentSessionNotFound(document_session_id)
            if document_session_id == missing_id
    ));
}

#[test]
fn edit_document_rejects_stale_expected_revision() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("hello");

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: Some(snapshot.revision),
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: "!".to_string(),
            },
        })
        .unwrap();

    let error = service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: Some(snapshot.revision),
            edit: Edit::Insert {
                offset: TextOffset::new(6),
                text: "?".to_string(),
            },
        })
        .unwrap_err();

    assert!(matches!(
        error,
        EditorServiceError::StaleRevision {
            document_session_id,
            expected,
            actual,
        } if document_session_id == snapshot.document_session_id
            && expected == snapshot.revision
            && actual == snapshot.revision.next()
    ));
}

#[test]
fn undo_document_rejects_stale_expected_revision() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("hello");

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: Some(snapshot.revision),
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: "!".to_string(),
            },
        })
        .unwrap();

    let error = service
        .undo_document(snapshot.document_session_id, Some(snapshot.revision))
        .unwrap_err();

    assert!(matches!(
        error,
        EditorServiceError::StaleRevision {
            document_session_id,
            expected,
            actual,
        } if document_session_id == snapshot.document_session_id
            && expected == snapshot.revision
            && actual == snapshot.revision.next()
    ));
}
