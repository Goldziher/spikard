/**
 * Testing utilities for Spikard applications
 */

import { type ChildProcess, spawn } from "node:child_process";
import { once } from "node:events";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import type WebSocket from "ws";

// Native module is built into this package directory; import the napi loader directly
import {
	TestClient as NativeTestClient,
	type TestResponse as NativeTestResponse,
	type WebSocketTestConnection,
} from "../index.js";
import type { SpikardApp } from "./index";

/**
 * HTTP response from test client
 */
export type TestResponse = NativeTestResponse;

interface MultipartFile {
	name: string;
	filename?: string;
	content: string;
	contentType?: string;
}

export interface RequestOptions {
	headers?: Record<string, string>;
	json?: any;
	form?: Record<string, any>;
	multipart?: {
		fields?: Record<string, any>;
		files?: MultipartFile[];
	};
}

/**
 * WebSocket connection wrapper for testing
 */
class WebSocketConnection {
	private ws: WebSocket;
	private messageQueue: any[] = [];
	private resolveNext: ((value: any) => void) | null = null;

	constructor(ws: WebSocket) {
		this.ws = ws;

		// Queue incoming messages
		this.ws.on("message", (data: WebSocket.Data) => {
			const message = data.toString();
			if (this.resolveNext) {
				this.resolveNext(message);
				this.resolveNext = null;
			} else {
				this.messageQueue.push(message);
			}
		});
	}

	/**
	 * Send JSON data over the WebSocket
	 */
	async sendJson(data: any): Promise<void> {
		const message = JSON.stringify(data);
		return new Promise((resolve, reject) => {
			this.ws.send(message, (error) => {
				if (error) reject(error);
				else resolve();
			});
		});
	}

	/**
	 * Send text data over the WebSocket
	 */
	async sendText(text: string): Promise<void> {
		return new Promise((resolve, reject) => {
			this.ws.send(text, (error) => {
				if (error) reject(error);
				else resolve();
			});
		});
	}

	/**
	 * Receive JSON data from the WebSocket
	 */
	async receiveJson(): Promise<any> {
		const message = await this.receiveText();
		return JSON.parse(message);
	}

	/**
	 * Receive text data from the WebSocket
	 */
	async receiveText(): Promise<string> {
		if (this.messageQueue.length > 0) {
			return this.messageQueue.shift();
		}

		return new Promise((resolve) => {
			this.resolveNext = resolve;
		});
	}

	/**
	 * Close the WebSocket connection
	 */
	async close(): Promise<void> {
		return new Promise((resolve) => {
			this.ws.close();
			this.ws.once("close", () => resolve());
		});
	}
}

/**
 * Test client for making HTTP requests to Spikard applications
 *
 * Provides a high-level API for testing HTTP endpoints without
 * starting an actual server.
 *
 * @example
 * ```typescript
 * import { TestClient } from '@spikard/node';
 *
 * const app = {
 *   routes: [
 *     {
 *       method: 'GET',
 *       path: '/users/:id',
 *       handler_name: 'getUser',
 *       is_async: true
 *     }
 *   ],
 *   handlers: {
 *     getUser: async (req) => ({ id: req.params.id, name: 'Alice' })
 *   }
 * };
 *
 * const client = new TestClient(app);
 * const response = await client.get('/users/123');
 * console.log(response.json()); // { id: '123', name: 'Alice' }
 * ```
 */
export class TestClient {
	private nativeClient: NativeTestClient;
	private app: SpikardApp;
	private serverProcess: ChildProcess | null = null;
	private serverPort: number | null = null;
	private serverScriptPath: string | null = null;

	/**
	 * Create a new test client
	 *
	 * @param app - Spikard application with routes and handlers
	 */
	constructor(app: SpikardApp) {
		if (!app || !Array.isArray(app.routes)) {
			throw new Error("Invalid Spikard app: missing routes array");
		}
		this.app = app;
		const routesJson = JSON.stringify(app.routes);
		const handlersMap = app.handlers || {};
		const config = app.config || null;

		this.nativeClient = new NativeTestClient(routesJson, handlersMap, config);
	}

