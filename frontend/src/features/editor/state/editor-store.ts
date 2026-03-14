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
    documentId: number,
    options: {
      preserveDraft?: boolean;
    } = {},
  ): Promise<void> {
    const snapshot = await getDocument(documentId);
    const draftText = options.preserveDraft ? get(store).draftText : snapshot.text;
    setSnapshot(snapshot, draftText);
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
        await editDocument(state.snapshot.document_id, edit);
        await refresh(state.snapshot.document_id, { preserveDraft: true });
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

    updateDraft(text: string): void {
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
      await waitForSync();
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
