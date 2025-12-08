/**
 * Testing utilities for Spikard applications
 */

import { createRequire } from "node:module";
import type { ServerConfig } from "./config";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { HandlerFunction, NativeHandlerFunction, SpikardApp } from "./index";
import type { JsonValue } from "./types";

interface NativeTestResponse {
	statusCode: number;
	headers(): Record<string, string>;
	text(): string;
	json<T>(): T;
	bytes(): Buffer;
}

interface WebSocketTestConnection {
	sendText(text: string): Promise<void>;
	sendJson(obj: unknown): Promise<void>;
	receiveText(): Promise<string>;
	receiveJson(): Promise<unknown>;
	receiveBytes(): Promise<Buffer>;
	receiveMessage(): Promise<unknown>;
	close(): Promise<void>;
}

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
	json?: JsonValue;
	form?: Record<string, JsonValue>;
	multipart?: {
		fields?: Record<string, JsonValue>;
		files?: MultipartFile[];
	};
}

type MultipartPayload = {
	__spikard_multipart__: {
		fields: Record<string, JsonValue>;
		files: MultipartFile[];
	};
};

type FormPayload = {
	__spikard_form__: Record<string, JsonValue>;
};

type NativeBody = MultipartPayload | FormPayload | JsonValue | null;

interface NativeClient {
	get(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	post(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	put(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	delete(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	patch(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	head(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	options(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	trace(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	websocket(path: string): Promise<WebSocketTestConnection>;
}

class MockWebSocketConnection {
	private readonly handler: (msg: unknown) => Promise<unknown>;
	private readonly queue: unknown[] = [];

	constructor(handler: (msg: unknown) => Promise<unknown>) {
		this.handler = handler;
	}

	async send_json(message: unknown): Promise<void> {
		const response = await this.handler(message);
		this.queue.push(response);
	}

	async receive_json(): Promise<unknown> {
		return this.queue.shift();
	}

	async close(): Promise<void> {
		// no-op for mock
	}
}

type NativeClientConstructor = new (
	routesJson: string,
	websocketRoutesJson: string | null,
	handlers: Record<string, NativeHandlerFunction>,
	websocketHandlers: Record<string, Record<string, unknown>>,
	dependencies: Record<string, unknown> | null,
	lifecycleHooks: Record<string, unknown> | null,
	config: ServerConfig | null,
) => NativeClient;

type NativeClientFactory = (
	routesJson: string,
	websocketRoutesJson: string | null,
	handlers: Record<string, NativeHandlerFunction>,
	websocketHandlers: Record<string, Record<string, unknown>>,
	dependencies: Record<string, unknown> | null,
	lifecycleHooks: Record<string, unknown> | null,
	config: ServerConfig | null,
) => NativeClient;

interface NativeBinding {
	TestClient: NativeClientConstructor;
}

let nativeTestClient: NativeClientConstructor | null = null;

const loadNativeTestClient = (): NativeClientConstructor | null => {
	try {
		// createRequire allows us to require CommonJS modules from ESM context
		// This is necessary to load the NAPI binding which is a .node file loaded via CommonJS
		const require = createRequire(import.meta.url);
		const binding = require("../index.js") as NativeBinding;
		return binding.TestClient;
	} catch {
		return null;
	}
};

nativeTestClient = loadNativeTestClient();
const isNativeCtor = nativeTestClient !== null;

class JsTestResponse implements NativeTestResponse {
	private readonly body: Buffer;
	private readonly headerBag: Record<string, string>;

	constructor(
		public readonly statusCode: number,
		headers: Record<string, string>,
		body: Buffer,
	) {
		this.headerBag = headers;
		this.body = body;
	}

	headers(): Record<string, string> {
		return this.headerBag;
	}

	text(): string {
		return this.body.toString("utf-8");
	}

	json<T>(): T {
		const raw = this.text();
		return raw.length === 0 ? (undefined as unknown as T) : (JSON.parse(raw) as T);
	}

	bytes(): Buffer {
		return this.body;
	}
}

class JsNativeClient implements NativeClient {
	private readonly routes: SpikardApp["routes"];
	private readonly handlers: Record<string, NativeHandlerFunction>;
	private readonly dependencies: Record<string, unknown> | null;

	constructor(
		routesJson: string,
		_websocketRoutesJson: string | null,
		handlers: Record<string, NativeHandlerFunction>,
		_websocketHandlers: Record<string, Record<string, unknown>>,
		dependencies: Record<string, unknown> | null,
		_lifecycleHooks: Record<string, unknown> | null,
		_dependencies: ServerConfig | null,
	) {
		this.routes = JSON.parse(routesJson) as SpikardApp["routes"];
		this.handlers = handlers;
		this.dependencies = dependencies;
	}

	private matchRoute(method: string, path: string): { handlerName: string; params: Record<string, string> } {
		const cleanedPath = path.split("?")[0] ?? path;
		for (const route of this.routes) {
			if (route.method.toUpperCase() !== method.toUpperCase()) {
				continue;
			}

			const params = this.extractParams(route.path, cleanedPath);
			if (params) {
				return { handlerName: route.handler_name, params };
			}
		}

		throw new Error(`No route matched ${method} ${path}`);
	}

	private extractParams(pattern: string, actual: string): Record<string, string> | null {
		const patternParts = pattern.split("/").filter(Boolean);
		const actualParts = actual.split("/").filter(Boolean);
		if (patternParts.length !== actualParts.length) {
			return null;
		}

		const params: Record<string, string> = {};
		for (let i = 0; i < patternParts.length; i += 1) {
			const patternPart = patternParts[i];
			const actualPart = actualParts[i];

			if (!patternPart || !actualPart) {
				return null;
			}

			if (patternPart.startsWith(":")) {
				params[patternPart.slice(1)] = decodeURIComponent(actualPart);
				continue;
			}

			if (patternPart !== actualPart) {
				return null;
			}
		}

		return params;
	}

	private buildQuery(path: string): Record<string, string> {
		const query: Record<string, string> = {};
		const url = new URL(path, "http://localhost");
		url.searchParams.forEach((value, key) => {
			if (!(key in query)) {
				query[key] = value;
			}
		});
		return query;
	}

	private async invoke(
		method: string,
		path: string,
		headers: Record<string, string> | null,
		body: NativeBody,
	): Promise<NativeTestResponse> {
		const { handlerName, params } = this.matchRoute(method, path);
		const handler = this.handlers[handlerName];
		if (!handler) {
			throw new Error(`Handler not found for ${handlerName}`);
		}

		const requestPayload = {
			method,
			path,
			params,
			query: this.buildQuery(path),
			headers: headers ?? {},
			cookies: {},
			body: body === null ? null : Array.from(Buffer.from(JSON.stringify(body))),
			dependencies: this.dependencies ?? undefined,
		};

		const result = await handler(JSON.stringify(requestPayload));
		return this.toResponse(result);
	}

	private toResponse(result: unknown): NativeTestResponse {
		if (typeof result === "string") {
			try {
				const parsed = JSON.parse(result) as {
					status?: number;
					statusCode?: number;
					headers?: Record<string, string>;
					body?: unknown;
				};
				if (
					parsed &&
					typeof parsed === "object" &&
					("status" in parsed || "statusCode" in parsed || "body" in parsed)
				) {
					const statusCode = parsed.status ?? parsed.statusCode ?? 200;
					const textBody =
						typeof parsed.body === "string" || parsed.body === undefined
							? (parsed.body ?? "")
							: JSON.stringify(parsed.body);
					return new JsTestResponse(statusCode, parsed.headers ?? {}, Buffer.from(textBody));
				}
			} catch {
				// fall through to treat as plain text
			}
			return new JsTestResponse(200, {}, Buffer.from(result));
		}

		if (
			result &&
			typeof result === "object" &&
			("status" in (result as Record<string, unknown>) || "statusCode" in (result as Record<string, unknown>))
		) {
			const payload = result as {
				status?: number;
				statusCode?: number;
				headers?: Record<string, string>;
				body?: unknown;
			};
			const statusCode = payload.status ?? payload.statusCode ?? 200;
			const textBody =
				typeof payload.body === "string" || payload.body === undefined
					? (payload.body ?? "")
					: JSON.stringify(payload.body);
			return new JsTestResponse(statusCode, payload.headers ?? {}, Buffer.from(textBody));
		}

		const text = JSON.stringify(result);
		return new JsTestResponse(200, {}, Buffer.from(text));
	}

	async get(path: string, headers: Record<string, string> | null): Promise<NativeTestResponse> {
		return this.invoke("GET", path, headers, null);
	}

	async post(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<NativeTestResponse> {
		return this.invoke("POST", path, headers, body);
	}

	async put(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<NativeTestResponse> {
		return this.invoke("PUT", path, headers, body);
	}

	async delete(path: string, headers: Record<string, string> | null): Promise<NativeTestResponse> {
		return this.invoke("DELETE", path, headers, null);
	}

	async patch(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<NativeTestResponse> {
		return this.invoke("PATCH", path, headers, body);
	}

	async head(path: string, headers: Record<string, string> | null): Promise<NativeTestResponse> {
		return this.invoke("HEAD", path, headers, null);
	}

	async options(path: string, headers: Record<string, string> | null): Promise<NativeTestResponse> {
		return this.invoke("OPTIONS", path, headers, null);
	}

	async trace(path: string, headers: Record<string, string> | null): Promise<NativeTestResponse> {
		return this.invoke("TRACE", path, headers, null);
	}

	async websocket(_path: string): Promise<WebSocketTestConnection> {
		throw new Error("WebSocket testing is not available in the JS fallback client");
	}
}

const defaultNativeClientFactory: NativeClientFactory = (
	routesJson,
	websocketRoutesJson,
	handlers,
	websocketHandlers,
	dependencies,
	lifecycleHooks,
	config,
) => {
	if (isNativeCtor && nativeTestClient) {
		return new nativeTestClient(
			routesJson,
			websocketRoutesJson,
			handlers,
			websocketHandlers,
			dependencies,
			lifecycleHooks,
			config,
		);
	}

	return new JsNativeClient(
		routesJson,
		websocketRoutesJson,
		handlers,
		websocketHandlers,
		dependencies,
		lifecycleHooks,
		config,
	);
};

let nativeClientFactory: NativeClientFactory = defaultNativeClientFactory;

export const __setNativeClientFactory = (factory?: NativeClientFactory): void => {
	nativeClientFactory = factory ?? defaultNativeClientFactory;
};

/**
 * Test client for making HTTP requests to Spikard applications
 *
 * Provides a high-level API for testing HTTP endpoints without
 * starting an actual server.
 *
 * @example
 * ```typescript
 * import { TestClient } from 'spikard';
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
	readonly app: SpikardApp;
	private nativeClient: NativeClient;

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
		const websocketRoutesJson = JSON.stringify(app.websocketRoutes ?? []);
		const handlersMap = Object.fromEntries(
			Object.entries(app.handlers || {}).map(([name, handler]) => {
				const nativeHandler = isNativeHandler(handler) ? handler : wrapHandler(handler as HandlerFunction);
				return [name, nativeHandler];
			}),
		);
		const websocketHandlersMap = app.websocketHandlers || {};
		const config = app.config ?? null;
		const dependencies = (app as SpikardApp & { dependencies?: Record<string, unknown> }).dependencies ?? null;
		const lifecycleHooks = (app as { getLifecycleHooks?: () => Record<string, unknown> }).getLifecycleHooks?.() ?? null;

		this.nativeClient = nativeClientFactory(
			routesJson,
			websocketRoutesJson,
			handlersMap,
			websocketHandlersMap,
			dependencies,
			lifecycleHooks,
			config,
		);
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

	private buildBody(options?: RequestOptions): NativeBody {
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
	 * Connect to a WebSocket endpoint
	 *
	 * Uses the native test client to create an in-memory WebSocket connection.
	 *
	 * @param path - WebSocket path
	 * @returns WebSocket test connection
	 */
	async websocketConnect(path: string): Promise<WebSocketTestConnection> {
		const handlerName = this.app.websocketRoutes?.find((r) => r.path === path)?.handler_name;
		const handlerEntry = handlerName ? this.app.websocketHandlers?.[handlerName] : undefined;
		const handler =
			handlerEntry && typeof handlerEntry.handleMessage === "function" ? handlerEntry.handleMessage : null;

		if (handler) {
			const mock = new MockWebSocketConnection(async (msg) => handler(msg));
			return mock as unknown as WebSocketTestConnection;
		}

		return this.nativeClient.websocket(path);
	}

	/**
	 * Cleanup resources when test client is done
	 */
	async cleanup(): Promise<void> {}
}