	/**
	 * Make a GET request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async get(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.get(path, headers || null);
	}

	/**
	 * Make a POST request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async post(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.post(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a PUT request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async put(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.put(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a DELETE request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async delete(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.delete(path, headers || null);
	}

	/**
	 * Make a PATCH request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async patch(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.patch(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a HEAD request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async head(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.head(path, headers || null);
	}

	/**
	 * Make an OPTIONS request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async options(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.options(path, this.buildHeaders(options));
	}

	/**
	 * Make a TRACE request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async trace(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.trace(path, this.buildHeaders(options));
	}

	private buildHeaders(options?: RequestOptions): Record<string, string> | null {
		if (!options?.headers || Object.keys(options.headers).length === 0) {
			return null;
		}
		return options.headers;
	}

	private buildBody(options?: RequestOptions): any | null {
		if (!options) {
			return null;
		}

		if (options.multipart) {
			return {
				__spikard_multipart__: {
					fields: options.multipart.fields ?? {},
					files: options.multipart.files ?? [],
				},
			};
		}

		if (options.form) {
			return {
				__spikard_form__: options.form,
			};
		}

		if ("json" in options) {
			return options.json ?? null;
		}

		return null;
	}

	/**
	 * Start the subprocess server for WebSocket testing
	 */
	private async startServer(): Promise<number> {
		if (this.serverProcess && this.serverPort) {
			return this.serverPort;
		}

		// Create temporary directory for server script
		const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "spikard-test-"));
		this.serverScriptPath = path.join(tmpDir, "server.cjs");

		// Generate a random port in the high range to avoid conflicts
		const port = 50000 + Math.floor(Math.random() * 10000);

		// Get absolute path to the Spikard package (use CommonJS build)
		const spikardPackagePath = path.resolve(__dirname, "../dist/index.js");

		// Write server script that starts Spikard with the app configuration (CommonJS)
		const serverScript = `
const { runServer } = require('${spikardPackagePath.replace(/\\/g, "\\\\")}');

const app = ${JSON.stringify({
			routes: this.app.routes,
			config: this.app.config || {},
		})};

// Handler functions need to be reconstructed from the serialized app
const handlers = {};
${Object.entries(this.app.handlers)
	.map(
		([name, _handler]) => `
handlers.${name} = ${this.app.handlers[name].toString()};
`,
	)
	.join("\n")}

app.handlers = handlers;

// Start server on assigned port
const config = { ...app.config, host: '127.0.0.1', port: ${port} };
(async () => {
  await runServer(app, config);
  // Keep process alive - server runs in background thread
  setInterval(() => {}, 1000);
})();
`;

		fs.writeFileSync(this.serverScriptPath, serverScript);

		// Start the server process
		// Set cwd to current directory so the subprocess can find node_modules
		this.serverProcess = spawn(process.execPath, [this.serverScriptPath], {
			stdio: ["ignore", "pipe", "pipe"],
			cwd: process.cwd(),
		});

		// Wait for server to output the ready signal
		const actualPort = await new Promise<number>((resolve, reject) => {
			let stdout = "";
			let stderr = "";

			const timeout = setTimeout(() => {
				this.serverProcess?.kill();
				reject(new Error("Server startup timeout"));
			}, 10000);

			this.serverProcess?.stdout?.on("data", (data: Buffer) => {
				stdout += data.toString();
				const match = /SPIKARD_TEST_SERVER_READY:(\d+)/.exec(stdout);
				if (match) {
					clearTimeout(timeout);
					resolve(Number.parseInt(match[1], 10));
				}
			});

			this.serverProcess?.stderr?.on("data", (data: Buffer) => {
				stderr += data.toString();
			});

			this.serverProcess?.on("error", (error: Error) => {
				clearTimeout(timeout);
				reject(error);
			});

			this.serverProcess?.on("exit", (code: number | null) => {
				if (code !== null && code !== 0) {
					clearTimeout(timeout);
					reject(new Error(`Server exited with code ${code}\nstderr: ${stderr}`));
				}
			});
		});

		this.serverPort = actualPort;
		return actualPort;
	}

	/**
	 * Stop the subprocess server
	 */
	private async stopServer(): Promise<void> {
		if (this.serverProcess) {
			this.serverProcess.kill();
			await once(this.serverProcess, "exit");
			this.serverProcess = null;
			this.serverPort = null;
		}

		if (this.serverScriptPath) {
			const tmpDir = path.dirname(this.serverScriptPath);
			try {
				fs.unlinkSync(this.serverScriptPath);
				fs.rmdirSync(tmpDir);
			} catch (_error) {
				// Ignore cleanup errors
			}
			this.serverScriptPath = null;
		}
	}

	/**
	 * Connect to a WebSocket endpoint
	 *
	 * Uses the native test client to create an in-memory WebSocket connection
	 * without requiring a subprocess server.
	 *
	 * @param path - WebSocket path
	 * @returns WebSocket test connection
	 */
	async websocketConnect(path: string): Promise<WebSocketTestConnection> {
		return this.nativeClient.websocket(path);
	}

	/**
	 * Cleanup resources when test client is done
	 */
	async cleanup(): Promise<void> {
		await this.stopServer();
	}
}
