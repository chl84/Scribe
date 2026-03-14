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
import { utf8ByteLength } from "../../../shared/utils/utf8";
import type { DocumentSnapshotDto } from "../../../shared/types/editor";

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
  let syncInFlight = false;
  let syncQueued = false;

  function setReady(snapshot: DocumentSnapshotDto): void {
    store.set({
      status: "ready",
      snapshot,
      draftText: snapshot.text,
      isDirty: false,
      error: null,
    });
  }

  async function refresh(documentId: number): Promise<void> {
    const snapshot = await getDocument(documentId);
    setReady(snapshot);
  }

  async function flushSync(): Promise<void> {
    const state = get(store);

    if (!state.snapshot || syncInFlight || state.draftText === state.snapshot.text) {
      return;
    }

    syncInFlight = true;
    store.update((current) => ({
      ...current,
      status: "syncing",
      error: null,
    }));

    try {
      await editDocument(state.snapshot.document_id, {
        kind: "replace",
        start: 0,
        end: utf8ByteLength(state.snapshot.text),
        text: state.draftText,
      });

      await refresh(state.snapshot.document_id);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);

      store.update((current) => ({
        ...current,
        status: "error",
        error: message,
      }));
    } finally {
      syncInFlight = false;

      if (syncQueued) {
        syncQueued = false;
        await flushSync();
      }
    }
  }

  return {
    subscribe: store.subscribe,

    async initialize(): Promise<void> {
      try {
        ensureTauriRuntime();
        const snapshot = await createScratchDocument();
        setReady(snapshot);
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

    updateDraft(text: string): void {
      store.update((state) => ({
        ...state,
        draftText: text,
        isDirty: state.snapshot ? text !== state.snapshot.text : false,
        error: null,
      }));

      if (syncInFlight) {
        syncQueued = true;
        return;
      }

      void flushSync();
    },

    async undo(): Promise<void> {
      const state = get(store);

      if (!state.snapshot) {
        return;
      }

      store.update((current) => ({ ...current, status: "syncing", error: null }));

      try {
        await undoDocument(state.snapshot.document_id);
        await refresh(state.snapshot.document_id);
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
      const state = get(store);

      if (!state.snapshot) {
        return;
      }

      store.update((current) => ({ ...current, status: "syncing", error: null }));

      try {
        await redoDocument(state.snapshot.document_id);
        await refresh(state.snapshot.document_id);
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
