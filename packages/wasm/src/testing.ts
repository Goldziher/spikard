import type { LifecycleHookFunction, LifecycleHooks } from "./app";
import type { StaticFilesConfig, StaticManifestEntry } from "./config";
import type { HandlerFunction, SpikardApp } from "./index";
import type { Request } from "./request";
import { isStreamingResponse } from "./streaming";
import type {
	AbortSignalLike,
	BinaryLike,
	HandlerBody,
	HandlerContext,
	HandlerPayload,
	JsonValue,
	StructuredHandlerResponse,
	WebSocketHandler,
	WebSocketHandlerLike,
	WebSocketServerSocket,
} from "./types";
import { gunzipSync } from "fflate";

type HeaderMap = Record<string, string>;
type HeaderInput = HeaderMap | Map<string, string> | null | undefined;

const globalAny = globalThis as unknown as { Buffer?: unknown };

export interface MultipartFile {
	name: string;
	filename?: string;
	content: string;
	contentType?: string;
}

export interface RequestOptions {
	headers?: HeaderMap;
	json?: JsonValue;
	form?: Record<string, JsonValue> | string;
	formRaw?: string;
	multipart?: {
		fields?: Record<string, JsonValue>;
		files?: MultipartFile[];
	};
	binary?: string;
}

type NativeRequestOptions = {
	headers?: HeaderMap;
	json?: JsonValue;
	form?: Record<string, JsonValue>;
	formRaw?: string;
	multipart?: {
		fields?: Record<string, JsonValue>;
		files?: MultipartFile[];
	};
	binary?: string;
};

interface NativeClient {
	get(path: string, headers: HeaderMap | null): Promise<NativeSnapshot>;
	delete(path: string, headers: HeaderMap | null): Promise<NativeSnapshot>;
	head(path: string, headers: HeaderMap | null): Promise<NativeSnapshot>;
	options(path: string, headers: HeaderMap | null): Promise<NativeSnapshot>;
	trace(path: string, headers: HeaderMap | null): Promise<NativeSnapshot>;
	post(path: string, options: NativeRequestOptions | null): Promise<NativeSnapshot>;
	put(path: string, options: NativeRequestOptions | null): Promise<NativeSnapshot>;
	patch(path: string, options: NativeRequestOptions | null): Promise<NativeSnapshot>;
	handle_request(requestJson: string): Promise<NativeSnapshot>;
}

type NativeLifecycleHookResult =
	| { type: "request"; payload: InternalRequestPayload }
	| { type: "response"; payload: StructuredHandlerResponse };

type NativeLifecycleHooksPayload = {
	onRequest: Array<(payload: InternalRequestPayload) => Promise<NativeLifecycleHookResult>>;
	preValidation: Array<(payload: InternalRequestPayload) => Promise<NativeLifecycleHookResult>>;
	preHandler: Array<(payload: InternalRequestPayload) => Promise<NativeLifecycleHookResult>>;
	onResponse: Array<(payload: StructuredHandlerResponse) => Promise<NativeLifecycleHookResult>>;
	onError: Array<(payload: StructuredHandlerResponse) => Promise<NativeLifecycleHookResult>>;
};

type NativeClientFactory = (
	routesJson: string,
	handlers: Record<string, HandlerFunction>,
	config: string | null,
	lifecycleHooks: NativeLifecycleHooksPayload | null,
	dependencies: Record<string, unknown> | null,
) => Promise<NativeClient>;

type NativeTestClientConstructor = new (
	routesJson: string,
	handlers: Record<string, HandlerFunction>,
	config: string | null,
	lifecycleHooks: NativeLifecycleHooksPayload | null,
	dependencies: Record<string, unknown> | null,
) => NativeClient;

type WasmBindings = {
	default?: (module?: unknown) => Promise<unknown>;
	init: () => unknown;
	TestClient: NativeTestClientConstructor;
};

let wasmBindingsPromise: Promise<WasmBindings> | null = null;

async function loadWasmBindings(): Promise<WasmBindings> {
	if (wasmBindingsPromise) {
		return wasmBindingsPromise;
	}
	wasmBindingsPromise = (async () => {
		const runtimePath = "../runtime/spikard_wasm.js";
		const webFallback = "../../../crates/spikard-wasm/dist-web/" + "spikard_wasm.js";
		const nodeFallback = "../../../crates/spikard-wasm/dist-node/" + "spikard_wasm.js";
		const preferNode = isNodeLikeEnvironment();

		const candidates = preferNode ? [nodeFallback, runtimePath, webFallback] : [runtimePath, webFallback, nodeFallback];

		for (const candidate of candidates) {
			try {
				return (await import(candidate)) as unknown as WasmBindings;
			} catch {}
		}

		throw new Error("Failed to load WASM bindings (runtime, dist-web, dist-node).");
	})();
	return wasmBindingsPromise;
}

const defaultNativeClientFactory: NativeClientFactory = async (
	routesJson,
	handlers,
	config,
	lifecycleHooks,
	dependencies,
) => {
	const bindings = await loadWasmBindings();
	if (typeof bindings.default === "function") {
		await bindings.default();
	}
	await Promise.resolve(bindings.init());
	return new bindings.TestClient(routesJson, handlers, config, lifecycleHooks, dependencies);
};

let nativeClientFactory: NativeClientFactory = defaultNativeClientFactory;

export function __setNativeClientFactory(factory?: NativeClientFactory): void {
	nativeClientFactory = factory ?? defaultNativeClientFactory;
}

interface NativeSnapshot {
	status: number;
	headers: HeaderMap;
	body: Uint8Array;
}

const textDecoder = new TextDecoder();
const textEncoder = new TextEncoder();
const RAW_REQUEST_KEY = "__spikard_raw_request__";
const ABORT_SIGNAL_KEY = "__spikard_abort_signal__";
const EMPTY_HOOKS: LifecycleHooks = {
	onRequest: [],
	preValidation: [],
	preHandler: [],
	onResponse: [],
	onError: [],
};

export type GunzipImplementation = (data: Uint8Array) => Uint8Array;

let gunzipImplementation: GunzipImplementation | null = null;

export function __setGunzipImplementation(fn: GunzipImplementation | null): void {
	gunzipImplementation = fn;
}

export class TestResponse {
	private readonly bodyBytes: Uint8Array;
	private decodedBody: Uint8Array | null = null;

	constructor(
		private readonly status: number,
		private readonly headersMap: HeaderMap,
		body: Uint8Array | ArrayBuffer | ArrayLike<number>,
	) {
		this.bodyBytes = toUint8Array(body);
	}

