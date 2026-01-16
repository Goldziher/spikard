#!/usr/bin/env -S deno run --allow-net --allow-read
/**
 * Spikard WASM HTTP server for workload benchmarking using Deno.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline.
 */

import { dirname } from "node:path";
import * as wasm from "./pkg/spikard_wasm.js";

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

const metricsEnabled = getMetricsOutputPath() !== null;

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

if (metricsEnabled) {
	addEventListener("unload", () => {
		writeMetricsIfConfigured();
	});

	try {
		Deno.addSignalListener("SIGTERM", () => writeMetricsIfConfigured());
		Deno.addSignalListener("SIGINT", () => writeMetricsIfConfigured());
	} catch {
		// Signal listeners may be unsupported on some platforms.
	}
}

interface Route {
	readonly method: string;
	readonly path: string;
	readonly handler_name: string;
	readonly is_async: boolean;
	readonly request_schema?: unknown;
	readonly response_schema?: unknown;
	readonly parameter_schema?: unknown;
}

interface PathParams {
	readonly [key: string]: string;
}

interface RequestPayload {
	readonly method?: string;
	readonly path?: string;
	readonly body?: unknown;
	readonly query?: unknown;
	readonly pathParams?: PathParams;
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

const schemaDir = new URL("../schemas/", import.meta.url);
const requestSchemas = JSON.parse(Deno.readTextFileSync(new URL("request_schemas.json", schemaDir))) as Record<
	string,
	unknown
>;
const responseSchemas = JSON.parse(Deno.readTextFileSync(new URL("response_schemas.json", schemaDir))) as Record<
	string,
	unknown
>;
const parameterSchemas = JSON.parse(Deno.readTextFileSync(new URL("parameter_schemas.json", schemaDir))) as Record<
	string,
	unknown
>;

function requestSchema(key: string): unknown {
	return requestSchemas[key];
}

function responseSchema(key: string): unknown {
	return responseSchemas[key];
}

function parameterSchema(key: string): unknown {
	return parameterSchemas[key];
}

const SmallPayloadSchema = {
	type: "object",
	required: ["name", "description", "price", "tax"],
	properties: {
		name: { type: "string" },
		description: { type: "string" },
		price: { type: "number" },
		tax: { type: "number" },
	},
	additionalProperties: false,
} as const;

const MediumPayloadSchema = {
	type: "object",
	required: ["name", "price", "image"],
	properties: {
		name: { type: "string" },
		price: { type: "number" },
		image: {
			type: "object",
			required: ["url", "name"],
			properties: {
				url: { type: "string" },
				name: { type: "string" },
			},
			additionalProperties: false,
		},
	},
	additionalProperties: false,
} as const;

const LargePayloadSchema = {
	type: "object",
	required: ["name", "price", "seller"],
	properties: {
		name: { type: "string" },
		price: { type: "number" },
		seller: {
			type: "object",
			required: ["name", "address"],
			properties: {
				name: { type: "string" },
				address: {
					type: "object",
					required: ["street", "city", "country"],
					properties: {
						street: { type: "string" },
						city: { type: "string" },
						country: {
							type: "object",
							required: ["name", "code"],
							properties: {
								name: { type: "string" },
								code: { type: "string" },
							},
							additionalProperties: false,
						},
					},
					additionalProperties: false,
				},
			},
			additionalProperties: false,
		},
	},
	additionalProperties: false,
} as const;

const VeryLargePayloadSchema = {
	type: "object",
	required: ["name", "tags", "images"],
	properties: {
		name: { type: "string" },
		tags: { type: "array", items: { type: "string" } },
		images: {
			type: "array",
			items: {
				type: "object",
				required: ["url", "name"],
				properties: {
					url: { type: "string" },
					name: { type: "string" },
				},
				additionalProperties: false,
			},
		},
	},
	additionalProperties: false,
} as const;

const UrlencodedSimpleSchema = {
	type: "object",
	required: ["name", "email", "age", "subscribe"],
	properties: {
		name: { type: "string" },
		email: { type: "string", format: "email" },
		age: { type: "integer" },
		subscribe: { type: "boolean" },
	},
	additionalProperties: false,
} as const;

const UrlencodedComplexSchema = {
	type: "object",
	required: [
		"username",
		"password",
		"email",
		"first_name",
		"last_name",
		"age",
		"country",
		"state",
		"city",
		"zip",
		"phone",
		"company",
		"job_title",
		"subscribe",
		"newsletter",
		"terms_accepted",
		"privacy_accepted",
		"marketing_consent",
		"two_factor_enabled",
	],
	properties: {
		username: { type: "string" },
		password: { type: "string" },
		email: { type: "string", format: "email" },
		first_name: { type: "string" },
		last_name: { type: "string" },
		age: { type: "integer" },
		country: { type: "string" },
		state: { type: "string" },
		city: { type: "string" },
		zip: { type: "string" },
		phone: { type: "string" },
		company: { type: "string" },
		job_title: { type: "string" },
		subscribe: { type: "boolean" },
		newsletter: { type: "boolean" },
		terms_accepted: { type: "boolean" },
		privacy_accepted: { type: "boolean" },
		marketing_consent: { type: "boolean" },
		two_factor_enabled: { type: "boolean" },
	},
	additionalProperties: false,
} as const;

const MultipartFileSchema = {
	type: "object",
	required: ["filename", "size", "content", "content_type"],
	properties: {
		filename: { type: "string" },
		size: { type: "integer" },
		content: { type: "string" },
		content_type: { type: "string" },
	},
	additionalProperties: false,
} as const;

const MultipartSchema = {
	type: "object",
	required: ["file"],
	properties: {
		file: {
			oneOf: [
				MultipartFileSchema,
				{
					type: "array",
					items: MultipartFileSchema,
				},
			],
		},
	},
	additionalProperties: false,
} as const;

const PathSimpleParamSchema = {
	type: "object",
	properties: {
		id: { type: "string", source: "path" },
	},
	required: ["id"],
} as const;

const PathMultipleParamSchema = {
	type: "object",
	properties: {
		user_id: { type: "string", source: "path" },
		post_id: { type: "string", source: "path" },
	},
	required: ["user_id", "post_id"],
} as const;

const PathDeepParamSchema = {
	type: "object",
	properties: {
		org: { type: "string", source: "path" },
		team: { type: "string", source: "path" },
		project: { type: "string", source: "path" },
		resource: { type: "string", source: "path" },
		id: { type: "string", source: "path" },
	},
	required: ["org", "team", "project", "resource", "id"],
} as const;

const PathIntParamSchema = {
	type: "object",
	properties: {
		id: { type: "integer", source: "path" },
	},
	required: ["id"],
} as const;

const PathUuidParamSchema = {
	type: "object",
	properties: {
		uuid: { type: "string", format: "uuid", source: "path" },
	},
	required: ["uuid"],
} as const;

const PathDateParamSchema = {
	type: "object",
	properties: {
		date: { type: "string", format: "date", source: "path" },
	},
	required: ["date"],
} as const;

const QueryFewParamSchema = {
	type: "object",
	properties: {
		q: { type: "string", source: "query" },
		page: { type: "integer", source: "query" },
		limit: { type: "integer", source: "query" },
	},
	required: ["q", "page", "limit"],
} as const;

const QueryMediumParamSchema = {
	type: "object",
	properties: {
		category: { type: "string", source: "query" },
		tags: { type: "string", source: "query" },
		min_price: { type: "number", source: "query" },
		max_price: { type: "number", source: "query" },
		sort: { type: "string", source: "query" },
		order: { type: "string", source: "query" },
		page: { type: "integer", source: "query" },
		limit: { type: "integer", source: "query" },
	},
	required: ["category", "tags", "min_price", "max_price", "sort", "order", "page", "limit"],
} as const;

const QueryManyParamSchema = {
	type: "object",
	properties: {
		q: { type: "string", source: "query" },
		page: { type: "integer", source: "query" },
		limit: { type: "integer", source: "query" },
		sort: { type: "string", source: "query" },
		order: { type: "string", source: "query" },
		filter: { type: "string", source: "query" },
		category: { type: "string", source: "query" },
		subcategory: { type: "string", source: "query" },
		brand: { type: "string", source: "query" },
		min_price: { type: "number", source: "query" },
		max_price: { type: "number", source: "query" },
		rating: { type: "integer", source: "query" },
		verified: { type: "boolean", source: "query" },
		in_stock: { type: "boolean", source: "query" },
		shipping: { type: "string", source: "query" },
		color: { type: "string", source: "query" },
	},
	required: [
		"q",
		"page",
		"limit",
		"sort",
		"order",
		"filter",
		"category",
		"subcategory",
		"brand",
		"min_price",
		"max_price",
		"rating",
		"verified",
		"in_stock",
		"shipping",
		"color",
	],
} as const;

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

function registerRoute(
	method: string,
	path: string,
	handler: HandlerFunction,
	requestSchema?: unknown,
	responseSchema?: unknown,
	parameterSchema?: unknown,
): void {
	routes.push({
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
		request_schema: requestSchema,
		response_schema: responseSchema,
		parameter_schema: parameterSchema,
	});
	handlers[handler.name] = handler;
}

function get(
	path: string,
	responseSchemaValue?: unknown,
	parameterSchemaValue?: unknown,
): (handler: HandlerFunction) => void {
	return (handler: HandlerFunction): void =>
		registerRoute("GET", path, handler, undefined, responseSchemaValue, parameterSchemaValue);
}

function post(
	path: string,
	requestSchemaValue?: unknown,
	responseSchemaValue?: unknown,
): (handler: HandlerFunction) => void {
	return (handler: HandlerFunction): void =>
		registerRoute("POST", path, handler, requestSchemaValue, responseSchemaValue);
}

function isRequestPayload(value: unknown): value is RequestPayload {
	return Boolean(value && typeof value === "object" && "method" in value && "path" in value);
}

function extractBody(value: unknown): unknown {
	const bodyValue = value && typeof value === "object" && "body" in value ? (value as { body?: unknown }).body : value;
	if (!bodyValue || typeof bodyValue !== "object") {
		return bodyValue;
	}
	const record = bodyValue as Record<string, unknown>;
	if (record.__spikard_form__ && typeof record.__spikard_form__ === "object") {
		return record.__spikard_form__;
	}
	if (record.__spikard_multipart__ && typeof record.__spikard_multipart__ === "object") {
		const multipart = record.__spikard_multipart__ as { fields?: unknown };
		return multipart.fields ?? {};
	}
	return bodyValue;
}

function coerceQueryValue(value: unknown): unknown {
	if (Array.isArray(value)) {
		return value.map((entry) => coerceQueryValue(entry));
	}
	if (value && typeof value === "object") {
		const record = value as Record<string, unknown>;
		const output: Record<string, unknown> = {};
		for (const [key, entry] of Object.entries(record)) {
			output[key] = coerceQueryValue(entry);
		}
		return output;
	}
	if (typeof value !== "string") {
		return value;
	}
	const lowered = value.toLowerCase();
	if (lowered === "true") {
		return true;
	}
	if (lowered === "false") {
		return false;
	}
	if (!value.trim()) {
		return value;
	}
	const numeric = Number(value);
	if (!Number.isNaN(numeric)) {
		return value.includes(".") ? numeric : Number.parseInt(value, 10);
	}
	return value;
}

function extractQuery(value: unknown): unknown {
	if (isRequestPayload(value)) {
		return coerceQueryValue(value.query ?? {});
	}
	return coerceQueryValue(value);
}

function extractPathParams(value: unknown): PathParams {
	if (isRequestPayload(value)) {
		return value.pathParams ?? {};
	}
	return (value ?? {}) as PathParams;
}

post(
	"/json/small",
	requestSchema("json/small"),
	responseSchema("json/small"),
)(async function jsonSmall(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/json/medium",
	requestSchema("json/medium"),
	responseSchema("json/medium"),
)(async function jsonMedium(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/json/large",
	requestSchema("json/large"),
	responseSchema("json/large"),
)(async function jsonLarge(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/json/very-large",
	requestSchema("json/very-large"),
	responseSchema("json/very-large"),
)(async function jsonVeryLarge(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/multipart/small",
	requestSchema("multipart/small"),
	responseSchema("multipart/small"),
)(async function multipartSmall(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 1, total_bytes: 1024 };
});

post(
	"/multipart/medium",
	requestSchema("multipart/medium"),
	responseSchema("multipart/medium"),
)(async function multipartMedium(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 2, total_bytes: 10240 };
});

post(
	"/multipart/large",
	requestSchema("multipart/large"),
	responseSchema("multipart/large"),
)(async function multipartLarge(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 5, total_bytes: 102400 };
});

post(
	"/urlencoded/simple",
	requestSchema("urlencoded/simple"),
	responseSchema("urlencoded/simple"),
)(async function urlencodedSimple(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/urlencoded/complex",
	requestSchema("urlencoded/complex"),
	responseSchema("urlencoded/complex"),
)(async function urlencodedComplex(body: unknown): Promise<unknown> {
	return extractBody(body);
});

get(
	"/path/simple/{id}",
	responseSchema("path/simple"),
	parameterSchema("path/simple"),
)(async function pathSimple(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { id: pathParams.id ?? "" };
});

get(
	"/path/multiple/{user_id}/{post_id}",
	responseSchema("path/multiple"),
	parameterSchema("path/multiple"),
)(async function pathMultiple(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { user_id: pathParams.user_id ?? "", post_id: pathParams.post_id ?? "" };
});

get(
	"/path/deep/{org}/{team}/{project}/{resource}/{id}",
	responseSchema("path/deep"),
	parameterSchema("path/deep"),
)(async function pathDeep(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return {
		org: pathParams.org ?? "",
		team: pathParams.team ?? "",
		project: pathParams.project ?? "",
		resource: pathParams.resource ?? "",
		id: pathParams.id ?? "",
	};
});

get(
	"/path/int/{id}",
	responseSchema("path/int"),
	parameterSchema("path/int"),
)(async function pathInt(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { id: Number.parseInt(pathParams.id ?? "0", 10) };
});

get(
	"/path/uuid/{uuid}",
	responseSchema("path/uuid"),
	parameterSchema("path/uuid"),
)(async function pathUuid(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { uuid: pathParams.uuid ?? "" };
});

get(
	"/path/date/{date}",
	responseSchema("path/date"),
	parameterSchema("path/date"),
)(async function pathDate(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { date: pathParams.date ?? "" };
});

get(
	"/query/few",
	responseSchema("query/few"),
	parameterSchema("query/few"),
)(async function queryFew(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

get(
	"/query/medium",
	responseSchema("query/medium"),
	parameterSchema("query/medium"),
)(async function queryMedium(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

get(
	"/query/many",
	responseSchema("query/many"),
	parameterSchema("query/many"),
)(async function queryMany(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

interface HealthResponse {
	readonly status: "ok";
}

get(
	"/health",
	responseSchema("health"),
)(async function health(): Promise<HealthResponse> {
	return { status: "ok" };
});

get(
	"/",
	responseSchema("root"),
)(async function root(): Promise<HealthResponse> {
	return { status: "ok" };
});

const client = new TestClient(JSON.stringify(routes), handlers, undefined, undefined);

const port: number = Deno.args[0] ? Number.parseInt(Deno.args[0], 10) : 8000;

console.log(`Starting Spikard WASM server on port ${port}`);

Deno.serve({ port }, async (req: Request): Promise<Response> => {
	try {
		const method = req.method;
		const urlValue = req.url;
		const schemeIndex = urlValue.indexOf("://");
		const pathStart = schemeIndex === -1 ? 0 : urlValue.indexOf("/", schemeIndex + 3);
		const pathIndex = pathStart === -1 ? 0 : pathStart;
		const queryIndex = urlValue.indexOf("?", pathIndex);
		const pathname = queryIndex === -1 ? urlValue.slice(pathIndex) : urlValue.slice(pathIndex, queryIndex);
		const search = queryIndex === -1 ? "" : urlValue.slice(queryIndex);
		const pathWithQuery = `${pathname}${search}`;
		const isUrlencodedRoute = pathname.startsWith("/urlencoded/");

		let response: ServerResponse;
		if (method === "GET") {
			response = (await client.get(pathWithQuery, {})) as ServerResponse;
		} else if (method === "POST") {
			const contentType = req.headers.get("content-type") ?? "";
			if (req.body && contentType.includes("application/json") && !isUrlencodedRoute) {
				const jsonBody = (await req.json()) as JsonBody;
				response = (await client.post(pathWithQuery, {
					json: jsonBody,
					headers: { "content-type": contentType },
				})) as ServerResponse;
			} else if (req.body && contentType.includes("multipart/form-data")) {
				// Multipart parsing in Deno is extremely expensive and dwarfs WASM binding overhead.
				// These benchmark endpoints intentionally ignore multipart bodies (matching Robyn),
				// so just drain the request body and forward the request without parsing.
				await req.arrayBuffer();
				response = (await client.post(pathWithQuery, { headers: { "content-type": contentType } })) as ServerResponse;
			} else if (req.body) {
				const formRawBody = await req.text();
				response = (await client.post(pathWithQuery, {
					formRaw: formRawBody,
					headers: { "content-type": contentType },
				})) as ServerResponse;
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
