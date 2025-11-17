import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { describe, expect, test } from "vitest";

const ROOT = path.resolve(__dirname, "../../..");
const CLI_MANIFEST = path.join(ROOT, "crates", "spikard-cli", "Cargo.toml");

const OPENAPI_SPEC = `openapi: 3.1.0
info:
  title: Node DTO Smoke
  version: "1.0.0"
paths:
  /hello:
    post:
      operationId: sayHello
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HelloResponse"
components:
  schemas:
    HelloResponse:
      type: object
      properties:
        message:
          type: string
      required:
        - message
`;

const ASYNCAPI_SPEC = `asyncapi: "3.0.0"
info:
  title: Node AsyncAPI Smoke
  version: "1.0.0"
servers:
  ws:
    host: chat.example.com
    protocol: ws
channels:
  /chat:
    messages:
      chatMessage:
        payload:
          type: object
          properties:
            type:
              const: chatMessage
            body:
              type: string
          required:
            - type
            - body
`;

function runCli(args: string[], workingDir = ROOT) {
	const result = spawnSync("cargo", ["run", "--manifest-path", CLI_MANIFEST, "--", ...args], {
		cwd: workingDir,
		encoding: "utf-8",
	});
	if (result.error || result.status !== 0) {
		throw new Error(`CLI failed: ${result.error ?? result.stderr}`);
	}
}

describe("spikard-cli DTO generation (node)", () => {
	test("generates TypeScript handlers with Zod schemas", () => {
		const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "spikard-cli-node-"));
		const specPath = path.join(tmp, "openapi.yaml");
		const outputPath = path.join(tmp, "app.ts");
		fs.writeFileSync(specPath, OPENAPI_SPEC);

		runCli(["generate", specPath, "--lang", "typescript", "--dto", "zod", "--output", outputPath]);

		const contents = fs.readFileSync(outputPath, "utf-8");
		expect(contents).toContain('import { z } from "zod"');
		expect(contents).toContain("export const HelloResponseSchema");
	});

	test("generates AsyncAPI WebSocket handler", () => {
		const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "spikard-cli-node-async-"));
		const specPath = path.join(tmp, "asyncapi.yaml");
		const outputPath = path.join(tmp, "ws_app.ts");
		fs.writeFileSync(specPath, ASYNCAPI_SPEC);

		runCli(["generate-asyncapi", specPath, "test-app", "--lang", "typescript", "--dto", "zod", "--output", outputPath]);

		const contents = fs.readFileSync(outputPath, "utf-8");
		expect(contents).toContain("async function handleWebSocket");
	});

	test("generates AsyncAPI handler scaffolding", () => {
		const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "spikard-cli-node-handlers-"));
		const specPath = path.join(tmp, "asyncapi.yaml");
		const outputPath = path.join(tmp, "handler.ts");
		fs.writeFileSync(specPath, ASYNCAPI_SPEC);

		runCli(["generate-asyncapi", specPath, "handlers", "--lang", "typescript", "--dto", "zod", "--output", outputPath]);

		const contents = fs.readFileSync(outputPath, "utf-8");
		expect(contents).toContain("createAsyncApiHandlers");
	});
});
