import { Buffer } from "node:buffer";
import fs from "node:fs";
import path from "node:path";
import { gunzipSync } from "node:zlib";
import { TestClient as NativeTestClient } from "../runtime/spikard_wasm.js";
import type { ServerConfig, StaticFilesConfig } from "./config";
import type { HandlerFunction, SpikardApp } from "./index";
import { isStreamingResponse } from "./streaming";
import type { HandlerPayload, JsonValue, StructuredHandlerResponse } from "./types";

type HeaderMap = Record<string, string>;
type HeaderInput = HeaderMap | Map<string, string> | null | undefined;

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
}

interface StaticManifestEntry {
	route: string;
	headers: HeaderMap;
	body: string;
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
};

interface NativeSnapshot {
	status: number;
	headers: HeaderMap;
	body: Uint8Array;
}

const textDecoder = new TextDecoder();

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
		if (typeof Buffer !== "undefined") {
			return Buffer.from(decoded);
		}
		return decoded.slice();
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
	public readonly websocketHandlers: SpikardApp["handlers"];
	private readonly nativeClient: InstanceType<typeof NativeTestClient>;

	constructor(app: SpikardApp) {
		if (!app || !Array.isArray(app.routes)) {
			throw new Error("Invalid Spikard app: missing routes");
		}
		this.routes = app.routes;
		this.websocketHandlers = app.handlers ?? {};
		const routesJson = JSON.stringify(app.routes);
		const handlers = wrapHandlers(this.websocketHandlers);
		const enhancedConfig = enhanceConfig(app.config);
		const configString = enhancedConfig ? JSON.stringify(enhancedConfig) : null;

		this.nativeClient = new NativeTestClient(routesJson, handlers, configString);
	}

	async get(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const snapshot = await this.nativeClient.get(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async delete(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const snapshot = await this.nativeClient.delete(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async head(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const snapshot = await this.nativeClient.head(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async options(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const snapshot = await this.nativeClient.options(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async trace(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const snapshot = await this.nativeClient.trace(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async post(path: string, options?: RequestOptions): Promise<TestResponse> {
		const snapshot = await this.nativeClient.post(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async put(path: string, options?: RequestOptions): Promise<TestResponse> {
		const snapshot = await this.nativeClient.put(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async patch(path: string, options?: RequestOptions): Promise<TestResponse> {
		const snapshot = await this.nativeClient.patch(path, this.buildNativeOptions(options));
		return this.responseFromNative(snapshot);
	}

	async websocketConnect(path: string): Promise<WebSocketTestConnection> {
		const route = this.findRoute("GET", path);
		if (!route) {
			throw new Error(`WebSocket route not found for ${path}`);
		}
		const handler = this.websocketHandlers[route.metadata.handler_name];
		if (!handler) {
			throw new Error(`Handler ${route.metadata.handler_name} not registered`);
		}
		return new WebSocketTestConnection(handler);
	}

	private responseFromNative(snapshot: NativeSnapshot) {
		const headers = normalizeRecord(snapshot.headers);
		return new TestResponse(snapshot.status, headers, snapshot.body);
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
			native.json = options.json ?? null;
		}
		return Object.keys(native).length === 0 ? null : native;
	}

	private buildHeaders(options?: RequestOptions): HeaderMap | null {
		if (!options?.headers || Object.keys(options.headers).length === 0) {
			return null;
		}
		return options.headers;
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
}

export class WebSocketTestConnection {
	private readonly pending: JsonValue[] = [];

	constructor(private readonly handler: HandlerFunction) {}

	async sendJson(payload: JsonValue): Promise<void> {
		const result = await this.handler(payload);
		if (isStreamingResponse(result)) {
			throw new Error("WebSocket handlers cannot return streaming responses");
		}
		const parsed = typeof result === "string" ? (JSON.parse(result) as JsonValue) : (result as JsonValue);
		this.pending.push(parsed);
	}

	async receiveJson(): Promise<JsonValue> {
		if (this.pending.length === 0) {
			throw new Error("No WebSocket messages available");
		}
		const message = this.pending.shift();
		if (message === undefined) {
			throw new Error("No WebSocket messages available");
		}
		return message;
	}

	async sendText(payload: string): Promise<void> {
		await this.sendJson(payload);
	}

	async receiveText(): Promise<string> {
		const result = await this.receiveJson();
		return typeof result === "string" ? result : JSON.stringify(result);
	}

	async close(): Promise<void> {
		this.pending.length = 0;
	}
}

function buildStaticManifest(configs: StaticFilesConfig[]): StaticManifestEntry[] {
	if (!configs || configs.length === 0) {
		return [];
	}
	const manifest: StaticManifestEntry[] = [];
	for (const config of configs) {
		if (!config?.directory || !config.routePrefix) {
			continue;
		}
		if (!fs.existsSync(config.directory)) {
			continue;
		}
		const files = listFiles(config.directory);
		for (const filePath of files) {
			const relative = path.relative(config.directory, filePath).split(path.sep).join("/");
			const route = normalizeRoute(`${config.routePrefix.replace(/\/+$/, "")}/${relative}`);
			const headers = buildStaticHeaders(filePath, config.cacheControl);
			const body = fs.readFileSync(filePath).toString("base64");
			manifest.push({ route, headers, body });
		}

		if (config.indexFile ?? true) {
			const indexPath = path.join(config.directory, "index.html");
			if (fs.existsSync(indexPath)) {
				const headers = buildStaticHeaders(indexPath, config.cacheControl);
				const body = fs.readFileSync(indexPath).toString("base64");
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

function listFiles(root: string): string[] {
	const entries: string[] = [];
	const stack: string[] = [root];
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
		} else {
			entries.push(current);
		}
	}
	return entries;
}

function buildStaticHeaders(filePath: string, cacheControl?: string | null): Record<string, string> {
	const headers: Record<string, string> = {
		"content-type": lookupContentType(filePath),
	};
	if (cacheControl) {
		headers["cache-control"] = cacheControl;
	}
	return headers;
}

function lookupContentType(filePath: string): string {
	const ext = path.extname(filePath).toLowerCase();
	switch (ext) {
		case ".html":
		case ".htm":
			return "text/html";
		case ".txt":
			return "text/plain";
		case ".json":
			return "application/json";
		case ".js":
			return "application/javascript";
		case ".css":
			return "text/css";
		default:
			return "application/octet-stream";
	}
}

function normalizeRoute(route: string): string {
	const normalized = route.replace(/\/+/g, "/");
	return normalized.startsWith("/") ? normalized : `/${normalized}`;
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

function isMapLike(record: HeaderInput): record is Map<string, string> {
	return Boolean(record) && record instanceof Map;
}

function toUint8Array(value: ArrayBuffer | ArrayLike<number> | Uint8Array): Uint8Array {
	if (value instanceof Uint8Array) {
		return value;
	}
	if (typeof Buffer !== "undefined" && value instanceof Buffer) {
		return new Uint8Array(value);
	}
	if (ArrayBuffer.isView(value)) {
		return new Uint8Array(value.buffer.slice(value.byteOffset, value.byteOffset + value.byteLength));
	}
	if (value instanceof ArrayBuffer) {
		return new Uint8Array(value);
	}
	return Uint8Array.from(value as ArrayLike<number>);
}

function gunzipBytes(data: Uint8Array): Uint8Array {
	if (typeof Buffer === "undefined") {
		return data;
	}
	try {
		const buffer = Buffer.from(data);
		return new Uint8Array(gunzipSync(buffer));
	} catch {
		return data;
	}
}

function bufferToBase64(bytes: Uint8Array): string {
	if (typeof Buffer !== "undefined") {
		return Buffer.from(bytes).toString("base64");
	}
	let binary = "";
	for (const byte of bytes) {
		binary += String.fromCharCode(byte);
	}
	return btoa(binary);
}

function shouldDecodeAsText(contentType?: string): boolean {
	if (!contentType) {
		return false;
	}
	const lowercase = contentType.toLowerCase();
	return lowercase.startsWith("text/") || lowercase.includes("json") || lowercase.includes("javascript");
}

function enhanceConfig(config?: ServerConfig | null) {
	const manifest = buildStaticManifest(config?.staticFiles ?? []);
	if ((!config || Object.keys(config).length === 0) && manifest.length === 0) {
		return null;
	}
	if (manifest.length === 0) {
		return config ?? null;
	}
	return {
		...config,
		__wasmStaticManifest: manifest,
	};
}

function wrapHandlers(handlers: Record<string, HandlerFunction>): Record<string, HandlerFunction> {
	const wrapped: Record<string, HandlerFunction> = {};
	for (const [name, handler] of Object.entries(handlers)) {
		wrapped[name] = async (request: HandlerPayload) => {
			const result = await handler(request);
			if (isStreamingResponse(result)) {
				const buffer = await result.collect();
				const headers = result.headers ?? {};
				const responsePayload: StructuredHandlerResponse = {
					status: result.statusCode,
					headers,
				};
				const contentType = headers["content-type"];
				if (shouldDecodeAsText(contentType)) {
					responsePayload.body = textDecoder.decode(buffer);
				} else {
					responsePayload.body = { __spikard_base64__: bufferToBase64(buffer) };
				}
				return JSON.stringify(responsePayload);
			}
			return result;
		};
	}
	return wrapped;
}

function matchPath(template: string, actual: string): Record<string, string> | null {
	const params: Record<string, string> = {};
	const [actualPath] = actual.split("?");

	const templateSegments = template.split("/").filter(Boolean);
	const actualSegments = actualPath.split("/").filter(Boolean);

	if (templateSegments.length !== actualSegments.length) {
		return null;
	}

	for (let i = 0; i < templateSegments.length; i += 1) {
		const templateSegment = templateSegments[i];
		const actualSegment = actualSegments[i];
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
