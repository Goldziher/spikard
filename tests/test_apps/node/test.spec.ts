import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { spawn, type ChildProcess } from "node:child_process";
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

describe("Spikard Node.js Test App", () => {
	let serverProcess: ChildProcess;
	let baseUrl: string;
	let port: number;

	beforeAll(async () => {
		// Find an available port by trying a random high port
		port = 8000 + Math.floor(Math.random() * 1000);
		baseUrl = `http://127.0.0.1:${port}`;

		// Spawn the server process using tsx
		serverProcess = spawn("npx", ["tsx", "app.ts", port.toString()], {
			cwd: __dirname,
			stdio: ["ignore", "pipe", "pipe"],
		});

		// Wait for server to start
		await new Promise<void>((resolve, reject) => {
			const timeout = setTimeout(() => {
				reject(new Error("Server failed to start within 10 seconds"));
			}, 10000);

			serverProcess.stdout?.on("data", (data) => {
				const output = data.toString();
				if (output.includes("Starting Spikard")) {
					clearTimeout(timeout);
					// Give it a moment to fully bind
					setTimeout(resolve, 500);
				}
			});

			serverProcess.stderr?.on("data", (data) => {
				console.error("Server error:", data.toString());
			});

			serverProcess.on("error", (err) => {
				clearTimeout(timeout);
				reject(err);
			});

			serverProcess.on("exit", (code) => {
				if (code !== null && code !== 0) {
					clearTimeout(timeout);
					reject(new Error(`Server exited with code ${code}`));
				}
			});
		});
	});

	afterAll(async () => {
		if (serverProcess) {
			serverProcess.kill("SIGTERM");
			await new Promise<void>((resolve) => {
				serverProcess.on("exit", () => resolve());
				setTimeout(() => {
					serverProcess.kill("SIGKILL");
					resolve();
				}, 5000);
			});
		}
	});

	it("should use the correct package version", () => {
		const pkg = JSON.parse(readFileSync(join(__dirname, "package.json"), "utf-8"));
		expect(pkg.dependencies["@spikard/node"]).toBe("0.7.2");
	});

	it("should respond to health check", async () => {
		const res = await fetch(`${baseUrl}/health`);
		expect(res.status).toBe(200);
		const data = await res.json();
		expect(data).toEqual({ status: "ok" });
	});

	it("should handle query parameters", async () => {
		const res = await fetch(`${baseUrl}/query?name=Alice&age=30`);
		expect(res.status).toBe(200);
		const data = await res.json();
		expect(data).toEqual({ name: "Alice", age: 30 });
	});

	it("should echo JSON requests", async () => {
		const payload = { message: "Hello from Node.js!" };
		const res = await fetch(`${baseUrl}/echo`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify(payload),
		});
		expect(res.status).toBe(200);
		const data = await res.json();
		expect(data.received).toEqual(payload);
		expect(data.method).toBe("POST");
	});

	it("should extract path parameters", async () => {
		const res = await fetch(`${baseUrl}/users/42`);
		expect(res.status).toBe(200);
		const data = await res.json();
		expect(data.userId).toBe("42");
		expect(data.type).toBe("string");
	});
});
