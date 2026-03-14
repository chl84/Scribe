use super::{
    CursorMove, Document, DocumentError, DocumentId, Edit, NewlineMode, Position, RevisionId,
    Selection, TextOffset, TextRange,
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
fn insert_undo_redo_cycle_restores_document() {
    let mut document = Document::open(DocumentId::new(11), "hello");

    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(5),
            text: "!".to_string(),
        })
        .unwrap();
    assert_eq!(document.text(), "hello!");

    let undo_changes = document.undo().unwrap().unwrap();
    assert_eq!(undo_changes.len(), 1);
    assert_eq!(document.text(), "hello");

    let redo_changes = document.redo().unwrap().unwrap();
    assert_eq!(redo_changes.len(), 1);
    assert_eq!(document.text(), "hello!");
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
fn delete_undo_redo_cycle_restores_document() {
    let mut document = Document::open(DocumentId::new(12), "hello world");
    let range = TextRange::new(TextOffset::new(5), TextOffset::new(11)).unwrap();

    document.apply_edit(Edit::Delete { range }).unwrap();
    assert_eq!(document.text(), "hello");

    document.undo().unwrap();
    assert_eq!(document.text(), "hello world");

    document.redo().unwrap();
    assert_eq!(document.text(), "hello");
}

#[test]
fn transaction_groups_multiple_edits_into_single_undo_step() {
    let mut document = Document::open(DocumentId::new(13), "");

    document.begin_transaction();
    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(0),
            text: "hel".to_string(),
        })
        .unwrap();
    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(3),
            text: "lo".to_string(),
        })
        .unwrap();
    document.commit_transaction();

    assert_eq!(document.text(), "hello");
    assert_eq!(document.undo().unwrap().unwrap().len(), 2);
    assert_eq!(document.text(), "");
    assert_eq!(document.redo().unwrap().unwrap().len(), 2);
    assert_eq!(document.text(), "hello");
}

#[test]
fn selection_supports_caret_and_non_empty_range() {
    let caret = Selection::caret(TextOffset::new(3));
    let range = Selection::new(TextOffset::new(7), TextOffset::new(3));

    assert!(caret.is_caret());
    assert_eq!(caret.range().unwrap(), TextRange::empty_at(TextOffset::new(3)));
    assert_eq!(
        range.range().unwrap(),
        TextRange::new(TextOffset::new(3), TextOffset::new(7)).unwrap()
    );
}

#[test]
fn cursor_movement_rules_are_separate_from_storage_logic() {
    let document = Document::open(DocumentId::new(14), "a🙂\nxyz");
    let selection = Selection::caret(TextOffset::new(5));

    assert_eq!(
        document
            .move_selection(selection, CursorMove::Left)
            .unwrap(),
        Selection::caret(TextOffset::new(1))
    );
    assert_eq!(
        document
            .move_selection(Selection::caret(TextOffset::new(1)), CursorMove::Down)
            .unwrap(),
        Selection::caret(TextOffset::new(7))
    );
}

#[test]
fn moving_with_a_non_empty_selection_collapses_before_advancing() {
    let document = Document::open(DocumentId::new(15), "abcdef");
    let selection = Selection::new(TextOffset::new(5), TextOffset::new(2));

    assert_eq!(
        document
            .move_selection(selection, CursorMove::Right)
            .unwrap(),
        Selection::caret(TextOffset::new(5))
    );
    assert_eq!(
        document
            .move_selection(selection, CursorMove::Left)
            .unwrap(),
        Selection::caret(TextOffset::new(2))
    );
}

#[test]
fn line_index_maps_offsets_to_positions() {
    let document = Document::open(DocumentId::new(6), "alpha\nbeta\n\nz");

    assert_eq!(document.line_count(), 4);
    assert_eq!(
        document.offset_to_position(TextOffset::new(6)).unwrap(),
        Position::new(1, 0)
    );
    assert_eq!(
        document.offset_to_position(TextOffset::new(10)).unwrap(),
        Position::new(1, 4)
    );
}