	get statusCode(): number {
		return this.status;
	}

	headers(): HeaderMap {
		return { ...this.headersMap };
	}

	text(): string {
		return textDecoder.decode(this.getDecodedBody());
	}

	json(): JsonValue | string | null {
		const body = this.getDecodedBody();
		if (body.length === 0) {
			return null;
		}
		const textValue = textDecoder.decode(body);
		try {
			return JSON.parse(textValue) as JsonValue;
		} catch {
			return textValue;
		}
	}

	bytes(): Uint8Array {
		const decoded = this.getDecodedBody();
		const bufferCtor = globalAny.Buffer as { from: (data: Uint8Array) => Uint8Array } | undefined;
		if (bufferCtor) {
			return bufferCtor.from(decoded);
		}
		return decoded.slice();
	}
	raw(): Uint8Array {
		return this.bodyBytes.slice();
	}

	private getDecodedBody(): Uint8Array {
		if (this.decodedBody) {
			return this.decodedBody;
		}
		const encoding = this.getHeaderValue("content-encoding");
		if (encoding && encoding.toLowerCase() === "gzip") {
			this.decodedBody = gunzipBytes(this.bodyBytes);
			return this.decodedBody;
		}
		this.decodedBody = this.bodyBytes;
		return this.decodedBody;
	}

	private getHeaderValue(name: string): string | undefined {
		for (const [key, value] of Object.entries(this.headersMap)) {
			if (key.toLowerCase() === name.toLowerCase()) {
				return value;
			}
		}
		return undefined;
	}
}

export class TestClient {
	private readonly routes: SpikardApp["routes"];
	private readonly websocketRoutes: SpikardApp["websocketRoutes"];
	public readonly websocketHandlers: Record<string, WebSocketHandlerLike>;
	private readonly nativeClientPromise: Promise<NativeClient>;

	constructor(app: SpikardApp) {
		if (!app || !Array.isArray(app.routes)) {
			throw new Error("Invalid Spikard app: missing routes");
		}
		this.routes = app.routes;
		this.websocketRoutes = app.websocketRoutes ?? [];
		const httpHandlers = app.handlers ?? {};
		this.websocketHandlers = app.websocketHandlers ?? httpHandlers;
		const routesJson = JSON.stringify(app.routes);
		const lifecycleHooks = normalizeLifecycleHooks(app.lifecycleHooks);
		const wrappedHandlers = wrapHandlers(httpHandlers);
		const dependencies = (app as SpikardApp & { dependencies?: Record<string, unknown> }).dependencies ?? null;
		const nativeLifecycleHooks = createNativeLifecycleHooks(lifecycleHooks);

		this.nativeClientPromise = (async () => {
			const resolvedApp = await withStaticManifest(app);
			const configString =
				resolvedApp.config && Object.keys(resolvedApp.config).length > 0 ? JSON.stringify(resolvedApp.config) : null;
			return nativeClientFactory(routesJson, wrappedHandlers, configString, nativeLifecycleHooks, dependencies);
		})();
	}

	async get(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await this.dispatchWithHeaders(native, "GET", path, headers);
		return this.responseFromNative(snapshot);
	}

	async delete(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await this.dispatchWithHeaders(native, "DELETE", path, headers);
		return this.responseFromNative(snapshot);
	}

	async head(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await this.dispatchWithHeaders(native, "HEAD", path, headers);
		return this.responseFromNative(snapshot);
	}

	async options(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await this.dispatchWithHeaders(native, "OPTIONS", path, headers);
		return this.responseFromNative(snapshot);
	}

	async trace(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await this.dispatchWithHeaders(native, "TRACE", path, headers);
		return this.responseFromNative(snapshot);
	}

