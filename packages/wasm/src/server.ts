import type { ServerConfig } from "./config";
import type { SpikardApp } from "./index";
import { type MultipartFile, TestClient, type TestResponse } from "./testing";
import { isStreamingResponse } from "./streaming";
import type { BinaryLike, JsonValue, WebSocketHandlerLike, WebSocketServerSocket } from "./types";
import type { RouteMetadata } from "./index";

export interface ServerOptions {
	host?: string;
	port?: number;
}

type FetchHandler = (request: Request) => Promise<Response>;

export function createFetchHandler(app: SpikardApp): FetchHandler {
	const client = new TestClient(app);
	return async (request: Request) => {
		const websocketResponse = await maybeHandleWebSocketUpgrade(app, request);
		if (websocketResponse) {
			return websocketResponse;
		}

		const url = new URL(request.url);
		const path = `${url.pathname}${url.search}`;
		const headers = headersToRecord(request.headers);
		const method = request.method.toUpperCase();
		const options = await buildRequestOptions(request);
		let response: TestResponse;

		switch (method) {
			case "GET":
				response = await client.get(path, headers);
				break;
			case "DELETE":
				response = await client.delete(path, headers);
				break;
			case "HEAD":
				response = await client.head(path, headers);
				break;
			case "OPTIONS":
				response = await client.options(path, headers);
				break;
			case "TRACE":
				response = await client.trace(path, headers);
				break;
			case "POST":
				response = await client.post(path, mergeHeaders(options, headers));
				break;
			case "PUT":
				response = await client.put(path, mergeHeaders(options, headers));
				break;
			case "PATCH":
				response = await client.patch(path, mergeHeaders(options, headers));
				break;
			default:
				return new Response("Method Not Allowed", { status: 405 });
		}

		return responseToFetch(response);
	};
}

export function runServer(app: SpikardApp, config: ServerConfig | ServerOptions = {}): void {
	const handler = createFetchHandler(app);
	const options = resolveServerOptions(config);
	if (isBun()) {
		const bun = (globalThis as unknown as { Bun: { serve: (...args: unknown[]) => unknown } }).Bun;
		bun.serve({
			fetch: (req: Request, server?: { upgrade: (req: Request, options?: { data?: unknown }) => boolean }) => {
				const upgradeResult = maybeUpgradeBunWebSocket(req, server, app);
				if (upgradeResult) {
					return upgradeResult;
				}
				return handler(req);
			},
			websocket: createBunWebSocketHandlers(),
			hostname: options.host ?? "0.0.0.0",
			port: options.port ?? 0,
		});
		return;
	}
	const globalAny = globalThis as unknown as {
		Deno?: { serve?: (options: { hostname?: string; port?: number }, handler: FetchHandler) => void };
		addEventListener?: (
			type: "fetch",
			listener: (event: Event & { request: Request; respondWith(response: Promise<Response>): void }) => void,
		) => void;
	};
	if (globalAny.Deno && typeof globalAny.Deno.serve === "function") {
		globalAny.Deno.serve(
			{
				hostname: options.host ?? "0.0.0.0",
				port: options.port ?? 0,
			},
			handler,
		);
		return;
	}
	if (typeof globalAny.addEventListener === "function") {
		globalAny.addEventListener(
			"fetch",
			(event: Event & { request: Request; respondWith(response: Promise<Response>): void }) => {
				event.respondWith(handler(event.request));
			},
		);
		return;
	}
	throw new Error("Unsupported runtime: unable to start WASM HTTP server");
}

function isBun(): boolean {
	const globalAny = globalThis as unknown as { Bun?: { serve?: unknown } };
	return typeof globalAny.Bun === "object" && typeof globalAny.Bun?.serve === "function";
}

function headersToRecord(headers: Headers): Record<string, string> {
	const record: Record<string, string> = {};
	for (const [key, value] of headers.entries()) {
		record[key] = value;
	}
	return record;
}

