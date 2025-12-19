#!/usr/bin/env -S deno run --allow-net --allow-read
/**
 * Spikard WASM HTTP server for workload benchmarking using Deno.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline.
 */

import { dirname } from "node:path";
import * as wasm from "../../../../crates/spikard-wasm/dist-web/spikard_wasm.js";

if (typeof (wasm as { default?: unknown }).default === "function") {
	await (wasm as { default: () => Promise<unknown> }).default();
}
if (typeof (wasm as { init?: unknown }).init === "function") {
	(wasm as { init: () => unknown }).init();
}

const TestClient = (wasm as { TestClient: typeof wasm.TestClient }).TestClient;

interface WasmMetricsFile {
	readonly rss_mb: number;
	readonly heap_total_mb: number;
	readonly heap_used_mb: number;
	readonly external_mb: number;
}

function bytesToMb(value: number): number {
	return value / 1024 / 1024;
}

function getMetricsOutputPath(): string | null {
	try {
		return Deno.env.get("SPIKARD_WASM_METRICS_FILE") ?? null;
	} catch {
		return null;
	}
}

function writeMetricsIfConfigured(): void {
	const outputPath = getMetricsOutputPath();
	if (!outputPath) {
		return;
	}

	try {
		const usage = Deno.memoryUsage();
		const payload: WasmMetricsFile = {
			rss_mb: bytesToMb(usage.rss),
			heap_total_mb: bytesToMb(usage.heapTotal),
			heap_used_mb: bytesToMb(usage.heapUsed),
			external_mb: bytesToMb(usage.external),
		};
		Deno.mkdirSync(dirname(outputPath), { recursive: true });
		Deno.writeTextFileSync(outputPath, JSON.stringify(payload));
	} catch (err) {
		console.error("Failed to write WASM metrics:", err);
	}
}

addEventListener("unload", () => {
	writeMetricsIfConfigured();
});

try {
	Deno.addSignalListener("SIGTERM", () => writeMetricsIfConfigured());
	Deno.addSignalListener("SIGINT", () => writeMetricsIfConfigured());
} catch {
	// Signal listeners may be unsupported on some platforms.
}

interface Route {
	readonly method: string;
	readonly path: string;
	readonly handler_name: string;
	readonly is_async: boolean;
}

interface PathParams {
	readonly [key: string]: string;
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

interface MultipartFile {
	readonly name: string;
	readonly filename?: string;
	readonly content: string;
	readonly contentType?: string;
	readonly size?: number;
}

interface MultipartOptions {
	readonly fields: Record<string, unknown>;
	readonly files: readonly MultipartFile[];
}

interface ServerRequest {
	readonly method: string;
	readonly path: string;
	readonly headers: Record<string, string>;
	readonly body: unknown;
}

interface ServerResponse {
	readonly status?: number;
	readonly headers?: Record<string, string>;
	readonly body?: Uint8Array | readonly number[] | ServerResponseBody | null;
}

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

post("/multipart/small")(async function multipartSmall(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 1, total_bytes: 1024 };
});

post("/multipart/medium")(async function multipartMedium(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 2, total_bytes: 10240 };
});

post("/multipart/large")(async function multipartLarge(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 5, total_bytes: 102400 };
});

post("/urlencoded/simple")(async function urlencodedSimple(body: unknown): Promise<unknown> {
	return body;
});

post("/urlencoded/complex")(async function urlencodedComplex(body: unknown): Promise<unknown> {
	return body;
});

get("/path/simple/{id}")(async function pathSimple(params: PathParams): Promise<PathResponse> {
	return { id: params.id ?? "" };
});

get("/path/multiple/{user_id}/{post_id}")(async function pathMultiple(params: PathParams): Promise<PathResponse> {
	return { user_id: params.user_id ?? "", post_id: params.post_id ?? "" };
});

get("/path/deep/{org}/{team}/{project}/{api}/{item}")(async function pathDeep(params: PathParams): Promise<PathResponse> {
	return {
		org: params.org ?? "",
		team: params.team ?? "",
		project: params.project ?? "",
		api: params.api ?? "",
		item: params.item ?? "",
	};
});

