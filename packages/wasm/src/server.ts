import type { ServerConfig } from "./config";
import type { SpikardApp } from "./index";
import { type MultipartFile, TestClient, type TestResponse } from "./testing";

export interface ServerOptions {
	host?: string;
	port?: number;
}

type FetchHandler = (request: Request) => Promise<Response>;

export function createFetchHandler(app: SpikardApp): FetchHandler {
	const client = new TestClient(app);
	return async (request: Request) => {
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
		(globalThis as unknown as { Bun: { serve: (...args: unknown[]) => unknown } }).Bun.serve({
			fetch: handler,
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
