#!/usr/bin/env -S deno run --allow-net --allow-read
/**
 * Spikard WASM HTTP server for workload benchmarking using Deno.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline.
 */

import * as wasm from "../../../../crates/spikard-wasm/dist-web/spikard_wasm.js";

if (typeof (wasm as { default?: unknown }).default === "function") {
	await (wasm as { default: () => Promise<unknown> }).default();
}
if (typeof (wasm as { init?: unknown }).init === "function") {
	(wasm as { init: () => unknown }).init();
}

const TestClient = (wasm as { TestClient: typeof wasm.TestClient }).TestClient;

// Type definitions for the WASM server
interface Route {
	readonly method: string;
	readonly path: string;
	readonly handler_name: string;
	readonly is_async: boolean;
}

interface PathParams {
	readonly [key: string]: string;
}

interface QueryParams {
	readonly [key: string]: string | readonly string[];
}

interface JsonBody {
	readonly [key: string]: unknown;
}

interface MultipartResponse {
	readonly files_received: number;
	readonly total_bytes: number;
}

interface PathResponse {
	readonly [key: string]: string | number;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type HandlerFunction = (input: any) => Promise<unknown>;

interface ServerRequest {
	readonly method: string;
	readonly path: string;
	readonly query: QueryParams;
	readonly headers: Record<string, string>;
	readonly body: unknown;
}

interface ServerResponseBody {
	readonly [key: string]: number;
}

interface ServerResponse {
	readonly status?: number;
	readonly headers?: Record<string, string>;
	readonly body?: Uint8Array | readonly number[] | ServerResponseBody | null;
}

// Route definitions and handlers for the benchmark app
const routes: Route[] = [];
const handlers: Record<string, HandlerFunction> = {};

function registerRoute(method: string, path: string, handler: HandlerFunction): void {
	routes.push({
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
	});
	handlers[handler.name] = handler;
}

function get(path: string): (handler: HandlerFunction) => void {
	return (handler: HandlerFunction): void => registerRoute("GET", path, handler);
}

function post(path: string): (handler: HandlerFunction) => void {
	return (handler: HandlerFunction): void => registerRoute("POST", path, handler);
}

// ============================================================================
// JSON Body Workloads
// ============================================================================

post("/json/small")(async function jsonSmall(body: unknown): Promise<unknown> {
	return body;
});

post("/json/medium")(async function jsonMedium(body: unknown): Promise<unknown> {
	return body;
});

post("/json/large")(async function jsonLarge(body: unknown): Promise<unknown> {
	return body;
});

post("/json/very-large")(async function jsonVeryLarge(body: unknown): Promise<unknown> {
	return body;
});

// ============================================================================
// Multipart Form Workloads
// ============================================================================

post("/multipart/small")(async function multipartSmall(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 1, total_bytes: 1024 };
});

post("/multipart/medium")(async function multipartMedium(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 2, total_bytes: 10240 };
});

post("/multipart/large")(async function multipartLarge(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

post("/urlencoded/simple")(async function urlencodedSimple(body: unknown): Promise<unknown> {
	return body;
});

post("/urlencoded/complex")(async function urlencodedComplex(body: unknown): Promise<unknown> {
	return body;
});

// ============================================================================
// Path Parameter Workloads
// ============================================================================

get("/path/simple/:id")(async function pathSimple(params: PathParams): Promise<PathResponse> {
	return { id: params.id ?? "" };
});

get("/path/multiple/:user_id/:post_id")(async function pathMultiple(params: PathParams): Promise<PathResponse> {
	return { user_id: params.user_id ?? "", post_id: params.post_id ?? "" };
});

get("/path/deep/:org/:team/:project/:api/:item")(async function pathDeep(params: PathParams): Promise<PathResponse> {
	return {
		org: params.org ?? "",
		team: params.team ?? "",
		project: params.project ?? "",
		api: params.api ?? "",
		item: params.item ?? "",
	};
});

get("/path/int/:id")(async function pathInt(params: PathParams): Promise<PathResponse> {
	return { id: Number.parseInt(params.id ?? "0", 10) };
});

get("/path/uuid/:id")(async function pathUuid(params: PathParams): Promise<PathResponse> {
	return { id: params.id ?? "" };
});

get("/path/date/:date")(async function pathDate(params: PathParams): Promise<PathResponse> {
	return { date: params.date ?? "" };
});

// ============================================================================
// Query Parameter Workloads
// ============================================================================

get("/query/few")(async function queryFew(query: unknown): Promise<unknown> {
	return query;
});

get("/query/medium")(async function queryMedium(query: unknown): Promise<unknown> {
	return query;
});

get("/query/many")(async function queryMany(query: unknown): Promise<unknown> {
	return query;
});

// ============================================================================
// Health Check
// ============================================================================

interface HealthResponse {
	readonly status: "ok";
}

get("/health")(async function health(): Promise<HealthResponse> {
	return { status: "ok" };
});

get("/")(async function root(): Promise<HealthResponse> {
	return { status: "ok" };
});

// Create the TestClient
const client = new TestClient(JSON.stringify(routes), handlers, undefined, undefined);

// Start HTTP server
const port: number = Deno.args[0] ? Number.parseInt(Deno.args[0], 10) : 8000;

console.log(`Starting Spikard WASM server on port ${port}`);

Deno.serve({ port }, async (req: Request): Promise<Response> => {
	try {
		const url = new URL(req.url);
		const method = req.method;
		const path = url.pathname;
		const query: QueryParams = Object.fromEntries(url.searchParams);

		let body: JsonBody | string | null = null;
		if (method === "POST" && req.body) {
			const contentType = req.headers.get("content-type") ?? "";
			if (contentType.includes("application/json")) {
				body = (await req.json()) as JsonBody;
			} else {
				body = await req.text();
			}
		}

		const requestPayload: ServerRequest = {
			method,
			path,
			query,
			headers: Object.fromEntries(req.headers),
			body,
		};

		const response = (await client.handle_request(JSON.stringify(requestPayload))) as ServerResponse;

		// Convert body from Uint8Array or array to string
		let bodyContent = "";
		if (response.body) {
			try {
				// Handle Uint8Array or regular array
				let bodyBytes: Uint8Array;
				if (response.body instanceof Uint8Array) {
					bodyBytes = response.body;
				} else if (Array.isArray(response.body)) {
					bodyBytes = new Uint8Array(response.body);
				} else if (response.body && typeof response.body === "object") {
					// Handle case where it's wrapped in an object with numeric keys
					const bodyObject = response.body as ServerResponseBody;
					const keys = Object.keys(bodyObject).filter((k) => !Number.isNaN(Number(k)));
					if (keys.length > 0) {
						bodyBytes = new Uint8Array(keys.length);
						for (let i = 0; i < keys.length; i++) {
							const value = bodyObject[String(i)];
							if (typeof value === "number") {
								bodyBytes[i] = value;
							}
						}
					} else {
						bodyBytes = new Uint8Array(0);
					}
				} else {
					bodyBytes = new Uint8Array(0);
				}

				if (bodyBytes.length > 0) {
					const decoder = new TextDecoder();
					bodyContent = decoder.decode(bodyBytes);
				}
			} catch (err) {
				// If decoding fails, log and continue
				console.error("Failed to decode body:", err);
			}
		}

		return new Response(bodyContent, {
			status: response.status ?? 200,
			headers: response.headers ?? { "content-type": "application/json" },
		});
	} catch (error) {
		return new Response(JSON.stringify({ error: String(error) }), {
			status: 500,
			headers: { "content-type": "application/json" },
		});
	}
});