async function buildRequestOptions(request: Request) {
	const method = request.method.toUpperCase();
	if (method === "GET" || method === "HEAD") {
		return undefined;
	}
	const contentType = request.headers.get("content-type") ?? "";
	if (contentType.includes("application/json")) {
		const text = await request.text();
		if (!text) {
			return { json: null };
		}
		try {
			return { json: JSON.parse(text) };
		} catch {
			return { json: text };
		}
	}
	if (contentType.includes("application/x-www-form-urlencoded")) {
		const text = await request.text();
		return text ? { formRaw: text } : undefined;
	}
	if (contentType.includes("multipart/form-data") && typeof request.formData === "function") {
		const formData = await request.formData();
		const fields: Record<string, string> = {};
		const files: MultipartFile[] = [];
		for (const [key, value] of formData.entries()) {
			if (typeof value === "string") {
				fields[key] = value;
			} else {
				const buffer = new Uint8Array(await value.arrayBuffer());
				const file: MultipartFile = {
					name: key,
					content: bufferToBase64(buffer),
					...(value.name ? { filename: value.name } : {}),
					...(value.type ? { contentType: value.type } : {}),
				};
				files.push(file);
			}
		}
		return { multipart: { fields, files } };
	}
	const buffer = new Uint8Array(await request.arrayBuffer());
	if (buffer.length === 0) {
		return undefined;
	}
	return { binary: bufferToBase64(buffer) };
}

function mergeHeaders(options: Parameters<TestClient["post"]>[1], headers: Record<string, string>) {
	if (options) {
		return {
			...options,
			headers: {
				...options.headers,
				...headers,
			},
		};
	}
	return { headers };
}

function responseToFetch(response: TestResponse): Response {
	const headers = new Headers(response.headers());
	return new Response(response.raw(), {
		status: response.statusCode,
		headers,
	});
}

function bufferToBase64(bytes: Uint8Array): string {
	const globalAny = globalThis as unknown as {
		Buffer?: { from: (bytes: Uint8Array) => { toString: (encoding: "base64") => string } };
	};
	if (globalAny.Buffer) {
		return globalAny.Buffer.from(bytes).toString("base64");
	}
	let binary = "";
	for (const byte of bytes) {
		binary += String.fromCharCode(byte);
	}
	if (typeof btoa === "function") {
		return btoa(binary);
	}
	throw new Error("Base64 encoding is not supported in this runtime");
}

function resolveServerOptions(config: ServerConfig | ServerOptions): ServerOptions {
	if ("host" in config || "port" in config) {
		const options = config as ServerOptions;
		const resolved: ServerOptions = {};
		if (options.host !== undefined) {
			resolved.host = options.host;
		}
		if (options.port !== undefined) {
			resolved.port = options.port;
		}
		return resolved;
	}
	return {};
}

type WebSocketRouteMatch = {
	metadata: RouteMetadata;
	handler: WebSocketHandlerLike;
};

type WebSocketRegistry = {
	routes: RouteMetadata[];
	handlers: Record<string, WebSocketHandlerLike>;
};

async function maybeHandleWebSocketUpgrade(app: SpikardApp, request: Request): Promise<Response | null> {
	if (!isWebSocketUpgrade(request)) {
		return null;
	}
	const url = new URL(request.url);
	const match = findWebSocketRoute(app, url.pathname);
	if (!match) {
		return new Response("WebSocket route not found", { status: 404 });
	}

	const denoUpgrade = getDenoUpgrade();
	if (denoUpgrade) {
		const { socket, response } = denoUpgrade(request);
		startWebSocketSession(socket, match.handler);
		return response;
	}

	const webSocketPair = getWebSocketPair();
	if (webSocketPair) {
		const pair = new webSocketPair();
		const [clientSocket, serverSocket] = pair;
		serverSocket.accept();
		startWebSocketSession(serverSocket, match.handler);
		return new Response(null, { status: 101, webSocket: clientSocket });
	}

	return new Response("WebSocket upgrade not supported in this runtime", { status: 501 });
}

function buildWebSocketRegistry(app: SpikardApp): WebSocketRegistry {
	return {
		routes: app.websocketRoutes ?? [],
		handlers: app.websocketHandlers ?? app.handlers ?? {},
	};
}

function findWebSocketRoute(app: SpikardApp, path: string): WebSocketRouteMatch | null {
	const registry = buildWebSocketRegistry(app);
	const routeMatch = findRouteByPath(registry.routes, path);
	if (routeMatch) {
		const handler = registry.handlers[routeMatch.metadata.handler_name];
		if (handler) {
			return { metadata: routeMatch.metadata, handler };
		}
	}

	const fallback = findRouteByPath(app.routes ?? [], path);
	if (fallback) {
		const handler = registry.handlers[fallback.metadata.handler_name];
		if (handler) {
			return { metadata: fallback.metadata, handler };
		}
	}

	return null;
}

