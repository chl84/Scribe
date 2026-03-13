use super::{
    Document, DocumentError, DocumentId, Edit, NewlineMode, RevisionId, TextOffset, TextRange,
};

#[test]
fn text_range_rejects_inverted_bounds() {
    let error = TextRange::new(TextOffset::new(5), TextOffset::new(4)).unwrap_err();

    assert_eq!(
        error,
        DocumentError::InvalidRange {
            start: TextOffset::new(5),
            end: TextOffset::new(4),
        }
    );
}

#[test]
fn document_starts_with_initial_revision_and_detected_newline_mode() {
    let document = Document::open(DocumentId::new(7), "line one\r\nline two");

    assert_eq!(document.id(), DocumentId::new(7));
    assert_eq!(document.revision(), RevisionId::initial());
    assert_eq!(document.newline_policy().preferred_mode(), NewlineMode::Crlf);
    assert!(document.newline_policy().preserve_existing());
}

#[test]
fn insert_updates_text_revision_and_change_set() {
    let mut document = Document::open(DocumentId::new(1), "hello");

    let change_set = document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(5),
            text: " world".to_string(),
        })
        .unwrap();

    assert_eq!(document.text(), "hello world");
    assert_eq!(document.revision(), RevisionId::initial().next());
    assert_eq!(change_set.inserted_text(), " world");
    assert_eq!(change_set.removed_text(), "");
    assert_eq!(change_set.revision_before(), RevisionId::initial());
    assert_eq!(change_set.revision_after(), RevisionId::initial().next());
}

#[test]
fn delete_returns_removed_text_and_inverse_edit() {
    let mut document = Document::open(DocumentId::new(2), "hello world");
    let range = TextRange::new(TextOffset::new(5), TextOffset::new(11)).unwrap();

    let change_set = document.apply_edit(Edit::Delete { range }).unwrap();

    assert_eq!(document.text(), "hello");
    assert_eq!(change_set.removed_text(), " world");
    assert_eq!(
        change_set.inverse_edit(),
        &Edit::Insert {
            offset: TextOffset::new(5),
            text: " world".to_string(),
        }
    );
}

#[test]
fn replace_updates_text_and_tracks_replaced_range() {
    let mut document = Document::open(DocumentId::new(3), "hello world");
    let range = TextRange::new(TextOffset::new(6), TextOffset::new(11)).unwrap();

    let change_set = document
        .apply_edit(Edit::Replace {
            range,
            text: "scribe".to_string(),
        })
        .unwrap();

    assert_eq!(document.text(), "hello scribe");
    assert_eq!(change_set.removed_text(), "world");
    assert_eq!(change_set.inserted_text(), "scribe");
    assert_eq!(
        change_set.range_after(),
        TextRange::new(TextOffset::new(6), TextOffset::new(12)).unwrap()
    );
}

#[test]
fn edit_rejects_out_of_bounds_range() {
    let mut document = Document::open(DocumentId::new(4), "hello");
    let range = TextRange::new(TextOffset::new(1), TextOffset::new(8)).unwrap();

    let error = document.apply_edit(Edit::Delete { range }).unwrap_err();

    assert_eq!(
        error,
        DocumentError::RangeOutOfBounds {
            len: 5,
            start: TextOffset::new(1),
            end: TextOffset::new(8),
        }
    );
}

#[test]
fn edit_rejects_non_boundary_utf8_offset() {
    let mut document = Document::open(DocumentId::new(5), "a🙂b");

    let error = document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(2),
            text: "!".to_string(),
        })
        .unwrap_err();

    assert_eq!(
        error,
        DocumentError::InvalidUtf8Boundary {
            offset: TextOffset::new(2),
        }
    );
}
