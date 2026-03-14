use std::thread;

use crate::application::commands::EditDocument;
use crate::application::runtime::{EditorRuntime, EditorRuntimeError};
use crate::domain::document::{Edit, TextOffset};

use super::support::MemoryFileSystem;

#[test]
fn editor_runtime_executes_document_lifecycle_on_dedicated_thread() {
    let runtime = EditorRuntime::new(MemoryFileSystem::default());

    let snapshot = runtime.create_document("hello").unwrap();
    let edited = runtime
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: None,
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: " world".to_string(),
            },
        })
        .unwrap();
    let current = runtime.get_document(snapshot.document_session_id).unwrap();

    assert_eq!(edited.changes.len(), 1);
    assert_eq!(current.text, "hello world");
}

#[test]
fn editor_runtime_serializes_commands_from_multiple_callers() {
    let runtime = EditorRuntime::new(MemoryFileSystem::default());
    let snapshot = runtime.create_document("").unwrap();

    let mut handles = Vec::new();

    for text in ["a", "b", "c", "d"] {
        let runtime = runtime.clone();
        let document_session_id = snapshot.document_session_id;
        let text = text.to_string();

        handles.push(thread::spawn(move || {
            runtime
                .edit_document(EditDocument {
                    document_session_id,
                    expected_revision: None,
                    edit: Edit::Insert {
                        offset: TextOffset::new(0),
                        text,
                    },
                })
                .unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let current = runtime.get_document(snapshot.document_session_id).unwrap();

    assert_eq!(current.text.len(), 4);
    for expected in ["a", "b", "c", "d"] {
        assert!(current.text.contains(expected));
    }
}

#[test]
fn editor_runtime_reuses_cached_snapshot_on_repeated_reads() {
    let runtime = EditorRuntime::new(MemoryFileSystem::default());
    let snapshot = runtime.create_document("hello").unwrap();

    let first = runtime.get_document(snapshot.document_session_id).unwrap();
    let second = runtime.get_document(snapshot.document_session_id).unwrap();

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

#[test]
fn editor_runtime_surfaces_stale_revision_errors() {
    let runtime = EditorRuntime::new(MemoryFileSystem::default());
    let snapshot = runtime.create_document("hello").unwrap();

    runtime
        .edit_document(EditDocument {
            document_session_id: snapshot.document_session_id,
            expected_revision: Some(snapshot.revision),
            edit: Edit::Insert {
                offset: TextOffset::new(5),
                text: "!".to_string(),
            },
        })
        .unwrap();

    let error = runtime
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
        EditorRuntimeError::Service(crate::application::services::EditorServiceError::StaleRevision {
            document_session_id,
            expected,
            actual,
        }) if document_session_id == snapshot.document_session_id
            && expected == snapshot.revision
            && actual == snapshot.revision.next()
    ));
}
