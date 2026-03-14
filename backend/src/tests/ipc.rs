use std::path::PathBuf;

use crate::application::commands::DocumentSnapshot;
use crate::domain::document::{Document, DocumentId, Edit, RevisionId, TextOffset, TextRange};
use crate::interface::ipc::dto::{
    ChangeSetDto, DocumentSnapshotDto, EditCommandDto, EditDocumentRequest,
    RevisionedDocumentReference, SaveDocumentRequest,
};

#[test]
fn edit_command_dto_rejects_inverted_ranges() {
    let error = Edit::try_from(EditCommandDto::Delete { start: 8, end: 2 }).unwrap_err();

    assert!(error.contains("invalid range"));
}

#[test]
fn edit_command_dto_maps_replace_commands() {
    let edit = Edit::try_from(EditCommandDto::Replace {
        start: 1,
        end: 3,
        text: "xy".to_string(),
    })
    .unwrap();

    assert_eq!(
        edit,
        Edit::Replace {
            range: TextRange::new(TextOffset::new(1), TextOffset::new(3)).unwrap(),
            text: "xy".to_string(),
        }
    );
}

#[test]
fn document_snapshot_dto_serializes_editor_state_for_ipc() {
    let document = Document::open(DocumentId::new(9), "line one\r\nline two");
    let snapshot = DocumentSnapshot::from_document(&document, Some(PathBuf::from("/tmp/doc.txt")));

    let dto = DocumentSnapshotDto::from(snapshot);

    assert_eq!(dto.document_id, 9);
    assert_eq!(dto.revision, RevisionId::initial().value());
    assert_eq!(dto.text, "line one\r\nline two");
    assert_eq!(dto.line_count, 2);
    assert_eq!(dto.newline_mode, "crlf");
    assert_eq!(dto.path.as_deref(), Some("/tmp/doc.txt"));
    assert!(dto.telemetry.is_none());
}

#[test]
fn change_set_dto_preserves_revision_and_range_metadata() {
    let mut document = Document::open(DocumentId::new(4), "hello");
    let change = document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(5),
            text: "!".to_string(),
        })
        .unwrap();

    let dto = ChangeSetDto::from(change.clone());

    assert_eq!(dto.revision_before, change.revision_before().value());
    assert_eq!(dto.revision_after, change.revision_after().value());
    assert_eq!(dto.range_before.start, 5);
    assert_eq!(dto.range_before.end, 5);
    assert_eq!(dto.range_after.start, 5);
    assert_eq!(dto.range_after.end, 6);
    assert_eq!(dto.inserted_text, "!");
    assert_eq!(dto.removed_text, "");
}

#[test]
fn revisioned_requests_capture_expected_revision_metadata() {
    let edit_request = EditDocumentRequest {
        document_id: 4,
        expected_revision: Some(9),
        edit: EditCommandDto::Insert {
            offset: 0,
            text: "a".to_string(),
        },
    };
    let undo_request = RevisionedDocumentReference {
        document_id: 4,
        expected_revision: Some(9),
    };
    let save_request = SaveDocumentRequest {
        document_id: 4,
        expected_revision: Some(9),
        path: None,
    };

    assert_eq!(edit_request.expected_revision, Some(9));
    assert_eq!(undo_request.expected_revision, Some(9));
    assert_eq!(save_request.expected_revision, Some(9));
}
