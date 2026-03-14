<script lang="ts">
  import { onMount } from "svelte";

  import { editorStore } from "../state/editor-store";
  import { now } from "../../../shared/utils/performance";

  onMount(() => {
    void editorStore.initialize();
  });

  const editor = editorStore;

  function handleKeydown(event: KeyboardEvent): void {
    if (!(event.metaKey || event.ctrlKey)) {
      return;
    }

    if (event.key.toLowerCase() === "z" && event.shiftKey) {
      event.preventDefault();
      void editor.redo();
      return;
    }

    if (event.key.toLowerCase() === "y") {
      event.preventDefault();
      void editor.redo();
      return;
    }

    if (event.key.toLowerCase() === "z") {
      event.preventDefault();
      void editor.undo();
    }
  }
</script>

<svelte:head>
  <title>Scribe</title>
</svelte:head>

<main class="shell" aria-label="Scribe window">
  <textarea
    class="editor__surface"
    spellcheck="false"
    value={$editor.draftText}
    oninput={(event) =>
      editor.updateDraft(
        (event.currentTarget as HTMLTextAreaElement).value,
        now(),
      )}
    onkeydown={handleKeydown}
    disabled={$editor.status === "booting"}
    placeholder="Start writing."
  ></textarea>

  {#if $editor.status === "booting"}
    <div class="overlay" aria-live="polite">Loading editor…</div>
  {/if}

  {#if $editor.error}
    <div class="overlay overlay--error" aria-live="assertive">{$editor.error}</div>
  {/if}
</main>
