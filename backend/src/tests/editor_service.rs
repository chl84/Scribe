use std::path::PathBuf;

use crate::application::commands::{EditDocument, SaveDocument};
use crate::application::services::EditorService;
use crate::domain::document::{DocumentSessionId, Edit, TextOffset, TextRange};

use super::support::{MemoryFileSystem, WritableMemoryFileSystem};

#[test]
fn application_service_can_open_edit_and_save_document() {
    let path = PathBuf::from("/tmp/example.txt");
    let filesystem = WritableMemoryFileSystem::with_file(&path, "hello");
    let mut service = EditorService::new(filesystem);

    let snapshot = service.open_document(&path).unwrap();
    assert_eq!(snapshot.text, "hello");

    let result = service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " world".to_string(),
            },
        })
        .unwrap();
    assert_eq!(result.changes.len(), 1);
    assert!(result.telemetry.document_operation_nanos.is_some());

    let saved = service
        .save_document(SaveDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
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
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(snapshot.text.len()),
                text: "tail".to_string(),
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

    assert!(saved.text.ends_with("tail"));
}

#[test]
fn application_service_supports_undo_and_redo() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("hello");

    service
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Replace {
                range: TextRange::new(TextOffset::new(0), TextOffset::new(5)).unwrap(),
                text: "scribe".to_string(),
            },
        })
        .unwrap();

    let undone = service
        .undo_document(snapshot.document_session_id, None)
        .unwrap();
    assert_eq!(undone.changes.len(), 1);
    assert!(undone.telemetry.document_operation_nanos.is_some());
    assert_eq!(
        service
            .get_document(snapshot.document_session_id)
            .unwrap()
            .text,
        "hello"
    );

    let redone = service
        .redo_document(snapshot.document_session_id, None)
        .unwrap();
    assert_eq!(redone.changes.len(), 1);
    assert!(redone.telemetry.document_operation_nanos.is_some());
    assert_eq!(
        service
            .get_document(snapshot.document_session_id)
            .unwrap()
            .text,
        "scribe"
    );
}

#[test]
fn application_service_can_close_documents() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("");

    service
        .close_document(snapshot.document_session_id)
        .unwrap();

    assert!(service
        .get_document(DocumentSessionId::new(snapshot.document_session_id.value()))
        .is_err());
}

#[test]
fn application_service_reuses_cached_snapshot_for_repeated_reads() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("hello");

    let first = service.get_document(snapshot.document_session_id).unwrap();
    let second = service.get_document(snapshot.document_session_id).unwrap();

    assert_eq!(first.text, "hello");
    assert_eq!(second.text, "hello");
    assert_eq!(
        second
            .telemetry
            .as_ref()
            .and_then(|telemetry| telemetry.snapshot_build_nanos),
        Some(0)
    );
}
