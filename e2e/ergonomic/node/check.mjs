/**
 * Drives the ergonomic smoke server with real HTTP requests and asserts:
 *
 * - a valid body  -> 2xx with the typed DTO serialized back
 * - an invalid body -> 422 ProblemDetails produced by the Rust CORE (not a
 *   language-side 400), proving validation is delegated to the core.
 *
 * Exit 0 = pass. Run from the repo root with:
 *   node e2e/ergonomic/node/check.mjs
 */

import { spawn } from "child_process";
import path from "path";
import { fileURLToPath } from "url";
import fs from "fs";
import http from "node:http";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const HERE = __dirname;
const PORT = 8000;

async function post(payload) {
  return new Promise((resolve, reject) => {
    const data = JSON.stringify(payload);
    const options = {
      hostname: "127.0.0.1",
      port: PORT,
      path: "/users",
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Content-Length": data.length,
      },
    };

    const req = http.request(options, (res) => {
      let body = "";
      res.on("data", (chunk) => {
        body += chunk;
      });
      res.on("end", () => {
        resolve([res.statusCode, body]);
      });
    });

    req.on("error", reject);
    req.write(data);
    req.end();
  });
}

async function main() {
  const logPath = path.join(HERE, ".server.log");
  const log = fs.openSync(logPath, "w");

  const serverPath = path.join(HERE, "server.mjs");

  // The ergonomic App + its zod dependency live in the built node package.
  // Point NODE_PATH at the package's node_modules so the spawned server can
  // resolve `zod` and `@spikard/node/app.cjs`.
  const packageNodeModules = path.resolve(HERE, "..", "..", "..", "crates", "spikard-node", "node_modules");

  // Spawn server (stdio: inherit our log fd for both stdout and stderr).
  const proc = spawn("node", [serverPath], {
    stdio: ["ignore", log, log],
    env: { ...process.env, NODE_PATH: `${packageNodeModules}${path.delimiter}${process.env.NODE_PATH || ""}` },
  });

  try {
    // Wait for server to bind (with early-exit detection)
    let bound = false;
    for (let i = 0; i < 60; i++) {
      // Check if process exited early
      if (proc.exitCode !== null) {
        try {
          fs.closeSync(log);
        } catch {}
        const logContent = fs.readFileSync(logPath, "utf-8").slice(0, 2500);
        console.log(`FAIL: server exited early rc=${proc.exitCode}`);
        console.log(logContent);
        return 1;
      }

      try {
        await post({ name: "warmup", age: 1 });
        bound = true;
        break;
      } catch (e) {
        // Connection refused, wait and retry
        await new Promise((r) => setTimeout(r, 250));
      }
    }

    if (!bound) {
      console.log("FAIL: server never came up");
      return 1;
    }

    // Test valid request
    const [status, body] = await post({ name: "Alice", age: 30 });
    console.log(`VALID   -> ${status} ${body}`);
    if (status < 200 || status >= 300 || !body.includes("Alice")) {
      console.log("FAIL: valid request did not return the typed DTO");
      return 1;
    }

    // Test invalid request (age is string, not integer)
    const [status2, body2] = await post({ name: "Bob", age: "not-a-number" });
    console.log(`INVALID -> ${status2} ${body2}`);
    if (status2 !== 422) {
      console.log(`FAIL: invalid body expected 422 from the core, got ${status2}`);
      return 1;
    }

    console.log("ERGO SMOKE PASS (node)");
    return 0;
  } finally {
    proc.kill();
    try {
      await new Promise((r, rej) => {
        const timeout = setTimeout(() => {
          proc.kill("SIGKILL");
          rej(new Error("timeout"));
        }, 5000);
        proc.on("exit", () => {
          clearTimeout(timeout);
          r();
        });
      });
    } catch (e) {
      // Ignore
    }
    try {
      fs.closeSync(log);
    } catch {}
  }
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error("Fatal error:", err);
    process.exit(1);
  });
