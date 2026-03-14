import { invokeCommand } from "../../../shared/ipc/tauri";
import type {
  DocumentSnapshotDto,
  EditCommandDto,
  EditResultDto,
} from "../../../shared/types/editor";

export async function createScratchDocument(
  text = "",
): Promise<DocumentSnapshotDto> {
  return invokeCommand<DocumentSnapshotDto>("create_document", {
    request: { text },
  });
}

export async function getDocument(
  documentId: number,
): Promise<DocumentSnapshotDto> {
  return invokeCommand<DocumentSnapshotDto>("get_document", {
    request: { document_id: documentId },
  });
}

export async function editDocument(
  documentId: number,
  expectedRevision: number | null,
  edit: EditCommandDto,
): Promise<EditResultDto> {
  return invokeCommand<EditResultDto>("edit_document", {
    request: {
      document_id: documentId,
      expected_revision: expectedRevision,
      edit,
    },
  });
}

export async function undoDocument(
  documentId: number,
  expectedRevision: number | null,
): Promise<EditResultDto> {
  return invokeCommand<EditResultDto>("undo_document", {
    request: { document_id: documentId, expected_revision: expectedRevision },
  });
}

export async function redoDocument(
  documentId: number,
  expectedRevision: number | null,
): Promise<EditResultDto> {
  return invokeCommand<EditResultDto>("redo_document", {
    request: { document_id: documentId, expected_revision: expectedRevision },
  });
}