get("/path/int/{id}")(async function pathInt(params: PathParams): Promise<PathResponse> {
	return { id: Number.parseInt(params.id ?? "0", 10) };
});

get("/path/uuid/{id}")(async function pathUuid(params: PathParams): Promise<PathResponse> {
	return { id: params.id ?? "" };
});

get("/path/date/{date}")(async function pathDate(params: PathParams): Promise<PathResponse> {
	return { date: params.date ?? "" };
});

get("/query/few")(async function queryFew(query: unknown): Promise<unknown> {
	return query;
});

get("/query/medium")(async function queryMedium(query: unknown): Promise<unknown> {
	return query;
});

get("/query/many")(async function queryMany(query: unknown): Promise<unknown> {
	return query;
});

interface HealthResponse {
	readonly status: "ok";
}

get("/health")(async function health(): Promise<HealthResponse> {
	return { status: "ok" };
});

get("/")(async function root(): Promise<HealthResponse> {
	return { status: "ok" };
});

const client = new TestClient(JSON.stringify(routes), handlers, undefined, undefined);

const port: number = Deno.args[0] ? Number.parseInt(Deno.args[0], 10) : 8000;

console.log(`Starting Spikard WASM server on port ${port}`);

Deno.serve({ port }, async (req: Request): Promise<Response> => {
	try {
		const url = new URL(req.url);
		const method = req.method;
		const pathWithQuery = `${url.pathname}${url.search}`;
		const isUrlencodedRoute = url.pathname.startsWith("/urlencoded/");

		let response: ServerResponse;
		if (method === "GET") {
			response = (await client.get(pathWithQuery, {})) as ServerResponse;
		} else if (method === "POST") {
			const contentType = req.headers.get("content-type") ?? "";
			if (req.body && contentType.includes("application/json") && !isUrlencodedRoute) {
				const jsonBody = (await req.json()) as JsonBody;
				response = (await client.post(pathWithQuery, { json: jsonBody, headers: { "content-type": contentType } })) as ServerResponse;
			} else if (req.body && contentType.includes("multipart/form-data")) {
				const formData = await req.formData();
				const fields: Record<string, unknown> = {};
				const files: MultipartFile[] = [];

				for (const [name, value] of formData.entries()) {
					if (typeof value === "string") {
						fields[name] = value;
						continue;
					}
					if (value instanceof File) {
						files.push({
							name,
							filename: value.name,
							content: "",
							contentType: value.type || undefined,
							size: value.size,
						});
					}
				}

				const multipart: MultipartOptions = { fields, files };
				response = (await client.post(pathWithQuery, { multipart, headers: { "content-type": contentType } })) as ServerResponse;
			} else if (req.body) {
				const formRawBody = await req.text();
				response = (await client.post(pathWithQuery, { formRaw: formRawBody, headers: { "content-type": contentType } })) as ServerResponse;
			} else {
				response = (await client.post(pathWithQuery, {})) as ServerResponse;
			}
		} else {
			response = (await client.handle_request(
				JSON.stringify({
					method,
					path: pathWithQuery,
					headers: {},
					body: null,
				} satisfies Omit<ServerRequest, "body"> & { readonly body: null }),
			)) as ServerResponse;
		}
		let bodyBytes: Uint8Array | null = null;
		if (response.body instanceof Uint8Array) {
			bodyBytes = response.body;
		} else if (response.body && typeof response.body === "object" && ArrayBuffer.isView(response.body)) {
			const view = response.body as ArrayBufferView;
			bodyBytes = new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
		} else if (response.body instanceof ArrayBuffer) {
			bodyBytes = new Uint8Array(response.body);
		} else if (Array.isArray(response.body)) {
			bodyBytes = new Uint8Array(response.body);
		} else if (response.body == null) {
			bodyBytes = null;
		} else {
			const record = response.body as Record<string, unknown>;
			if ("0" in record) {
				const values = Object.values(record);
				if (values.length > 0 && values.every((value) => typeof value === "number")) {
					bodyBytes = Uint8Array.from(values as number[]);
				} else {
					bodyBytes = new Uint8Array(0);
				}
			} else {
				bodyBytes = new Uint8Array(0);
			}
		}

		return new Response(bodyBytes, {
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
