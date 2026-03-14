import { get, writable } from "svelte/store";

import {
  createScratchDocument,
  editDocument,
  getDocument,
  redoDocument,
  undoDocument,
} from "../commands/editor-ipc";
import {
  ensureTauriRuntime,
  TauriCommandError,
} from "../../../shared/ipc/tauri";
import type { DocumentSnapshotDto } from "../../../shared/types/editor";
import { deriveEditCommand } from "../commands/text-edit";
import {
  now,
  recordEditorPipelineSample,
  telemetryToMs,
  waitForNextPaint,
} from "../../../shared/utils/performance";

export interface EditorState {
  status: "booting" | "ready" | "syncing" | "error";
  snapshot: DocumentSnapshotDto | null;
  draftText: string;
  isDirty: boolean;
  error: string | null;
}

const initialState: EditorState = {
  status: "booting",
  snapshot: null,
  draftText: "",
  isDirty: false,
  error: null,
};

function createEditorStore() {
  const store = writable<EditorState>(initialState);
  let activeSync: Promise<void> | null = null;
  let lastInputReceivedAt: number | null = null;

  function setSnapshot(snapshot: DocumentSnapshotDto, draftText = snapshot.text): void {
    store.set({
      status: "ready",
      snapshot,
      draftText,
      isDirty: draftText !== snapshot.text,
      error: null,
    });
  }

  async function refresh(
    documentSessionId: number,
    options: {
      preserveDraft?: boolean;
    } = {},
  ): Promise<{ snapshot: DocumentSnapshotDto; finishedAt: number }> {
    const snapshot = await getDocument(documentSessionId);
    const draftText = options.preserveDraft ? get(store).draftText : snapshot.text;
    setSnapshot(snapshot, draftText);
    return { snapshot, finishedAt: now() };
  }

  async function processSyncLoop(): Promise<void> {
    while (true) {
      const state = get(store);

      if (!state.snapshot || state.draftText === state.snapshot.text) {
        store.update((current) => ({
          ...current,
          status: current.snapshot ? "ready" : current.status,
        }));
        return;
      }

      const edit = deriveEditCommand(state.snapshot.text, state.draftText);

      if (!edit) {
        store.update((current) => ({
          ...current,
          status: current.snapshot ? "ready" : current.status,
          isDirty: false,
        }));
        return;
      }

      store.update((current) => ({
        ...current,
        status: "syncing",
        error: null,
      }));

      try {
        const inputReceivedAt = lastInputReceivedAt ?? now();
        const editResult = await editDocument(
          state.snapshot.document_session_id,
          state.snapshot.revision,
          edit,
        );
        const editResponseAt = now();
        const refreshStartedAt = now();
        const { snapshot, finishedAt } = await refresh(
          state.snapshot.document_session_id,
          {
            preserveDraft: true,
          },
        );
        const framePaintedAt = await waitForNextPaint();
        const telemetry = telemetryToMs(editResult.telemetry);
        const snapshotTelemetry = telemetryToMs(snapshot.telemetry);

        recordEditorPipelineSample({
          revision: snapshot.revision,
          reason: "edit",
          inputToIpcResponseMs: editResponseAt - inputReceivedAt,
          inputToPaintMs: framePaintedAt - inputReceivedAt,
          backendDocumentOperationMs: telemetry.documentOperationMs,
          backendSnapshotBuildMs: snapshotTelemetry.snapshotBuildMs,
          refreshRoundTripMs: finishedAt - refreshStartedAt,
          viewportBuildMs: finishedAt - refreshStartedAt,
          framePaintWaitMs: framePaintedAt - finishedAt,
        });

        if (get(store).draftText === snapshot.text) {
          lastInputReceivedAt = null;
        }
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);

        store.update((current) => ({
          ...current,
          status: "error",
          error: message,
        }));
        return;
      }
    }
  }

  function scheduleSync(): Promise<void> {
    if (!activeSync) {
      activeSync = processSyncLoop().finally(() => {
        activeSync = null;
      });
    }

    return activeSync;
  }

  async function waitForSync(): Promise<void> {
    if (activeSync) {
      await activeSync;
    }
  }

  return {
    subscribe: store.subscribe,

    async initialize(): Promise<void> {
      try {
        ensureTauriRuntime();
        const snapshot = await createScratchDocument();
        setSnapshot(snapshot);
      } catch (error) {
        const message =
          error instanceof TauriCommandError || error instanceof Error
            ? error.message
            : String(error);

        store.set({
          ...initialState,
          status: "error",
          error: message,
        });
      }
    },

    updateDraft(text: string, inputReceivedAt = now()): void {
      lastInputReceivedAt = inputReceivedAt;
      store.update((state) => ({
        ...state,
        draftText: text,
        isDirty: state.snapshot ? text !== state.snapshot.text : false,
        error: null,
      }));

      void scheduleSync();
    },

    async undo(): Promise<void> {
      await waitForSync();
      const state = get(store);

      if (!state.snapshot) {
        return;
      }

      store.update((current) => ({ ...current, status: "syncing", error: null }));

      try {
        const operationStartedAt = now();
        const result = await undoDocument(
          state.snapshot.document_session_id,
          state.snapshot.revision,
        );
        const refreshStartedAt = now();
        const { snapshot, finishedAt } = await refresh(
          state.snapshot.document_session_id,
        );
        const framePaintedAt = await waitForNextPaint();
        const telemetry = telemetryToMs(result.telemetry);
        const snapshotTelemetry = telemetryToMs(snapshot.telemetry);

        recordEditorPipelineSample({
          revision: snapshot.revision,
          reason: "undo",
          inputToIpcResponseMs: now() - operationStartedAt,
          inputToPaintMs: framePaintedAt - operationStartedAt,
          backendDocumentOperationMs: telemetry.documentOperationMs,
          backendSnapshotBuildMs: snapshotTelemetry.snapshotBuildMs,
          refreshRoundTripMs: finishedAt - refreshStartedAt,
          viewportBuildMs: finishedAt - refreshStartedAt,
          framePaintWaitMs: framePaintedAt - finishedAt,
        });
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);

        store.update((current) => ({
          ...current,
          status: "error",
          error: message,
        }));
      }
    },

    async redo(): Promise<void> {
      await waitForSync();
      const state = get(store);

      if (!state.snapshot) {
        return;
      }

      store.update((current) => ({ ...current, status: "syncing", error: null }));

      try {
        const operationStartedAt = now();
        const result = await redoDocument(
          state.snapshot.document_session_id,
          state.snapshot.revision,
        );
        const refreshStartedAt = now();
        const { snapshot, finishedAt } = await refresh(
          state.snapshot.document_session_id,
        );
        const framePaintedAt = await waitForNextPaint();
        const telemetry = telemetryToMs(result.telemetry);
        const snapshotTelemetry = telemetryToMs(snapshot.telemetry);

        recordEditorPipelineSample({
          revision: snapshot.revision,
          reason: "redo",
          inputToIpcResponseMs: now() - operationStartedAt,
          inputToPaintMs: framePaintedAt - operationStartedAt,
          backendDocumentOperationMs: telemetry.documentOperationMs,
          backendSnapshotBuildMs: snapshotTelemetry.snapshotBuildMs,
          refreshRoundTripMs: finishedAt - refreshStartedAt,
          viewportBuildMs: finishedAt - refreshStartedAt,
          framePaintWaitMs: framePaintedAt - finishedAt,
        });
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);

        store.update((current) => ({
          ...current,
          status: "error",
          error: message,
        }));
      }
    },
  };
}

export const editorStore = createEditorStore();
