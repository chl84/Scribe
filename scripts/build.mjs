#!/usr/bin/env node

import { spawn } from "node:child_process";
import { accessSync, constants } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, "..");
const backendDir = path.join(rootDir, "backend");
const frontendDir = path.join(rootDir, "frontend");
const tauriBinary = path.join(
  frontendDir,
  "node_modules",
  ".bin",
  process.platform === "win32" ? "tauri.cmd" : "tauri"
);

function ensureExists(target, message) {
  try {
    accessSync(target, constants.F_OK);
  } catch {
    console.error(message);
    process.exit(1);
  }
}

ensureExists(
  path.join(frontendDir, "package-lock.json"),
  "Missing frontend/package-lock.json. Run `npm run setup` from the repository root."
);
ensureExists(
  tauriBinary,
  "Frontend dependencies are not installed. Run `npm run setup` from the repository root."
);

const child = spawn(tauriBinary, ["build"], {
  cwd: backendDir,
  stdio: "inherit",
});

child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
  }

  process.exit(code ?? 1);
});

child.on("error", (error) => {
  console.error(`Failed to build Tauri: ${error.message}`);
  process.exit(1);
});
