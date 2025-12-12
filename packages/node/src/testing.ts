/**
 * Testing utilities for Spikard applications
 */

import fs from "node:fs/promises";
import { createRequire } from "node:module";
import path from "node:path";
import { gunzipSync, gzipSync } from "node:zlib";
import type { ServerConfig } from "./config";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { HandlerFunction, NativeHandlerFunction, SpikardApp } from "./index";
import { getStreamingHandle, isStreamingResponse } from "./streaming";
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

	async sendJson(message: unknown): Promise<void> {
		return this.send_json(message);
	}

	async sendText(text: string): Promise<void> {
		return this.send_json(text);
	}

	async receive_json(): Promise<unknown> {
		return this.queue.shift();
	}

	async receiveJson(): Promise<unknown> {
		return this.receive_json();
	}

	async receiveText(): Promise<string> {
		const value = await this.receive_json();
		return typeof value === "string" ? value : JSON.stringify(value);
	}

	async receiveBytes(): Promise<Buffer> {
		const value = await this.receive_json();
		if (Buffer.isBuffer(value)) {
			return value;
		}
		if (typeof value === "string") {
			return Buffer.from(value);
		}
		return Buffer.from(JSON.stringify(value));
	}

	async receiveMessage(): Promise<unknown> {
		return this.receive_json();
	}

	async close(): Promise<void> {}
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
		return this.decodeBody().toString("utf-8");
	}

	json<T>(): T {
		const raw = this.text();
		if (raw.length === 0) {
			return undefined as unknown as T;
		}
		try {
			return JSON.parse(raw) as T;
		} catch {
			return raw as unknown as T;
		}
	}

	bytes(): Buffer {
		return this.decodeBody();
	}

	private decodeBody(): Buffer {
		const encoding = (this.headerBag["content-encoding"] ?? "").toLowerCase();
		if (encoding === "gzip") {
			try {
				return gunzipSync(this.body);
			} catch {
				return this.body;
			}
		}
		return this.body;
	}
}

class JsNativeClient implements NativeClient {
	private readonly routes: SpikardApp["routes"];
	private readonly handlers: Record<string, NativeHandlerFunction>;
	private readonly dependencies: Record<string, unknown> | null;
	private readonly config: ServerConfig | null;
	private readonly rateLimitBuckets: Map<string, { tokens: number; resetAt: number }>;
	private readonly websocketRoutes: SpikardApp["websocketRoutes"];
	private readonly websocketHandlers: Record<string, Record<string, unknown>>;

	constructor(
		routesJson: string,
		_websocketRoutesJson: string | null,
		handlers: Record<string, NativeHandlerFunction>,
		websocketHandlers: Record<string, Record<string, unknown>>,
		dependencies: Record<string, unknown> | null,
		_lifecycleHooks: Record<string, unknown> | null,
		config: ServerConfig | null,
	) {
		this.routes = JSON.parse(routesJson) as SpikardApp["routes"];
		this.websocketRoutes = JSON.parse(_websocketRoutesJson ?? "[]") as SpikardApp["websocketRoutes"];
		this.handlers = handlers;
		this.websocketHandlers = websocketHandlers;
		this.dependencies = dependencies;
		this.config = config;
		this.rateLimitBuckets = new Map();
	}

	private matchRoute(
		method: string,
		path: string,
	): { handlerName: string; params: Record<string, string>; route: SpikardApp["routes"][number] } {
		const cleanedPath = path.split("?")[0] ?? path;
		for (const route of this.routes) {
			if (route.method.toUpperCase() !== method.toUpperCase()) {
				continue;
			}

			const params = this.extractParams(route.path, cleanedPath);
			if (params) {
				return { handlerName: route.handler_name, params, route };
			}
		}

		throw new Error(`No route matched ${method} ${path}`);
	}

