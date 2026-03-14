import { invoke, isTauri } from "@tauri-apps/api/core";

export class TauriCommandError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "TauriCommandError";
  }
}

export function ensureTauriRuntime(): void {
  if (!isTauri()) {
    throw new TauriCommandError(
      "The frontend must run inside a Tauri window to reach the backend.",
    );
  }
}

export async function invokeCommand<TResponse>(
  command: string,
  args?: Record<string, unknown>,
): Promise<TResponse> {
  try {
    return await invoke<TResponse>(command, args);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new TauriCommandError(message);
  }
}
