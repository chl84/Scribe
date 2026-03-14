use std::path::PathBuf;

use tauri::State;

use crate::application::commands::{CreateViewport, EditDocument, SaveDocument, ScrollViewport};
use crate::application::runtime::EditorRuntime;
use crate::domain::document::{DocumentSessionId, RevisionId, ViewportSessionId};
use crate::infrastructure::filesystem::LocalFileSystem;
use crate::interface::ipc::dto::{
    CreateDocumentRequest, CreateViewportRequest, DocumentReference, DocumentSnapshotDto,
    EditDocumentRequest, EditResultDto, OpenDocumentRequest, RevisionedDocumentReference,
    SaveDocumentRequest, ScrollViewportRequest, ViewportReference, ViewportSnapshotDto,
};

type SharedEditorRuntime = EditorRuntime<LocalFileSystem>;

#[tauri::command]
pub fn create_document(
    state: State<'_, SharedEditorRuntime>,
    request: CreateDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    Ok(state
        .create_document(request.text.unwrap_or_default())
        .map_err(|error| error.to_string())?
        .into())
}

#[tauri::command]
pub fn open_document(
    state: State<'_, SharedEditorRuntime>,
    request: OpenDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    state
        .open_document(request.path)
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_document(
    state: State<'_, SharedEditorRuntime>,
    request: DocumentReference,
) -> Result<DocumentSnapshotDto, String> {
    state
        .get_document(DocumentSessionId::new(request.document_session_id))
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn edit_document(
    state: State<'_, SharedEditorRuntime>,
    request: EditDocumentRequest,
) -> Result<EditResultDto, String> {
    let edit = request.edit.try_into()?;

    state
        .edit_document(EditDocument {
            document_session_id: DocumentSessionId::new(request.document_session_id),
            expected_revision: request.expected_revision.map(RevisionId::new),
            edit,
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_viewport(
    state: State<'_, SharedEditorRuntime>,
    request: CreateViewportRequest,
) -> Result<ViewportSnapshotDto, String> {
    state
        .create_viewport(CreateViewport {
            document_session_id: DocumentSessionId::new(request.document_session_id),
            top_line: request.top_line,
            visible_line_count: request.visible_line_count,
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_viewport(
    state: State<'_, SharedEditorRuntime>,
    request: ViewportReference,
) -> Result<ViewportSnapshotDto, String> {
    state
        .get_viewport(ViewportSessionId::new(request.viewport_session_id))
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn scroll_viewport(
    state: State<'_, SharedEditorRuntime>,
    request: ScrollViewportRequest,
) -> Result<ViewportSnapshotDto, String> {
    state
        .scroll_viewport(ScrollViewport {
            viewport_session_id: ViewportSessionId::new(request.viewport_session_id),
            top_line: request.top_line,
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn undo_document(
    state: State<'_, SharedEditorRuntime>,
    request: RevisionedDocumentReference,
) -> Result<EditResultDto, String> {
    state
        .undo_document_with_revision(
            DocumentSessionId::new(request.document_session_id),
            request.expected_revision.map(RevisionId::new),
        )
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn redo_document(
    state: State<'_, SharedEditorRuntime>,
    request: RevisionedDocumentReference,
) -> Result<EditResultDto, String> {
    state
        .redo_document_with_revision(
            DocumentSessionId::new(request.document_session_id),
            request.expected_revision.map(RevisionId::new),
        )
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_document(
    state: State<'_, SharedEditorRuntime>,
    request: SaveDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    state
        .save_document(SaveDocument {
            document_session_id: DocumentSessionId::new(request.document_session_id),
            expected_revision: request.expected_revision.map(RevisionId::new),
            path: request.path.map(PathBuf::from),
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn close_document(
    state: State<'_, SharedEditorRuntime>,
    request: DocumentReference,
) -> Result<(), String> {
    state
        .close_document(DocumentSessionId::new(request.document_session_id))
        .map_err(|error| error.to_string())
}
