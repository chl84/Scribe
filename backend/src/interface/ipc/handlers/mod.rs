use std::path::PathBuf;
use std::sync::Mutex;

use tauri::State;

use crate::application::commands::{EditDocument, SaveDocument};
use crate::application::services::EditorService;
use crate::domain::document::DocumentId;
use crate::infrastructure::filesystem::LocalFileSystem;
use crate::interface::ipc::dto::{
    CreateDocumentRequest, DocumentReference, DocumentSnapshotDto, EditDocumentRequest,
    EditResultDto, OpenDocumentRequest, SaveDocumentRequest,
};

type SharedEditorService = Mutex<EditorService<LocalFileSystem>>;

#[tauri::command]
pub fn create_document(
    state: State<'_, SharedEditorService>,
    request: CreateDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    Ok(service
        .create_document(request.text.unwrap_or_default())
        .into())
}

#[tauri::command]
pub fn open_document(
    state: State<'_, SharedEditorService>,
    request: OpenDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    service
        .open_document(request.path)
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_document(
    state: State<'_, SharedEditorService>,
    request: DocumentReference,
) -> Result<DocumentSnapshotDto, String> {
    let service = state.lock().map_err(|error| error.to_string())?;
    service
        .get_document(DocumentId::new(request.document_id))
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn edit_document(
    state: State<'_, SharedEditorService>,
    request: EditDocumentRequest,
) -> Result<EditResultDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    let edit = request.edit.try_into()?;

    service
        .edit_document(EditDocument {
            document_id: DocumentId::new(request.document_id),
            edit,
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn undo_document(
    state: State<'_, SharedEditorService>,
    request: DocumentReference,
) -> Result<EditResultDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    service
        .undo_document(DocumentId::new(request.document_id))
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn redo_document(
    state: State<'_, SharedEditorService>,
    request: DocumentReference,
) -> Result<EditResultDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    service
        .redo_document(DocumentId::new(request.document_id))
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_document(
    state: State<'_, SharedEditorService>,
    request: SaveDocumentRequest,
) -> Result<DocumentSnapshotDto, String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;

    service
        .save_document(SaveDocument {
            document_id: DocumentId::new(request.document_id),
            path: request.path.map(PathBuf::from),
        })
        .map(Into::into)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn close_document(
    state: State<'_, SharedEditorService>,
    request: DocumentReference,
) -> Result<(), String> {
    let mut service = state.lock().map_err(|error| error.to_string())?;
    service
        .close_document(DocumentId::new(request.document_id))
        .map_err(|error| error.to_string())
}
