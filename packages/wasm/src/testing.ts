import { init as initWasm, TestClient as NativeTestClient } from "../runtime/spikard_wasm.js";
import type { LifecycleHookFunction, LifecycleHookPayload, LifecycleHooks } from "./app.ts";
import type { HandlerFunction, SpikardApp } from "./index.ts";
import type { Request } from "./request.ts";
import { isStreamingResponse } from "./streaming.ts";
import type { HandlerBody, HandlerPayload, JsonValue, StructuredHandlerResponse } from "./types.ts";

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

type NativeClient = InstanceType<typeof NativeTestClient>;

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
) => Promise<NativeClient>;

const defaultNativeClientFactory: NativeClientFactory = (routesJson, handlers, config, lifecycleHooks) =>
	wasmInitPromise.then(() => new NativeTestClient(routesJson, handlers, config, lifecycleHooks));

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
const wasmInitPromise = Promise.resolve(initWasm());
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
		if (typeof globalThis.Buffer !== "undefined") {
			return globalThis.Buffer.from(decoded);
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
	public readonly websocketHandlers: SpikardApp["handlers"];
	private readonly nativeClientPromise: Promise<NativeClient>;

	constructor(app: SpikardApp) {
		if (!app || !Array.isArray(app.routes)) {
			throw new Error("Invalid Spikard app: missing routes");
		}
		this.routes = app.routes;
		this.websocketHandlers = app.handlers ?? {};
		const routesJson = JSON.stringify(app.routes);
		const lifecycleHooks = normalizeLifecycleHooks(app.lifecycleHooks);
		const wrappedHandlers = wrapHandlers(this.websocketHandlers);
		const configString = app.config && Object.keys(app.config).length > 0 ? JSON.stringify(app.config) : null;

		const nativeLifecycleHooks = createNativeLifecycleHooks(lifecycleHooks);

		this.nativeClientPromise = nativeClientFactory(routesJson, wrappedHandlers, configString, nativeLifecycleHooks);
	}

	async get(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.get(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async delete(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.delete(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async head(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.head(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async options(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.options(path, headers ?? null);
		return this.responseFromNative(snapshot);
	}

	async trace(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		const native = await this.nativeClientPromise;
		const snapshot = await native.trace(path, headers ?? null);
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
		if (options.binary) {
			native.binary = options.binary;
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
		return value;
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

function toUint8Array(value: ArrayBuffer | ArrayLike<number> | Uint8Array): Uint8Array {
	if (value instanceof Uint8Array) {
		return value;
	}
	if (typeof globalThis.Buffer !== "undefined" && value instanceof globalThis.Buffer) {
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
	if (gunzipImplementation) {
		try {
			return gunzipImplementation(data);
		} catch {
			return data;
		}
	}
	return data;
}

function bufferToBase64(bytes: Uint8Array): string {
	if (typeof globalThis.Buffer !== "undefined") {
		return globalThis.Buffer.from(bytes).toString("base64");
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

function wrapHandlers(handlers: Record<string, HandlerFunction>): Record<string, HandlerFunction> {
	const wrapped: Record<string, HandlerFunction> = {};
	for (const [name, handler] of Object.entries(handlers)) {
		wrapped[name] = async (request) => {
			const normalizedRequest = maybeWrapRequestPayload(request);
			return handler(normalizedRequest);
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
	private _body: Buffer | Uint8Array | null;

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

	get body(): Buffer | Uint8Array | null {
		return this._body;
	}

	set body(value: Buffer | Uint8Array | null) {
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
		// Normalize Maps to plain objects (from serde_wasm_bindgen conversion)
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
		if (typeof currentBody === "string") {
			return currentBody;
		}
		if (
			typeof globalThis.Buffer !== "undefined" &&
			typeof globalThis.Buffer === "function" &&
			currentBody instanceof globalThis.Buffer
		) {
			return { __spikard_base64__: bufferToBase64(toUint8Array(currentBody)) };
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
	if (typeof globalThis.Buffer !== "undefined") {
		return new Uint8Array(globalThis.Buffer.from(base64, "base64"));
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

function buildBuffer(bytes: Uint8Array | null): Buffer | Uint8Array | null {
	if (!bytes) {
		return null;
	}
	if (typeof globalThis.Buffer !== "undefined") {
		return globalThis.Buffer.from(bytes);
	}
	return bytes;
}

function applyBodyMetadata(
	metadata: BodyMetadata | null | undefined,
	payloadBody: JsonValue | null | undefined,
): {
	kind: RequestBodyKind;
	buffer: Buffer | Uint8Array | null;
	jsonValue: JsonValue | undefined;
	formValue: Record<string, string> | undefined;
	textValue: string | undefined;
	files: MultipartFile[] | undefined;
} {
	const kind = normalizeRequestBodyKind(metadata?.kind);
	let buffer: Buffer | Uint8Array | null = null;
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
	value: HandlerPayload | LifecycleHookPayload,
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
					? (value.body as StructuredHandlerResponse["body"])
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
	return (
		!!value &&
		typeof value === "object" &&
		("status" in value || "statusCode" in value || "headers" in value || "body" in value)
	);
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