function findRouteByPath(
	routes: RouteMetadata[],
	targetPath: string,
): { metadata: RouteMetadata; params: Record<string, string> } | null {
	for (const metadata of routes) {
		if (metadata.method !== "GET") {
			continue;
		}
		const params = matchPath(metadata.path, targetPath);
		if (params) {
			return { metadata, params };
		}
	}
	return null;
}

function isWebSocketUpgrade(request: Request): boolean {
	const upgrade = request.headers.get("upgrade");
	if (!upgrade || upgrade.toLowerCase() !== "websocket") {
		return false;
	}
	const connection = request.headers.get("connection")?.toLowerCase();
	return !connection || connection.includes("upgrade");
}

type DenoUpgrade = (request: Request) => { socket: WebSocket; response: Response };

function getDenoUpgrade(): DenoUpgrade | null {
	const denoGlobal = (globalThis as unknown as { Deno?: { upgradeWebSocket?: DenoUpgrade } }).Deno;
	return denoGlobal?.upgradeWebSocket ?? null;
}

type WebSocketPairCtor = new () => [WebSocket, WebSocket];

function getWebSocketPair(): WebSocketPairCtor | null {
	const pairCtor = (globalThis as unknown as { WebSocketPair?: WebSocketPairCtor }).WebSocketPair;
	return pairCtor ?? null;
}

function startWebSocketSession(socket: WebSocket, handler: WebSocketHandlerLike): void {
	const runtimeSocket = createRuntimeSocket(socket);

	if (isWebSocketHandler(handler) && handler.onOpen) {
		void handler.onOpen(runtimeSocket);
	}

	socket.addEventListener("message", (event: MessageEvent) => {
		void handleWebSocketMessage(runtimeSocket, handler, event.data).catch(() => {
			closeSocket(runtimeSocket, 1011, "WebSocket handler error");
		});
	});

	socket.addEventListener("close", (event: CloseEvent) => {
		if (isWebSocketHandler(handler) && handler.onClose) {
			void handler.onClose(runtimeSocket, event.code, event.reason);
		}
		runtimeSocket.close(event.code, event.reason);
	});
}

async function handleWebSocketMessage(
	socket: WebSocketServerSocket,
	handler: WebSocketHandlerLike,
	data: unknown,
): Promise<void> {
	const payload = decodeWebSocketMessage(data);
	const result = isWebSocketHandler(handler)
		? await handler.onMessage?.(socket, payload)
		: await handler(payload);

	if (result === undefined) {
		return;
	}
	if (isStreamingResponse(result)) {
		throw new Error("WebSocket handlers cannot return streaming responses");
	}

	await sendWebSocketResult(socket, result);
}

async function sendWebSocketResult(socket: WebSocketServerSocket, result: unknown): Promise<void> {
	if (result === null || result === undefined) {
		return;
	}
	if (typeof result === "string") {
		await socket.sendText(result);
		return;
	}
	if (isBinaryLike(result)) {
		await socket.sendBytes?.(result);
		return;
	}
	await socket.sendJson(result as JsonValue);
}

function decodeWebSocketMessage(data: unknown): JsonValue | string | BinaryLike {
	if (typeof data === "string") {
		const parsed = safeParseJson(data);
		return parsed ?? data;
	}
	if (data instanceof ArrayBuffer || ArrayBuffer.isView(data)) {
		return data as BinaryLike;
	}
	if (typeof Blob !== "undefined" && data instanceof Blob) {
		return data;
	}
	return data as JsonValue;
}

function safeParseJson(text: string): JsonValue | null {
	try {
		return JSON.parse(text) as JsonValue;
	} catch {
		return null;
	}
}

function isBinaryLike(value: unknown): value is BinaryLike {
	if (value instanceof ArrayBuffer || ArrayBuffer.isView(value)) {
		return true;
	}
	return typeof Blob !== "undefined" && value instanceof Blob;
}

function createRuntimeSocket(socket: WebSocket): WebSocketServerSocket {
	let closed = false;
	return {
		sendText(message: string) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			socket.send(message);
		},
		sendJson(payload: JsonValue) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			socket.send(JSON.stringify(payload));
		},
		sendBytes(payload: BinaryLike) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			socket.send(payload);
		},
		close(code?: number, reason?: string) {
			if (closed) {
				return;
			}
			closed = true;
			socket.close(code, reason);
		},
		isClosed() {
			return closed || socket.readyState === 3;
		},
	};
}