#[test]
fn line_index_remains_correct_after_inserting_leading_newline() {
    let mut document = Document::open(DocumentId::new(19), "alpha");

    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(0),
            text: "\n".to_string(),
        })
        .unwrap();

    assert_eq!(document.line_count(), 2);
    assert_eq!(
        document.offset_to_position(TextOffset::new(1)).unwrap(),
        Position::new(1, 0)
    );
    assert_eq!(
        document.position_to_offset(Position::new(1, 0)).unwrap(),
        TextOffset::new(1)
    );
}

#[test]
fn line_index_maps_positions_to_offsets() {
    let document = Document::open(DocumentId::new(7), "alpha\nbeta\n🙂z");

    assert_eq!(
        document.position_to_offset(Position::new(0, 3)).unwrap(),
        TextOffset::new(3)
    );
    assert_eq!(
        document.position_to_offset(Position::new(2, 1)).unwrap(),
        TextOffset::new(15)
    );
}

#[test]
fn delete_across_line_boundaries_updates_text_and_line_count() {
    let mut document = Document::open(DocumentId::new(8), "one\ntwo\nthree\nfour");
    let range = TextRange::new(TextOffset::new(2), TextOffset::new(13)).unwrap();

    document.apply_edit(Edit::Delete { range }).unwrap();

    assert_eq!(document.text(), "on\nfour");
    assert_eq!(document.line_count(), 2);
}

#[test]
fn large_insert_keeps_piece_table_content_consistent() {
    let mut document = Document::open(DocumentId::new(9), "");
    let inserted = "line\n".repeat(1024);

    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(0),
            text: inserted.clone(),
        })
        .unwrap();

    assert_eq!(document.text(), inserted);
    assert_eq!(document.line_count(), 1025);
}

#[test]
fn position_lookup_rejects_columns_past_line_end() {
    let document = Document::open(DocumentId::new(10), "one\n\nthree");

    let error = document.position_to_offset(Position::new(1, 1)).unwrap_err();

    assert_eq!(
        error,
        DocumentError::PositionOutOfBounds { line: 1, column: 1 }
    );
}

#[test]
fn property_style_edit_sequence_matches_string_model() {
    let mut document = Document::open(DocumentId::new(16), "alpha");
    let mut expected = String::from("alpha");

    let edits = vec![
        Edit::Insert {
            offset: TextOffset::new(5),
            text: "\nbeta".to_string(),
        },
        Edit::Replace {
            range: TextRange::new(TextOffset::new(0), TextOffset::new(5)).unwrap(),
            text: "ALPHA".to_string(),
        },
        Edit::Delete {
            range: TextRange::new(TextOffset::new(5), TextOffset::new(6)).unwrap(),
        },
        Edit::Insert {
            offset: TextOffset::new(5),
            text: " ".to_string(),
        },
    ];

    for edit in edits {
        match &edit {
            Edit::Insert { offset, text } => expected.insert_str(offset.value(), text),
            Edit::Delete { range } => {
                expected.replace_range(range.start().value()..range.end().value(), "")
            }
            Edit::Replace { range, text } => {
                expected.replace_range(range.start().value()..range.end().value(), text)
            }
        }

        document.apply_edit(edit).unwrap();
        assert_eq!(document.text(), expected);
    }
}

#[test]
fn newline_regression_preserves_detected_crlf_mode() {
    let mut document = Document::open(DocumentId::new(17), "one\r\ntwo\r\n");

    document
        .apply_edit(Edit::Insert {
            offset: TextOffset::new(8),
            text: "three\r\n".to_string(),
        })
        .unwrap();

    assert_eq!(document.newline_policy().preferred_mode(), NewlineMode::Crlf);
    assert_eq!(document.line_count(), 4);
}

#[test]
fn utf8_regression_handles_non_ascii_replace_and_navigation() {
    let mut document = Document::open(DocumentId::new(18), "blåbær");

    document
        .apply_edit(Edit::Replace {
            range: TextRange::new(TextOffset::new(0), TextOffset::new(8)).unwrap(),
            text: "東京🙂".to_string(),
        })
        .unwrap();

    assert_eq!(document.text(), "東京🙂");
    assert_eq!(
        document.offset_to_position(TextOffset::new(6)).unwrap(),
        Position::new(0, 2)
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
