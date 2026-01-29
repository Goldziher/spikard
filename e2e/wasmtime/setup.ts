import { execSync, spawn, type ChildProcess } from "node:child_process";
import { existsSync } from "node:fs";
import { resolve } from "node:path";

const WASM_APP_DIR = resolve(__dirname, "app");
const WASM_BINARY = resolve(
  WASM_APP_DIR,
  "target/wasm32-wasip2/release/spikard_wasmtime_e2e.wasm"
);

let wasmtimeProcess: ChildProcess | null = null;

// Random port to avoid conflicts
const PORT = 18000 + Math.floor(Math.random() * 1000);

export async function setup() {
  // Build the WASM component
  console.log("Building WASM component...");
  execSync("cargo build --target wasm32-wasip2 --release", {
    cwd: WASM_APP_DIR,
    stdio: "inherit",
  });

  if (!existsSync(WASM_BINARY)) {
    throw new Error(`WASM binary not found at ${WASM_BINARY}`);
  }

  // Start wasmtime serve
  console.log(`Starting wasmtime serve on port ${PORT}...`);
  wasmtimeProcess = spawn(
    "wasmtime",
    ["serve", "--addr", `0.0.0.0:${PORT}`, WASM_BINARY],
    { stdio: "pipe" }
  );

  // Wait for server to be ready
  const maxWait = 30_000;
  const start = Date.now();
  while (Date.now() - start < maxWait) {
    try {
      const res = await fetch(`http://localhost:${PORT}/health`);
      if (res.ok) {
        console.log(`wasmtime serve ready on port ${PORT}`);
        // Expose port to tests via env
        process.env.WASMTIME_PORT = String(PORT);
        return;
      }
    } catch {
      // Not ready yet
    }
    await new Promise((r) => setTimeout(r, 200));
  }

  throw new Error("wasmtime serve failed to start within 30s");
}

export async function teardown() {
  if (wasmtimeProcess) {
    wasmtimeProcess.kill("SIGTERM");
    wasmtimeProcess = null;
  }
}
