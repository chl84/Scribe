use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;

use crate::application::commands::{DocumentSnapshot, EditDocument, EditResult, SaveDocument};
use crate::application::services::{EditorService, EditorServiceError};
use crate::domain::document::{DocumentId, RevisionId};
use crate::infrastructure::filesystem::FileSystem;

#[derive(Debug)]
pub enum EditorRuntimeError {
    Service(EditorServiceError),
    RuntimeUnavailable,
}

impl std::fmt::Display for EditorRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Service(error) => write!(f, "{error}"),
            Self::RuntimeUnavailable => write!(f, "editor runtime is unavailable"),
        }
    }
}

impl std::error::Error for EditorRuntimeError {}

impl From<EditorServiceError> for EditorRuntimeError {
    fn from(value: EditorServiceError) -> Self {
        Self::Service(value)
    }
}

impl From<RecvError> for EditorRuntimeError {
    fn from(_: RecvError) -> Self {
        Self::RuntimeUnavailable
    }
}

type RuntimeResult<T> = Result<T, EditorRuntimeError>;
type RuntimeResponse<T> = Sender<RuntimeResult<T>>;

enum EditorRuntimeRequest {
    CreateDocument {
        text: String,
        response: RuntimeResponse<DocumentSnapshot>,
    },
    OpenDocument {
        path: PathBuf,
        response: RuntimeResponse<DocumentSnapshot>,
    },
    GetDocument {
        document_id: DocumentId,
        response: RuntimeResponse<DocumentSnapshot>,
    },
    EditDocument {
        command: EditDocument,
        response: RuntimeResponse<EditResult>,
    },
    UndoDocument {
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
        response: RuntimeResponse<EditResult>,
    },
    RedoDocument {
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
        response: RuntimeResponse<EditResult>,
    },
    SaveDocument {
        command: SaveDocument,
        response: RuntimeResponse<DocumentSnapshot>,
    },
    CloseDocument {
        document_id: DocumentId,
        response: RuntimeResponse<()>,
    },
}

pub struct EditorRuntime<F: FileSystem + Send + 'static> {
    sender: Sender<EditorRuntimeRequest>,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FileSystem + Send + 'static> Clone for EditorRuntime<F> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F: FileSystem + Send + 'static> EditorRuntime<F> {
    pub fn new(filesystem: F) -> Self {
        let (sender, receiver) = channel();

        thread::Builder::new()
            .name("scribe-editor-runtime".to_string())
            .spawn(move || run_editor_runtime(EditorService::new(filesystem), receiver))
            .expect("failed to spawn editor runtime");

        Self {
            sender,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn create_document(&self, text: impl Into<String>) -> RuntimeResult<DocumentSnapshot> {
        let text = text.into();
        self.send_request(move |response| EditorRuntimeRequest::CreateDocument { text, response })
    }

    pub fn open_document(&self, path: impl Into<PathBuf>) -> RuntimeResult<DocumentSnapshot> {
        let path = path.into();
        self.send_request(move |response| EditorRuntimeRequest::OpenDocument { path, response })
    }

    pub fn get_document(&self, document_id: DocumentId) -> RuntimeResult<DocumentSnapshot> {
        self.send_request(move |response| EditorRuntimeRequest::GetDocument {
            document_id,
            response,
        })
    }

    pub fn edit_document(&self, command: EditDocument) -> RuntimeResult<EditResult> {
        self.send_request(move |response| EditorRuntimeRequest::EditDocument { command, response })
    }

    pub fn undo_document(&self, document_id: DocumentId) -> RuntimeResult<EditResult> {
        self.undo_document_with_revision(document_id, None)
    }

    pub fn undo_document_with_revision(
        &self,
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
    ) -> RuntimeResult<EditResult> {
        self.send_request(move |response| EditorRuntimeRequest::UndoDocument {
            document_id,
            expected_revision,
            response,
        })
    }

    pub fn redo_document(&self, document_id: DocumentId) -> RuntimeResult<EditResult> {
        self.redo_document_with_revision(document_id, None)
    }

    pub fn redo_document_with_revision(
        &self,
        document_id: DocumentId,
        expected_revision: Option<RevisionId>,
    ) -> RuntimeResult<EditResult> {
        self.send_request(move |response| EditorRuntimeRequest::RedoDocument {
            document_id,
            expected_revision,
            response,
        })
    }

    pub fn save_document(&self, command: SaveDocument) -> RuntimeResult<DocumentSnapshot> {
        self.send_request(move |response| EditorRuntimeRequest::SaveDocument { command, response })
    }

    pub fn close_document(&self, document_id: DocumentId) -> RuntimeResult<()> {
        self.send_request(move |response| EditorRuntimeRequest::CloseDocument {
            document_id,
            response,
        })
    }

    fn send_request<T>(
        &self,
        build_request: impl FnOnce(RuntimeResponse<T>) -> EditorRuntimeRequest,
    ) -> RuntimeResult<T> {
        let (response_sender, response_receiver) = channel();

        self.sender
            .send(build_request(response_sender))
            .map_err(|_| EditorRuntimeError::RuntimeUnavailable)?;

        response_receiver.recv()?
    }
}

fn run_editor_runtime<F: FileSystem + Send + 'static>(
    mut service: EditorService<F>,
    receiver: Receiver<EditorRuntimeRequest>,
) {
    while let Ok(request) = receiver.recv() {
        match request {
            EditorRuntimeRequest::CreateDocument { text, response } => {
                let _ = response.send(Ok(service.create_document(text)));
            }
            EditorRuntimeRequest::OpenDocument { path, response } => {
                let _ = response.send(service.open_document(path).map_err(Into::into));
            }
            EditorRuntimeRequest::GetDocument {
                document_id,
                response,
            } => {
                let _ = response.send(service.get_document(document_id).map_err(Into::into));
            }
            EditorRuntimeRequest::EditDocument { command, response } => {
                let _ = response.send(service.edit_document(command).map_err(Into::into));
            }
            EditorRuntimeRequest::UndoDocument {
                document_id,
                expected_revision,
                response,
            } => {
                let _ = response.send(
                    service
                        .undo_document(document_id, expected_revision)
                        .map_err(Into::into),
                );
            }
            EditorRuntimeRequest::RedoDocument {
                document_id,
                expected_revision,
                response,
            } => {
                let _ = response.send(
                    service
                        .redo_document(document_id, expected_revision)
                        .map_err(Into::into),
                );
            }
            EditorRuntimeRequest::SaveDocument { command, response } => {
                let _ = response.send(service.save_document(command).map_err(Into::into));
            }
            EditorRuntimeRequest::CloseDocument {
                document_id,
                response,
            } => {
                let _ = response.send(service.close_document(document_id).map_err(Into::into));
            }
        }
    }
}