	private extractParams(pattern: string, actual: string): Record<string, string> | null {
		const patternParts = pattern.split("/").filter(Boolean);
		const actualParts = actual.split("/").filter(Boolean);
		const hasTailParam = patternParts.some((part) => part.includes(":path"));
		if (!hasTailParam && patternParts.length !== actualParts.length) {
			return null;
		}
		if (hasTailParam && actualParts.length < patternParts.length - 1) {
			return null;
		}

		const params: Record<string, string> = {};
		for (let i = 0; i < patternParts.length; i += 1) {
			const patternPart = patternParts[i];
			const actualPart = actualParts[i];

			if (!patternPart || !actualPart) {
				return null;
			}

			if (patternPart.startsWith(":") || (patternPart.startsWith("{") && patternPart.endsWith("}"))) {
				const isPathTailParam = patternPart.includes(":path");
				const rawKey = patternPart.startsWith("{") ? patternPart.slice(1, -1).split(":")[0] : patternPart.slice(1);
				const key = rawKey ?? "";
				if (isPathTailParam) {
					params[key] = decodeURIComponent(actualParts.slice(i).join("/"));
					return params;
				}
				params[key] = decodeURIComponent(actualPart);
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

	private validateParams(
		route: SpikardApp["routes"][number],
		params: Record<string, string>,
	): NativeTestResponse | null {
		const schema = route.parameter_schema as
			| { properties?: Record<string, { format?: string; source?: string; type?: string }> }
			| undefined;
		if (!schema?.properties) {
			return null;
		}

		for (const [key, meta] of Object.entries(schema.properties)) {
			if (meta.source !== "path") {
				continue;
			}
			const raw = params[key];
			if (raw === undefined) {
				continue;
			}

			if (meta.format === "date" || meta.format === "date-time") {
				const parsed = new Date(raw);
				if (Number.isNaN(parsed.valueOf())) {
					return new JsTestResponse(422, {}, Buffer.from(""));
				}
				(params as Record<string, unknown>)[key] = parsed;
			}
		}

		return null;
	}

	private async invoke(
		method: string,
		path: string,
		headers: Record<string, string> | null,
		body: NativeBody,
	): Promise<NativeTestResponse> {
		const routeMatch = (() => {
			try {
				return this.matchRoute(method, path);
			} catch {
				return null;
			}
		})();

		if (!routeMatch) {
			const staticResponse = await this.serveStatic(path);
			if (staticResponse) {
				return staticResponse;
			}
			throw new Error(`No route matched ${method} ${path}`);
		}

		const { handlerName, params, route } = routeMatch;
		const handler = this.handlers[handlerName];
		if (!handler) {
			throw new Error(`Handler not found for ${handlerName}`);
		}

		if (this.config?.rateLimit && this.isRateLimited(route.path)) {
			return new JsTestResponse(429, {}, Buffer.from(""));
		}

		const validationResponse = this.validateParams(route, params);
		if (validationResponse) {
			return validationResponse;
		}

		const requestPayload = {
			method,
			path,
			params,
			query: this.buildQuery(path),
			headers: headers ?? {},
			cookies: {},
			body: this.encodeBody(body),
			dependencies: this.dependencies ?? undefined,
		};

		const result = await handler(this.safeStringify(requestPayload));
		const response = await this.toResponse(result);
		return this.applyCompression(response, headers);
	}

	private async toResponse(result: unknown): Promise<NativeTestResponse> {
		if (isStreamingResponse(result as never)) {
			const handle = getStreamingHandle(result as never);
			if (handle.kind === "js") {
				const buffers: Buffer[] = [];
				for await (const chunk of handle.iterator) {
					if (chunk === null || chunk === undefined) {
						continue;
					}
					if (Buffer.isBuffer(chunk)) {
						buffers.push(chunk);
						continue;
					}
					if (typeof chunk === "string") {
						buffers.push(Buffer.from(chunk));
						continue;
					}
					if (chunk instanceof ArrayBuffer || ArrayBuffer.isView(chunk)) {
						const view = ArrayBuffer.isView(chunk)
							? Buffer.from(chunk.buffer, chunk.byteOffset, chunk.byteLength)
							: Buffer.from(chunk as ArrayBuffer);
						buffers.push(view);
						continue;
					}
					buffers.push(Buffer.from(this.safeStringify(chunk)));
				}
				const bodyBuffer = buffers.length === 0 ? Buffer.alloc(0) : Buffer.concat(buffers);
				return new JsTestResponse(handle.init.statusCode ?? 200, handle.init.headers ?? {}, bodyBuffer);
			}
		}

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
			} catch {}
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

		const text = this.safeStringify(result);
		return new JsTestResponse(200, {}, Buffer.from(text));
	}

	private isRateLimited(routePath: string): boolean {
		if (!this.config?.rateLimit) {
			return false;
		}
		const now = Math.floor(Date.now() / 1000);
		const key = routePath;
		const existing = this.rateLimitBuckets.get(key);
		const bucket =
			existing && existing.resetAt === now ? existing : { tokens: this.config.rateLimit.burst, resetAt: now };
		if (bucket.tokens <= 0) {
			this.rateLimitBuckets.set(key, bucket);
			return true;
		}
		bucket.tokens -= 1;
		this.rateLimitBuckets.set(key, bucket);
		return false;
	}

	private async serveStatic(targetPath: string): Promise<NativeTestResponse | null> {
		const normalized = targetPath.split("?")[0] ?? targetPath;
		const staticConfig = this.config?.staticFiles ?? [];
		for (const entry of staticConfig) {
			if (!normalized.startsWith(entry.routePrefix)) {
				continue;
			}
			const relative = normalized.slice(entry.routePrefix.length);
			const resolved = relative === "/" || relative === "" ? "index.html" : relative.replace(/^\//, "");
			const filePath = path.join(entry.directory, resolved);
			try {
				const contents = await fs.readFile(filePath);
				const contentType = this.detectContentType(filePath);
				const headers: Record<string, string> = {
					"content-type": contentType,
				};
				if (entry.cacheControl) {
					headers["cache-control"] = entry.cacheControl;
				}
				const bodyBuffer = contentType.startsWith("text/")
					? Buffer.from(contents.toString("utf-8").replace(/\n$/, ""))
					: contents;
				return new JsTestResponse(200, headers, bodyBuffer);
			} catch {}
		}
		return null;
	}

	private detectContentType(filePath: string): string {
		const ext = path.extname(filePath).toLowerCase();
		switch (ext) {
			case ".txt":
				return "text/plain";
			case ".html":
			case ".htm":
				return "text/html";
			case ".json":
				return "application/json";
			case ".xml":
				return "application/xml";
			case ".csv":
				return "text/csv";
			case ".png":
				return "image/png";
			case ".jpg":
			case ".jpeg":
				return "image/jpeg";
			case ".pdf":
				return "application/pdf";
			default:
				return "application/octet-stream";
		}
	}

	private applyCompression(response: NativeTestResponse, requestHeaders: Record<string, string> | null) {
		const config = this.config?.compression;
		const acceptsGzip = (this.lookupHeader(requestHeaders, "accept-encoding") ?? "").includes("gzip");
		if (!config || !config.gzip || !acceptsGzip) {
			return response;
		}

		const rawBody = response.bytes();
		if (rawBody.length < (config.minSize ?? 0)) {
			return response;
		}

		const gzipped = gzipSync(rawBody, { level: config.quality ?? 6 });
		const headers = { ...response.headers(), "content-encoding": "gzip" };
		return new JsTestResponse(response.statusCode, headers, gzipped);
	}

	private lookupHeader(headers: Record<string, string> | null, name: string): string | undefined {
		if (!headers) {
			return undefined;
		}
		const target = name.toLowerCase();
		for (const [key, value] of Object.entries(headers)) {
			if (key.toLowerCase() === target) {
				return value;
			}
		}
		return undefined;
	}

	private safeStringify(value: unknown): string {
		return JSON.stringify(value, (_key, val) => {
			if (typeof val === "bigint") {
				return val.toString();
			}
			return val;
		});
	}

	private encodeBody(body: NativeBody): NativeBody {
		if (body === null) {
			return null;
		}

		if (typeof body === "object" && ("__spikard_multipart__" in body || "__spikard_form__" in body)) {
			return Array.from(Buffer.from(this.safeStringify(body)));
		}

		if (Buffer.isBuffer(body)) {
			return Array.from(body);
		}

		return body;
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
		const match = this.websocketRoutes?.find((route) => route.path === _path);
		if (!match) {
			throw new Error("WebSocket testing is not available in the JS fallback client");
		}
		const handlerEntry = this.websocketHandlers?.[match.handler_name];
		if (!handlerEntry) {
			throw new Error("WebSocket testing is not available in the JS fallback client");
		}
		const handler =
			handlerEntry &&
			typeof (handlerEntry as { handleMessage?: (msg: unknown) => Promise<unknown> }).handleMessage === "function"
				? (handlerEntry as { handleMessage: (msg: unknown) => Promise<unknown> }).handleMessage
				: null;
		if (!handler) {
			throw new Error("WebSocket testing is not available in the JS fallback client");
		}
		const mock = new MockWebSocketConnection(async (msg) => handler(msg));
		return mock as unknown as WebSocketTestConnection;
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

	private looksLikeStringHandler(fn: HandlerFunction | NativeHandlerFunction): boolean {
		const source = fn.toString();
		return (
			source.includes("requestJson") ||
			source.includes("request_json") ||
			source.includes("JSON.parse") ||
			source.includes("JSON.parse(")
		);
	}

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
		const handlerEntries = Object.entries(app.handlers || {});

		const handlersMap = Object.fromEntries(
			handlerEntries.map(([name, handler]) => {
				if (isNativeHandler(handler) || this.looksLikeStringHandler(handler)) {
					return [name, handler as NativeHandlerFunction];
				}
				return [name, wrapHandler(handler as HandlerFunction)];
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

		const routeMatch = this.app.routes.find((r) => r.path === path);
		if (routeMatch) {
			const handlerFn = this.app.handlers?.[routeMatch.handler_name];
			if (handlerFn) {
				const mock = new MockWebSocketConnection(async (msg) => {
					const payload = typeof msg === "string" ? msg : JSON.stringify(msg);
					const result = await (handlerFn as HandlerFunction | NativeHandlerFunction)(payload as never);
					if (typeof result === "string") {
						try {
							return JSON.parse(result);
						} catch {
							return result;
						}
					}
					return result;
				});
				return mock as unknown as WebSocketTestConnection;
			}
		}

		return this.nativeClient.websocket(path);
	}

	/**
	 * Cleanup resources when test client is done
	 */
	async cleanup(): Promise<void> {}
}