	async post(path: string, options?: RequestOptions): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.post(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async put(path: string, options?: RequestOptions): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.put(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async patch(path: string, options?: RequestOptions): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.patch(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async websocketConnect(path: string): Promise<WebSocketTestConnection> {
		await this.nativeClientPromise;
		const route = this.findWebSocketRoute(path);
		if (!route) {
			throw new Error(`WebSocket route not found for ${path}`);
		}
		const handler = this.websocketHandlers[route.metadata.handler_name];
		if (!handler) {
			throw new Error(`Handler ${route.metadata.handler_name} not registered`);
		}
		return WebSocketTestConnection.connect(handler);
	}

	private responseFromNative(snapshot: NativeSnapshot) {
		const rawHeaders = normalizeRecord(snapshot.headers);
		const normalizedHeaders = lowerCaseHeaderKeys(rawHeaders);
		const bodyBytes = toUint8Array(snapshot.body);
		const unwrapped = tryUnwrapStructuredSnapshot(bodyBytes);
		if (unwrapped) {
			const flattened = flattenStructuredResponse(unwrapped);
			const status = flattened.status ?? flattened.statusCode ?? snapshot.status;
			const headers = lowerCaseHeaderKeys({
				...normalizeRecord(flattened.headers ?? {}),
				...normalizedHeaders,
			});
			const body = flattened.body ?? null;
			return new TestResponse(status, headers, encodeBodyBytes(body));
		}
		return new TestResponse(snapshot.status, normalizedHeaders, bodyBytes);
	}

	private dispatchWithHeaders(
		native: NativeClient,
		method: string,
		path: string,
		headers?: Record<string, string>,
	): Promise<NativeSnapshot> {
		const normalizedHeaders = normalizeHeaderInput(headers);
		if (!normalizedHeaders) {
			switch (method) {
				case "GET":
					return native.get(path, null);
				case "DELETE":
					return native.delete(path, null);
				case "HEAD":
					return native.head(path, null);
				case "OPTIONS":
					return native.options(path, null);
				case "TRACE":
					return native.trace(path, null);
				default:
					return native.get(path, null);
			}
		}
		const maybeHandleRequest = (native as unknown as { handle_request?: unknown }).handle_request;
		if (typeof maybeHandleRequest === "function") {
			return native.handle_request(JSON.stringify({ method, path, headers: normalizedHeaders }));
		}
		switch (method) {
			case "GET":
				return native.get(path, normalizedHeaders);
			case "DELETE":
				return native.delete(path, normalizedHeaders);
			case "HEAD":
				return native.head(path, normalizedHeaders);
			case "OPTIONS":
				return native.options(path, normalizedHeaders);
			case "TRACE":
				return native.trace(path, normalizedHeaders);
			default:
				return native.get(path, normalizedHeaders);
		}
	}

	private buildNativeOptions(options?: RequestOptions): NativeRequestOptions | null {
		if (!options) {
			return null;
		}
		const native: NativeRequestOptions = {};
		const headers = this.buildHeaders(options);
		if (headers) {
			native.headers = headers;
		}
		if (options.multipart) {
			native.multipart = {
				fields: options.multipart.fields ?? {},
				files: options.multipart.files ?? [],
			};
			return native;
		}
		if (options.form && typeof options.form === "object") {
			native.form = options.form;
			return native;
		}
		if (typeof options.form === "string") {
			native.formRaw = options.form;
			return native;
		}
		if (options.formRaw) {
			native.formRaw = options.formRaw;
			return native;
		}
		if ("json" in options) {
			native.json = options.json == null ? null : normalizeJsonValue(options.json);
		}
		if (options.binary) {
			native.binary = options.binary;
		}
		return Object.keys(native).length === 0 ? null : native;
	}

	private buildHeaders(options?: RequestOptions): HeaderMap | null {
		if (!options?.headers || Object.keys(options.headers).length === 0) {
			return null;
		}
		return normalizeHeaderInput(options.headers);
	}

	private findRoute(
		method: string,
		targetPath: string,
	): { metadata: SpikardApp["routes"][number]; params: Record<string, string> } | undefined {
		for (const metadata of this.routes) {
			if (!methodsMatch(metadata.method, method)) {
				continue;
			}
			const params = matchPath(metadata.path, targetPath);
			if (params) {
				return { metadata, params };
			}
		}
		return undefined;
	}

	private findWebSocketRoute(
		targetPath: string,
	): { metadata: SpikardApp["routes"][number]; params: Record<string, string> } | undefined {
		for (const metadata of this.websocketRoutes ?? []) {
			const params = matchPath(metadata.path, targetPath);
			if (params) {
				return { metadata, params };
			}
		}
		return this.findRoute("GET", targetPath);
	}
}

function normalizeHeaderInput(headers?: Record<string, string> | null): HeaderMap | null {
	if (!headers) {
		return null;
	}
	const normalized: HeaderMap = {};
	for (const [key, value] of Object.entries(headers)) {
		normalized[key] = String(value);
	}
	return normalized;
}

async function withStaticManifest(app: SpikardApp): Promise<SpikardApp> {
	const config = app.config;
	if (!config || !config.staticFiles || config.staticFiles.length === 0) {
		return app;
	}
	if (Array.isArray(config.__wasmStaticManifest) && config.__wasmStaticManifest.length > 0) {
		return app;
	}

	const manifest = await buildStaticManifest(config.staticFiles);
	if (manifest.length === 0) {
		return app;
	}
	return {
		...app,
		config: {
			...config,
			__wasmStaticManifest: manifest,
		},
	};
}

function isNodeLikeEnvironment(): boolean {
	const processValue = (globalThis as Record<string, unknown>)["process"];
	if (!processValue || typeof processValue !== "object") {
		return false;
	}
	const versionsValue = (processValue as Record<string, unknown>)["versions"];
	if (!versionsValue || typeof versionsValue !== "object") {
		return false;
	}
	return typeof (versionsValue as Record<string, unknown>)["node"] === "string";
}

function hasDeno(): boolean {
	return typeof (globalThis as Record<string, unknown>)["Deno"] === "object";
}

function normalizeRoute(route: string): string {
	let normalized = route.replaceAll("\\", "/");
	while (normalized.includes("//")) {
		normalized = normalized.replaceAll("//", "/");
	}
	if (!normalized.startsWith("/")) {
		normalized = `/${normalized}`;
	}
	return normalized;
}

function contentTypeForPath(path: string): string {
	const lower = path.toLowerCase();
	if (lower.endsWith(".html")) {
		return "text/html";
	}
	if (lower.endsWith(".txt")) {
		return "text/plain";
	}
	if (lower.endsWith(".css")) {
		return "text/css";
	}
	if (lower.endsWith(".js") || lower.endsWith(".mjs")) {
		return "application/javascript";
	}
	if (lower.endsWith(".json")) {
		return "application/json";
	}
	if (lower.endsWith(".svg")) {
		return "image/svg+xml";
	}
	if (lower.endsWith(".png")) {
		return "image/png";
	}
	if (lower.endsWith(".jpg") || lower.endsWith(".jpeg")) {
		return "image/jpeg";
	}
	return "application/octet-stream";
}

function buildStaticHeaders(filePath: string, cacheControl?: string | null): Record<string, string> {
	const headers: Record<string, string> = {
		"content-type": contentTypeForPath(filePath),
	};
	if (cacheControl) {
		headers["cache-control"] = cacheControl;
	}
	return headers;
}

async function buildStaticManifest(configs: StaticFilesConfig[]): Promise<StaticManifestEntry[]> {
	if (isNodeLikeEnvironment()) {
		const fs = await import("node:fs");
		const path = await import("node:path");
		const manifest: StaticManifestEntry[] = [];
		for (const config of configs) {
			if (!config.directory || !config.routePrefix) {
				continue;
			}
			if (!fs.existsSync(config.directory)) {
				continue;
			}

			const stack: string[] = [config.directory];
			while (stack.length > 0) {
				const current = stack.pop();
				if (!current) {
					continue;
				}
				const stats = fs.statSync(current);
				if (stats.isDirectory()) {
					for (const child of fs.readdirSync(current)) {
						stack.push(path.join(current, child));
					}
					continue;
				}

				const relative = path.relative(config.directory, current).split(path.sep).join("/");
				let prefix = config.routePrefix;
				while (prefix.endsWith("/")) {
					prefix = prefix.slice(0, -1);
				}
				const route = normalizeRoute(`${prefix}/${relative}`);
				const headers = buildStaticHeaders(current, config.cacheControl ?? null);
				const body = bufferToBase64(new Uint8Array(fs.readFileSync(current)));
				manifest.push({ route, headers, body });
			}

			if (config.indexFile ?? true) {
				const indexPath = path.join(config.directory, "index.html");
				if (fs.existsSync(indexPath)) {
					const headers = buildStaticHeaders(indexPath, config.cacheControl ?? null);
					const body = bufferToBase64(new Uint8Array(fs.readFileSync(indexPath)));
					const prefix = normalizeRoute(config.routePrefix);
					manifest.push({ route: prefix, headers: { ...headers }, body });
					if (!prefix.endsWith("/")) {
						manifest.push({ route: `${prefix}/`, headers: { ...headers }, body });
					}
				}
			}
		}
		return manifest;
	}

	if (hasDeno()) {
		const deno = (globalThis as Record<string, unknown>)["Deno"] as {
			readDir: (path: string) => AsyncIterable<{ name: string; isFile: boolean; isDirectory: boolean }>;
			readFile: (path: string) => Promise<Uint8Array>;
		};

		const manifest: StaticManifestEntry[] = [];
		for (const config of configs) {
			if (!config.directory || !config.routePrefix) {
				continue;
			}
			const root = config.directory.replaceAll("\\", "/").replace(/\/+$/, "");

			const stack: string[] = [root];
			while (stack.length > 0) {
				const current = stack.pop();
				if (!current) {
					continue;
				}
				for await (const entry of deno.readDir(current)) {
					const entryPath = `${current}/${entry.name}`;
					if (entry.isDirectory) {
						stack.push(entryPath);
						continue;
					}
					if (!entry.isFile) {
						continue;
					}
					const relative = entryPath.startsWith(`${root}/`) ? entryPath.slice(root.length + 1) : entry.name;
					let prefix = config.routePrefix;
					while (prefix.endsWith("/")) {
						prefix = prefix.slice(0, -1);
					}
					const route = normalizeRoute(`${prefix}/${relative}`);
					const headers = buildStaticHeaders(entryPath, config.cacheControl ?? null);
					const body = bufferToBase64(await deno.readFile(entryPath));
					manifest.push({ route, headers, body });
				}
			}

			if (config.indexFile ?? true) {
				const indexPath = `${root}/index.html`;
				try {
					const bytes = await deno.readFile(indexPath);
					const headers = buildStaticHeaders(indexPath, config.cacheControl ?? null);
					const body = bufferToBase64(bytes);
					const prefix = normalizeRoute(config.routePrefix);
					manifest.push({ route: prefix, headers: { ...headers }, body });
					if (!prefix.endsWith("/")) {
						manifest.push({ route: `${prefix}/`, headers: { ...headers }, body });
					}
				} catch {}
			}
		}
		return manifest;
	}

	return [];
}

type WebSocketQueuedMessage =
	| { kind: "json"; payload: JsonValue }
	| { kind: "text"; payload: string }
	| { kind: "binary"; payload: Uint8Array };

class WebSocketServerSocketImpl implements WebSocketServerSocket {
	private closed = false;

	constructor(
		private readonly enqueue: (message: WebSocketQueuedMessage) => void,
		private readonly onClose: (code?: number, reason?: string) => Promise<void>,
	) {}

	async sendText(message: string): Promise<void> {
		this.ensureOpen();
		this.enqueue({ kind: "text", payload: message });
	}

	async sendJson(payload: JsonValue): Promise<void> {
		this.ensureOpen();
		this.enqueue({ kind: "json", payload });
	}

	async sendBytes(payload: BinaryLike): Promise<void> {
		this.ensureOpen();
		this.enqueue({ kind: "binary", payload: toUint8Array(payload) });
	}

	async close(code?: number, reason?: string): Promise<void> {
		if (this.closed) {
			return;
		}
		this.closed = true;
		await this.onClose(code, reason);
	}

	async broadcast(payload: JsonValue | string): Promise<void> {
		if (typeof payload === "string") {
			await this.sendText(payload);
			return;
		}
		await this.sendJson(payload);
	}

	isClosed(): boolean {
		return this.closed;
	}

	private ensureOpen(): void {
		if (this.closed) {
			throw new Error("WebSocket connection is closed");
		}
	}
}

export class WebSocketTestConnection {
	private readonly pending: WebSocketQueuedMessage[] = [];
	private readonly socket: WebSocketServerSocketImpl;
	private closed = false;

	private constructor(private readonly handler: WebSocketHandlerLike) {
		this.socket = new WebSocketServerSocketImpl(
			(message) => {
				this.pending.push(message);
			},
			async (code, reason) => {
				this.closed = true;
				if (isWebSocketHandler(this.handler) && this.handler.onClose) {
					await this.handler.onClose(this.socket, code, reason);
				}
			},
		);
	}

	static async connect(handler: WebSocketHandlerLike): Promise<WebSocketTestConnection> {
		const connection = new WebSocketTestConnection(handler);
		if (isWebSocketHandler(handler) && handler.onOpen) {
			await handler.onOpen(connection.socket);
		}
		return connection;
	}

	async sendJson(payload: JsonValue): Promise<void> {
		this.ensureOpen();
		await this.dispatchMessage(payload);
	}

	async sendText(payload: string): Promise<void> {
		this.ensureOpen();
		await this.dispatchMessage(payload);
	}

	async sendBytes(payload: BinaryLike): Promise<void> {
		this.ensureOpen();
		await this.dispatchMessage(payload);
	}

	async receiveJson(): Promise<JsonValue> {
		const message = this.shiftMessage();
		if (message.kind === "json") {
			return message.payload;
		}
		if (message.kind === "binary") {
			const text = textDecoder.decode(message.payload);
			const parsed = safeParseJson(text);
			if (parsed === null) {
				throw new Error("WebSocket binary message is not valid JSON");
			}
			return parsed;
		}
		const parsed = safeParseJson(message.payload);
		if (parsed === null) {
			throw new Error("WebSocket text message is not valid JSON");
		}
		return parsed;
	}

	async receiveText(): Promise<string> {
		const message = this.shiftMessage();
		if (message.kind === "text") {
			return message.payload;
		}
		if (message.kind === "binary") {
			return textDecoder.decode(message.payload);
		}
		return JSON.stringify(message.payload);
	}

	async receiveBytes(): Promise<Uint8Array> {
		const message = this.shiftMessage();
		if (message.kind === "binary") {
			return message.payload;
		}
		if (message.kind === "text") {
			return textEncoder.encode(message.payload);
		}
		return textEncoder.encode(JSON.stringify(message.payload));
	}

	async close(code?: number, reason?: string): Promise<void> {
		if (this.closed) {
			return;
		}
		this.closed = true;
		this.pending.length = 0;
		await this.socket.close(code, reason);
	}

	private async dispatchMessage(payload: JsonValue | string | BinaryLike): Promise<void> {
		const result = await (isWebSocketHandler(this.handler)
			? this.handler.onMessage?.(this.socket, payload)
			: this.handler(payload as HandlerPayload));
		if (result === undefined) {
			return;
		}
		if (isStreamingResponse(result)) {
			throw new Error("WebSocket handlers cannot return streaming responses");
		}
		const message = normalizeWebSocketResult(result);
		if (message) {
			this.pending.push(message);
		}
	}

	private shiftMessage(): WebSocketQueuedMessage {
		if (this.pending.length === 0) {
			throw new Error("No WebSocket messages available");
		}
		const message = this.pending.shift();
		if (!message) {
			throw new Error("No WebSocket messages available");
		}
		return message;
	}

	private ensureOpen(): void {
		if (this.closed) {
			throw new Error("WebSocket connection is closed");
		}
	}
}

function isWebSocketHandler(handler: WebSocketHandlerLike): handler is WebSocketHandler {
	if (typeof handler !== "object" || handler === null) {
		return false;
	}
	const candidate = handler as WebSocketHandler;
	return (
		typeof candidate.onMessage === "function" ||
		typeof candidate.onOpen === "function" ||
		typeof candidate.onClose === "function"
	);
}

function normalizeWebSocketResult(result: unknown): WebSocketQueuedMessage | null {
	if (result === null || result === undefined) {
		return null;
	}
	if (typeof result === "string") {
		const parsed = safeParseJson(result);
		if (parsed === null) {
			return { kind: "text", payload: result };
		}
		return { kind: "json", payload: parsed };
	}
	if (isBinaryLike(result)) {
		return { kind: "binary", payload: toUint8Array(result) };
	}
	return { kind: "json", payload: result as JsonValue };
}

function safeParseJson(text: string): JsonValue | null {
	try {
		return JSON.parse(text) as JsonValue;
	} catch {
		return null;
	}
}

function isBinaryLike(value: unknown): value is BinaryLike {
	return value instanceof ArrayBuffer || ArrayBuffer.isView(value);
}

function normalizeRecord(record: HeaderInput): HeaderMap {
	if (!record) {
		return {};
	}
	if (isMapLike(record)) {
		return Object.fromEntries(record.entries());
	}
	return record;
}

function normalizeValueRecord<T>(record: Map<string, T> | Record<string, T> | null | undefined): Record<string, T> {
	if (!record) {
		return {};
	}
	if (record instanceof Map) {
		return Object.fromEntries(record.entries());
	}
	return record;
}

function isMapLike(record: HeaderInput): record is Map<string, string> {
	return Boolean(record) && record instanceof Map;
}

function normalizeLifecycleHooks(hooks?: Partial<LifecycleHooks>): LifecycleHooks {
	if (!hooks) {
		return EMPTY_HOOKS;
	}
	return {
		onRequest: [...(hooks.onRequest ?? [])],
		preValidation: [...(hooks.preValidation ?? [])],
		preHandler: [...(hooks.preHandler ?? [])],
		onResponse: [...(hooks.onResponse ?? [])],
		onError: [...(hooks.onError ?? [])],
	};
}

function hasLifecycleHooks(hooks: LifecycleHooks): boolean {
	return (
		hooks.onRequest.length > 0 ||
		hooks.preValidation.length > 0 ||
		hooks.preHandler.length > 0 ||
		hooks.onResponse.length > 0 ||
		hooks.onError.length > 0
	);
}

function createNativeLifecycleHooks(hooks: LifecycleHooks): NativeLifecycleHooksPayload | null {
	if (!hasLifecycleHooks(hooks)) {
		return null;
	}
	return {
		onRequest: hooks.onRequest.map((hook) => wrapNativeRequestHook(hook)),
		preValidation: hooks.preValidation.map((hook) => wrapNativeRequestHook(hook)),
		preHandler: hooks.preHandler.map((hook) => wrapNativeRequestHook(hook)),
		onResponse: hooks.onResponse.map((hook) => wrapNativeResponseHook(hook)),
		onError: hooks.onError.map((hook) => wrapNativeResponseHook(hook)),
	};
}

function wrapNativeRequestHook(
	hook: LifecycleHookFunction,
): (payload: InternalRequestPayload) => Promise<NativeLifecycleHookResult> {
	return async (payload) => {
		const wrapped = maybeWrapRequestPayload(payload);
		if (!isWasmRequest(wrapped)) {
			throw new Error("Invalid lifecycle request payload");
		}
		const request = wrapped as WasmRequest;
		const result = await hook(request);
		const maybeRequest = ensureWasmRequest(result);
		if (maybeRequest) {
			return { type: "request", payload: serializeWasmRequest(maybeRequest) };
		}
		const response = await normalizeStructuredResponse(result);
		return { type: "response", payload: response };
	};
}

function wrapNativeResponseHook(
	hook: LifecycleHookFunction,
): (payload: StructuredHandlerResponse) => Promise<NativeLifecycleHookResult> {
	return async (payload) => {
		const current = await normalizeStructuredResponse(payload);
		const result = await hook(current);
		const maybeRequest = ensureWasmRequest(result);
		if (maybeRequest) {
			return { type: "response", payload: current };
		}
		const updated = await normalizeStructuredResponse(result, current);
		return { type: "response", payload: updated };
	};
}

function serializeWasmRequest(request: WasmRequest): InternalRequestPayload {
	return request.toPayload();
}

function normalizeJsonValue(value: unknown): JsonValue {
	if (value instanceof Map) {
		const result: Record<string, JsonValue> = {};
		for (const [key, entry] of value.entries()) {
			result[key] = normalizeJsonValue(entry);
		}
		return result;
	}
	if (Array.isArray(value)) {
		return value.map((entry) => normalizeJsonValue(entry)) as JsonValue;
	}
	if (value === null || typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
		return value as JsonValue;
	}
	if (typeof value === "bigint") {
		return value.toString();
	}
	if (typeof value === "object" && value) {
		const result: Record<string, JsonValue> = {};
		for (const [key, entry] of Object.entries(value as Record<string, unknown>)) {
			result[key] = normalizeJsonValue(entry);
		}
		return result;
	}
	return value as JsonValue;
}

function lowerCaseHeaderKeys(headers: HeaderMap): HeaderMap {
	const lowered: HeaderMap = {};
	for (const [key, value] of Object.entries(headers)) {
		lowered[key.toLowerCase()] = value;
	}
	return lowered;
}

function tryUnwrapStructuredSnapshot(body: Uint8Array): StructuredHandlerResponse | null {
	if (body.length === 0) {
		return null;
	}
	const text = textDecoder.decode(body);
	try {
		const parsed = JSON.parse(text) as unknown;
		if (!isStructuredResponseLike(parsed)) {
			return null;
		}
		return parsed as StructuredHandlerResponse;
	} catch {
		return null;
	}
}

function flattenStructuredResponse(response: StructuredHandlerResponse): StructuredHandlerResponse {
	let current: StructuredHandlerResponse = response;
	let mergedHeaders: HeaderMap = normalizeRecord(current.headers ?? {});

	while (current.body && typeof current.body === "object" && isStructuredResponseLike(current.body)) {
		const next = current.body as unknown as StructuredHandlerResponse;
		mergedHeaders = {
			...mergedHeaders,
			...normalizeRecord(next.headers ?? {}),
		};
		current = next;
	}

	return {
		...current,
		headers: mergedHeaders,
	};
}

function encodeBodyBytes(body: HandlerBody): Uint8Array {
	if (body == null) {
		return new Uint8Array();
	}
	if (typeof body === "string") {
		return textEncoder.encode(body);
	}
	return textEncoder.encode(JSON.stringify(body));
}

function toUint8Array(value: BinaryLike | ArrayLike<number>): Uint8Array {
	if (value instanceof Uint8Array) {
		return value;
	}
	if (value instanceof ArrayBuffer) {
		return new Uint8Array(value);
	}
	if (ArrayBuffer.isView(value)) {
		return new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
	}
	if (typeof Blob !== "undefined" && value instanceof Blob) {
		throw new Error("Blob payloads are not supported synchronously");
	}
	if (isArrayLike(value)) {
		return Uint8Array.from(value);
	}
	throw new Error("Unsupported binary payload");
}

function isArrayLike(value: unknown): value is ArrayLike<number> {
	if (typeof value === "object" && value !== null) {
		const length = (value as { length?: unknown }).length;
		return typeof length === "number";
	}
	return false;
}

function gunzipBytes(data: Uint8Array): Uint8Array {
	if (gunzipImplementation) {
		try {
			return gunzipImplementation(data);
		} catch {
			return data;
		}
	}
	try {
		return gunzipSync(data);
	} catch {
		return data;
	}
}

function bufferToBase64(bytes: Uint8Array): string {
	const bufferCtor = globalAny.Buffer as
		| { from: (data: Uint8Array) => { toString: (encoding: string) => string } }
		| undefined;
	if (bufferCtor) {
		return bufferCtor.from(bytes).toString("base64");
	}
	let binary = "";
	for (const byte of bytes) {
		binary += String.fromCharCode(byte);
	}
	if (typeof btoa === "function") {
		return btoa(binary);
	}
	throw new Error("Base64 encoding is not supported in this environment");
}

function isAbortSignalLike(value: unknown): value is AbortSignalLike {
	if (!value || typeof value !== "object") {
		return false;
	}
	const candidate = value as AbortSignalLike;
	return (
		typeof candidate.aborted === "boolean" &&
		typeof candidate.addEventListener === "function" &&
		typeof candidate.removeEventListener === "function"
	);
}

function wrapHandlers(handlers: Record<string, HandlerFunction>): Record<string, HandlerFunction> {
	const wrapped: Record<string, HandlerFunction> = {};
	for (const [name, handler] of Object.entries(handlers)) {
		const looksLikeStringHandler = handler.toString().includes("JSON.parse");
		wrapped[name] = async (request) => {
			const rawJson =
				request && typeof request === "object" && RAW_REQUEST_KEY in request
					? String((request as { [RAW_REQUEST_KEY]?: unknown })[RAW_REQUEST_KEY] ?? "")
					: JSON.stringify(request);
			const maybeSignal =
				request && typeof request === "object" && ABORT_SIGNAL_KEY in request
					? (request as { [ABORT_SIGNAL_KEY]?: unknown })[ABORT_SIGNAL_KEY]
					: undefined;
			const context: HandlerContext | undefined = isAbortSignalLike(maybeSignal) ? { signal: maybeSignal } : undefined;
			const normalizedRequest = maybeWrapRequestPayload(request);
			return looksLikeStringHandler ? handler(rawJson, context) : handler(normalizedRequest, context);
		};
	}
	return wrapped;
}

type RequestBodyKind = "none" | "json" | "form" | "multipart" | "binary" | "text";

interface BodyMetadata {
	kind: RequestBodyKind;
	text?: string | null;
	json?: JsonValue | null;
	form?: Record<string, string> | null;
	files?: MultipartFile[];
	rawBase64?: string | null;
}

interface InternalRequestPayload {
	method: string;
	path: string;
	pathParams?: Record<string, string>;
	params?: Record<string, JsonValue>;
	query?: Record<string, JsonValue>;
	rawQuery?: Record<string, string[]>;
	headers?: Record<string, string>;
	cookies?: Record<string, string>;
	body?: JsonValue | null;
	__spikard_body_metadata__?: BodyMetadata | null;
}

class WasmRequest implements Request {
	public readonly method: string;
	public readonly path: string;
	public readonly queryString: string;
	public readonly headers: Record<string, string>;
	public readonly params: Record<string, JsonValue>;
	public readonly pathParams: Record<string, string>;
	public readonly query: Record<string, JsonValue>;
	public readonly files?: MultipartFile[];
	private readonly rawJson: string;
	private readonly bodyKind: RequestBodyKind;
	private readonly bodyJson: JsonValue | undefined;
	private readonly formData: Record<string, string> | undefined;
	private readonly textBody: string | undefined;
	private bodyMetadata: BodyMetadata | null = null;
	private bodyDirty = false;
	private _body: Uint8Array | null;

	constructor(payload: InternalRequestPayload, rawJson: string) {
		this.rawJson = rawJson;
		this.method = payload.method;
		this.path = payload.path;
		this.params = normalizeValueRecord(payload.params as Map<string, JsonValue> | Record<string, JsonValue>);
		this.pathParams = normalizeValueRecord(payload.pathParams as Map<string, string> | Record<string, string>);
		this.query = normalizeValueRecord(payload.query as Map<string, JsonValue> | Record<string, JsonValue>);
		this.headers = normalizeRecord(payload.headers as HeaderInput);
		this.queryString = buildQueryString(payload.rawQuery as Map<string, string[]> | Record<string, string[]>);
		const normalizedBody = payload.body == null ? null : normalizeJsonValue(payload.body);
		let metadata = payload.__spikard_body_metadata__ ?? null;
		if ((!metadata || metadata.kind === "none") && normalizedBody != null) {
			metadata = buildBodyMetadata(normalizedBody);
		}
		if (metadata) {
			metadata.kind = normalizeRequestBodyKind(metadata.kind);
		}
		this.bodyMetadata = metadata ?? null;
		const applied = applyBodyMetadata(this.bodyMetadata, normalizedBody);
		this.bodyKind = applied.kind;
		this.bodyJson = applied.jsonValue;
		this.formData = applied.formValue;
		this.textBody = applied.textValue;
		if (applied.files !== undefined) {
			this.files = applied.files;
		}
		this._body = applied.buffer;
	}

	get body(): Uint8Array | null {
		return this._body;
	}

	set body(value: Uint8Array | null) {
		this._body = value;
		this.bodyDirty = true;
	}

	json<T extends JsonValue = JsonValue>(): T {
		let value: unknown;
		if (this.bodyKind === "json" && this.bodyJson !== undefined) {
			value = this.bodyJson;
		} else if (this.bodyKind === "text" && this.textBody !== undefined) {
			try {
				value = JSON.parse(this.textBody);
			} catch {
				throw new Error("Request body is not valid JSON");
			}
		} else {
			throw new Error("Request body is not JSON");
		}
		return normalizeJsonValue(value) as T;
	}

	form(): Record<string, string> {
		if (this.formData) {
			return { ...this.formData };
		}
		throw new Error("Request body is not form data");
	}

	toPayload(): InternalRequestPayload {
		const bodyValue = this.buildBodyValue();
		const metadata = this.getOrBuildBodyMetadata(bodyValue);
		const payload: InternalRequestPayload = {
			method: this.method,
			path: this.path,
			pathParams: { ...this.pathParams },
			params: { ...this.params },
			query: { ...this.query },
			rawQuery: buildRawQueryMap(this.queryString),
			headers: { ...this.headers },
			cookies: {},
			body: bodyValue,
		};
		if (metadata) {
			payload.__spikard_body_metadata__ = metadata;
		}
		return payload;
	}

	private buildBodyValue(): JsonValue | null {
		const currentBody = this.body;
		if (!currentBody) {
			switch (this.bodyKind) {
				case "json":
					return this.bodyJson ?? null;
				case "text":
					return this.textBody ?? null;
				case "form":
					return this.formData ? { __spikard_form__: this.formData } : null;
				case "multipart":
					return this.formData || this.files
						? ({
								__spikard_multipart__: {
									fields: this.formData ?? {},
									files: this.files ?? [],
								},
							} as unknown as JsonValue)
						: null;
				case "binary":
					if (this.bodyMetadata?.rawBase64) {
						return { __spikard_base64__: this.bodyMetadata.rawBase64 };
					}
					return null;
				default:
					return null;
			}
		}
		if (currentBody instanceof Uint8Array) {
			return { __spikard_base64__: bufferToBase64(currentBody) };
		}
		return normalizeJsonValue(currentBody as Record<string, unknown>);
	}

	private getOrBuildBodyMetadata(bodyValue: JsonValue | null): BodyMetadata | null {
		if (!this.bodyDirty && this.bodyMetadata) {
			return this.bodyMetadata;
		}
		const metadata = buildBodyMetadata(bodyValue ?? null);
		if (metadata) {
			metadata.kind = normalizeRequestBodyKind(metadata.kind);
		}
		this.bodyMetadata = metadata;
		this.bodyDirty = false;
		return metadata;
	}

	toString(): string {
		return this.rawJson;
	}

	valueOf(): string {
		return this.rawJson;
	}

	[Symbol.toPrimitive](): string {
		return this.rawJson;
	}

	toJSON(): Record<string, unknown> {
		try {
			return JSON.parse(this.rawJson) as Record<string, unknown>;
		} catch {
			return {
				method: this.method,
				path: this.path,
			};
		}
	}
}

function isWasmRequest(value: unknown): value is WasmRequest {
	return value instanceof WasmRequest;
}

function maybeWrapRequestPayload(payload: HandlerPayload | InternalRequestPayload): HandlerPayload {
	if (!payload || typeof payload !== "object") {
		return payload as HandlerPayload;
	}
	if (!("method" in payload) || !("path" in payload)) {
		return payload as HandlerPayload;
	}
	const candidate = payload as InternalRequestPayload & { [RAW_REQUEST_KEY]?: string };
	const rawJson =
		typeof candidate[RAW_REQUEST_KEY] === "string" ? candidate[RAW_REQUEST_KEY] : JSON.stringify(candidate);
	return new WasmRequest(candidate, rawJson) as unknown as HandlerPayload;
}

function buildQueryString(raw?: Map<string, string[]> | Record<string, string[]>): string {
	if (!raw) {
		return "";
	}
	const queryRecord = raw instanceof Map ? Object.fromEntries(raw.entries()) : raw;
	const params = new URLSearchParams();
	for (const [key, values] of Object.entries(queryRecord)) {
		for (const value of values ?? []) {
			params.append(key, value);
		}
	}
	return params.toString();
}

function buildRawQueryMap(queryString: string): Record<string, string[]> {
	if (!queryString) {
		return {};
	}
	const params = new URLSearchParams(queryString);
	const record: Record<string, string[]> = {};
	for (const [key, value] of params.entries()) {
		if (!record[key]) {
			record[key] = [];
		}
		record[key].push(value);
	}
	return record;
}

function base64ToUint8Array(base64: string): Uint8Array {
	const bufferCtor = globalAny.Buffer as { from: (data: string, encoding: string) => Uint8Array } | undefined;
	if (bufferCtor) {
		const buf = bufferCtor.from(base64, "base64");
		return new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
	}
	if (typeof atob === "function") {
		const binary = atob(base64);
		const bytes = new Uint8Array(binary.length);
		for (let i = 0; i < binary.length; i += 1) {
			bytes[i] = binary.charCodeAt(i);
		}
		return bytes;
	}
	throw new Error("Base64 decoding is not supported in this environment");
}

function decodeFormValues(values: Record<string, JsonValue>): Record<string, string> {
	const form: Record<string, string> = {};
	for (const [key, value] of Object.entries(values)) {
		if (value == null) {
			form[key] = "";
		} else if (typeof value === "string") {
			form[key] = value;
		} else {
			form[key] = JSON.stringify(value);
		}
	}
	return form;
}

function buildBuffer(bytes: Uint8Array | null): Uint8Array | null {
	if (!bytes) {
		return null;
	}
	const bufferCtor = globalAny.Buffer as { from: (data: Uint8Array) => Uint8Array } | undefined;
	if (bufferCtor) {
		const buf = bufferCtor.from(bytes);
		return new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
	}
	return bytes;
}

function applyBodyMetadata(
	metadata: BodyMetadata | null | undefined,
	payloadBody: JsonValue | null | undefined,
): {
	kind: RequestBodyKind;
	buffer: Uint8Array | null;
	jsonValue: JsonValue | undefined;
	formValue: Record<string, string> | undefined;
	textValue: string | undefined;
	files: MultipartFile[] | undefined;
} {
	const kind = normalizeRequestBodyKind(metadata?.kind);
	let buffer: Uint8Array | null = null;
	if (metadata?.rawBase64) {
		buffer = buildBuffer(base64ToUint8Array(metadata.rawBase64));
	}

	let jsonValue: JsonValue | undefined;
	let formValue: Record<string, string> | undefined;
	let textValue: string | undefined;
	const files = metadata?.files;

	switch (kind) {
		case "json":
			jsonValue = (metadata?.json as JsonValue) ?? (payloadBody as JsonValue);
			if (!buffer && payloadBody != null) {
				const text = JSON.stringify(payloadBody);
				buffer = buildBuffer(textEncoder.encode(text));
			}
			break;
		case "form":
		case "multipart":
			formValue = metadata?.form ?? undefined;
			break;
		case "text":
			textValue = metadata?.text ?? undefined;
			if (!buffer && textValue !== undefined) {
				buffer = buildBuffer(textEncoder.encode(textValue));
			}
			break;
		default:
			break;
	}

	return {
		kind,
		buffer,
		jsonValue,
		formValue,
		textValue,
		files,
	};
}

function buildBodyMetadata(body: JsonValue | null): BodyMetadata {
	if (body == null) {
		return { kind: "none" };
	}
	if (typeof body === "string") {
		const bytes = textEncoder.encode(body);
		return {
			kind: "text",
			text: body,
			rawBase64: bufferToBase64(bytes),
		};
	}
	if (typeof body === "number" || typeof body === "boolean") {
		const text = JSON.stringify(body);
		return {
			kind: "json",
			json: body as JsonValue,
			rawBase64: bufferToBase64(textEncoder.encode(text)),
		};
	}
	if (typeof body === "object") {
		const payload = body as Record<string, unknown>;
		if (typeof payload.__spikard_base64__ === "string") {
			return {
				kind: "binary",
				rawBase64: payload.__spikard_base64__ as string,
			};
		}
		if (payload.__spikard_form__ && typeof payload.__spikard_form__ === "object") {
			const formSource = normalizeValueRecord(
				payload.__spikard_form__ as Map<string, JsonValue> | Record<string, JsonValue>,
			);
			const formValue = decodeFormValues(formSource);
			return {
				kind: "form",
				form: formValue,
				rawBase64: bufferToBase64(textEncoder.encode(new URLSearchParams(formValue).toString())),
			};
		}
		if (payload.__spikard_multipart__ && typeof payload.__spikard_multipart__ === "object") {
			const multipart = payload.__spikard_multipart__ as {
				fields?: Record<string, JsonValue>;
				files?: MultipartFile[];
			};
			const fields = normalizeValueRecord(multipart.fields as Map<string, JsonValue> | Record<string, JsonValue>);
			const formValue = decodeFormValues(fields);
			return {
				kind: "multipart",
				form: formValue,
				files: multipart.files ?? [],
				rawBase64: bufferToBase64(textEncoder.encode(new URLSearchParams(formValue).toString())),
			};
		}
		return {
			kind: "json",
			json: normalizeJsonValue(payload),
			rawBase64: bufferToBase64(textEncoder.encode(JSON.stringify(payload))),
		};
	}
	return {
		kind: "json",
		json: body,
		rawBase64: bufferToBase64(textEncoder.encode(JSON.stringify(body))),
	};
}

function normalizeRequestBodyKind(value?: string | null): RequestBodyKind {
	switch ((value ?? "none").toLowerCase()) {
		case "json":
			return "json";
		case "form":
			return "form";
		case "multipart":
			return "multipart";
		case "binary":
			return "binary";
		case "text":
			return "text";
		default:
			return "none";
	}
}

function ensureWasmRequest(value: unknown): WasmRequest | null {
	if (value instanceof WasmRequest) {
		return value;
	}
	if (value && typeof value === "object" && "method" in value && "path" in value) {
		const rawJson =
			typeof (value as Record<string, unknown>)[RAW_REQUEST_KEY] === "string"
				? ((value as Record<string, string>)[RAW_REQUEST_KEY] as string)
				: JSON.stringify(value);
		return new WasmRequest(value as InternalRequestPayload, rawJson);
	}
	return null;
}

async function normalizeStructuredResponse(
	value: unknown,
	defaultResponse?: StructuredHandlerResponse,
): Promise<StructuredHandlerResponse> {
	if (typeof value === "string") {
		try {
			const parsed = JSON.parse(value) as StructuredHandlerResponse;
			return normalizeStructuredResponse(parsed);
		} catch {
			return {
				status: 200,
				headers: {},
				body: value,
			};
		}
	}
	if (value instanceof WasmRequest) {
		return (
			defaultResponse ?? {
				status: 200,
				headers: {},
				body: null,
			}
		);
	}
	if (isStructuredResponseLike(value)) {
		const headers = normalizeRecord(value.headers ?? {});
		const status = value.status ?? value.statusCode ?? 200;
		const body: HandlerBody =
			value.body === undefined
				? null
				: typeof value.body === "object" && value.body !== null
					? (value.body as HandlerBody)
					: (value.body as JsonValue);
		return {
			status,
			headers,
			body,
		};
	}
	return {
		status: 200,
		headers: {},
		body: value == null ? null : (value as JsonValue),
	};
}

function isStructuredResponseLike(value: unknown): value is StructuredHandlerResponse {
	if (!value || typeof value !== "object") {
		return false;
	}
	const hasStatus = "status" in value || "statusCode" in value;
	if (!hasStatus) {
		return false;
	}
	return "headers" in value || "body" in value;
}

function matchPath(template: string, actual: string): Record<string, string> | null {
	const params: Record<string, string> = {};
	const actualPath = actual.split("?")[0] ?? actual;

	const templateSegments = template.split("/").filter(Boolean);
	const actualSegments = actualPath.split("/").filter(Boolean);

	if (templateSegments.length !== actualSegments.length) {
		return null;
	}

	for (let i = 0; i < templateSegments.length; i += 1) {
		const templateSegment = templateSegments[i];
		const actualSegment = actualSegments[i];
		if (templateSegment === undefined || actualSegment === undefined) {
			return null;
		}
		if (templateSegment.startsWith("{") && templateSegment.endsWith("}")) {
			const key = templateSegment.slice(1, -1);
			params[key] = decodeURIComponent(actualSegment);
		} else if (templateSegment !== actualSegment) {
			return null;
		}
	}

	return params;
}

function methodsMatch(routeMethod: string, actualMethod: string): boolean {
	return routeMethod.toUpperCase() === actualMethod.toUpperCase();
}