function closeSocket(socket: WebSocketServerSocket, code?: number, reason?: string): void {
	try {
		socket.close(code, reason);
	} catch {}
}

function isWebSocketHandler(handler: WebSocketHandlerLike): handler is {
	onOpen?: (socket: WebSocketServerSocket) => Promise<void> | void;
	onMessage?: (socket: WebSocketServerSocket, data: JsonValue | string | BinaryLike) => Promise<unknown> | unknown;
	onClose?: (socket: WebSocketServerSocket, code?: number, reason?: string) => Promise<void> | void;
} {
	if (typeof handler !== "object" || handler === null) {
		return false;
	}
	const candidate = handler as {
		onOpen?: unknown;
		onMessage?: unknown;
		onClose?: unknown;
	};
	return (
		typeof candidate.onOpen === "function" ||
		typeof candidate.onMessage === "function" ||
		typeof candidate.onClose === "function"
	);
}

function matchPath(template: string, actual: string): Record<string, string> | null {
	const templateParts = template.split("/").filter(Boolean);
	const actualParts = actual.split("/").filter(Boolean);
	if (templateParts.length !== actualParts.length) {
		return null;
	}
	const params: Record<string, string> = {};
	for (let i = 0; i < templateParts.length; i++) {
		const templatePart = templateParts[i];
		const actualPart = actualParts[i];
		if (!templatePart || actualPart === undefined) {
			return null;
		}
		if (templatePart.startsWith(":")) {
			params[templatePart.slice(1)] = decodeURIComponent(actualPart);
		} else if (templatePart !== actualPart) {
			return null;
		}
	}
	return params;
}

type BunServer = { upgrade: (req: Request, options?: { data?: unknown }) => boolean };

function maybeUpgradeBunWebSocket(request: Request, server: BunServer | undefined, app: SpikardApp): Response | null {
	if (!server || !isWebSocketUpgrade(request)) {
		return null;
	}
	const url = new URL(request.url);
	const match = findWebSocketRoute(app, url.pathname);
	if (!match) {
		return null;
	}
	const upgraded = server.upgrade(request, { data: { handler: match.handler, path: url.pathname } });
	if (upgraded) {
		return new Response(null, { status: 101 });
	}
	return null;
}

function createBunWebSocketHandlers() {
	return {
		open(ws: { data?: { handler?: WebSocketHandlerLike }; send: (data: unknown) => void }) {
			const handler = ws.data?.handler;
			if (!handler) {
				return;
			}
			const runtimeSocket = createBunRuntimeSocket(ws);
			ws.data = { handler, socket: runtimeSocket };
			if (isWebSocketHandler(handler) && handler.onOpen) {
				void handler.onOpen(runtimeSocket);
			}
		},
		async message(
			ws: { data?: { handler?: WebSocketHandlerLike; socket?: WebSocketServerSocket }; send: (data: unknown) => void },
			message: string | Uint8Array,
		) {
			const handler = ws.data?.handler;
			const runtimeSocket = ws.data?.socket;
			if (!handler || !runtimeSocket) {
				return;
			}
			try {
				await handleWebSocketMessage(runtimeSocket, handler, message);
			} catch {
				closeSocket(runtimeSocket, 1011, "WebSocket handler error");
			}
		},
		close(
			ws: { data?: { handler?: WebSocketHandlerLike; socket?: WebSocketServerSocket } },
			code: number,
			reason: string,
		) {
			const handler = ws.data?.handler;
			const runtimeSocket = ws.data?.socket;
			if (!handler || !runtimeSocket) {
				return;
			}
			if (isWebSocketHandler(handler) && handler.onClose) {
				void handler.onClose(runtimeSocket, code, reason);
			}
		},
	} satisfies Record<string, unknown>;
}

function createBunRuntimeSocket(ws: { send: (data: unknown) => void; close?: (code?: number, reason?: string) => void }): WebSocketServerSocket {
	let closed = false;
	return {
		sendText(message: string) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			ws.send(message);
		},
		sendJson(payload: JsonValue) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			ws.send(JSON.stringify(payload));
		},
		sendBytes(payload: BinaryLike) {
			if (closed) {
				throw new Error("WebSocket connection is closed");
			}
			ws.send(payload);
		},
		close(code?: number, reason?: string) {
			if (closed) {
				return;
			}
			closed = true;
			ws.close?.(code, reason);
		},
		isClosed() {
			return closed;
		},
	};
}
