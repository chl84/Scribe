import type { PerformanceTelemetryDto } from "../types/editor";

export interface EditorPipelineSample {
  revision: number;
  reason: "edit" | "undo" | "redo";
  inputToIpcResponseMs: number;
  inputToPaintMs: number;
  backendDocumentOperationMs: number | null;
  backendSnapshotBuildMs: number | null;
  refreshRoundTripMs: number;
  viewportBuildMs: number;
  framePaintWaitMs: number;
}

declare global {
  interface Window {
    __SCRIBE_PERF__?: EditorPipelineSample[];
  }
}

export function now(): number {
  return performance.now();
}

export function nanosToMs(value: number | null | undefined): number | null {
  return value == null ? null : value / 1_000_000;
}

export async function waitForNextPaint(): Promise<number> {
  return new Promise((resolve) => {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        resolve(performance.now());
      });
    });
  });
}

export function recordEditorPipelineSample(sample: EditorPipelineSample): void {
  if (typeof window === "undefined") {
    return;
  }

  const queue = window.__SCRIBE_PERF__ ?? [];
  queue.push(sample);

  if (queue.length > 50) {
    queue.shift();
  }

  window.__SCRIBE_PERF__ = queue;
  console.debug("[scribe:perf]", sample);
}

export function telemetryToMs(telemetry: PerformanceTelemetryDto | null | undefined): {
  documentOperationMs: number | null;
  snapshotBuildMs: number | null;
} {
  return {
    documentOperationMs: nanosToMs(telemetry?.document_operation_nanos),
    snapshotBuildMs: nanosToMs(telemetry?.snapshot_build_nanos),
  };
}
