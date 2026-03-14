use crate::application::commands::{CreateViewport, ScrollViewport};
use crate::application::services::EditorService;

use super::support::MemoryFileSystem;

#[test]
fn viewport_creation_returns_visible_lines_for_document_session() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("alpha\nbeta\ngamma\ndelta");

    let viewport = service
        .create_viewport(CreateViewport {
            document_session_id: snapshot.document_session_id,
            top_line: 1,
            visible_line_count: 2,
        })
        .unwrap();

    assert_eq!(viewport.document_session_id, snapshot.document_session_id);
    assert_eq!(viewport.top_line, 1);
    assert_eq!(viewport.document_line_count, 4);
    assert_eq!(viewport.lines.len(), 2);
    assert_eq!(viewport.lines[0].line_number, 1);
    assert_eq!(viewport.lines[0].text, "beta\n");
    assert_eq!(viewport.lines[1].line_number, 2);
    assert_eq!(viewport.lines[1].text, "gamma\n");
}

#[test]
fn viewport_scroll_updates_top_line_and_reuses_session() {
    let mut service = EditorService::new(MemoryFileSystem::default());
    let snapshot = service.create_document("alpha\nbeta\ngamma\ndelta");

    let viewport = service
        .create_viewport(CreateViewport {
            document_session_id: snapshot.document_session_id,
            top_line: 0,
            visible_line_count: 2,
        })
        .unwrap();

    let scrolled = service
        .scroll_viewport(ScrollViewport {
            viewport_session_id: viewport.viewport_session_id,
            top_line: 2,
        })
        .unwrap();

    assert_eq!(scrolled.viewport_session_id, viewport.viewport_session_id);
    assert_eq!(scrolled.top_line, 2);
    assert_eq!(scrolled.lines.len(), 2);
    assert_eq!(scrolled.lines[0].line_number, 2);
    assert_eq!(scrolled.lines[0].text, "gamma\n");
    assert_eq!(scrolled.lines[1].line_number, 3);
    assert_eq!(scrolled.lines[1].text, "delta");
}
