#!/usr/bin/env -S deno run --allow-net --allow-read
/**
 * Spikard WASM HTTP server for workload benchmarking using Deno.
 *
 * This server implements all workload types to measure WASM binding performance
 * against the pure Rust baseline. It includes both raw endpoints and validated
 * endpoints at /validated/ prefix.
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

// Load schemas for validated endpoints
const schemaDir = new URL("../schemas/", import.meta.url);
let requestSchemas: Record<string, unknown> = {};
let responseSchemas: Record<string, unknown> = {};
let parameterSchemas: Record<string, unknown> = {};

try {
	requestSchemas = JSON.parse(Deno.readTextFileSync(new URL("request_schemas.json", schemaDir))) as Record<
		string,
		unknown
	>;
	responseSchemas = JSON.parse(Deno.readTextFileSync(new URL("response_schemas.json", schemaDir))) as Record<
		string,
		unknown
	>;
	parameterSchemas = JSON.parse(Deno.readTextFileSync(new URL("parameter_schemas.json", schemaDir))) as Record<
		string,
		unknown
	>;
} catch {
	console.warn("Failed to load schemas from ../schemas/ - validation endpoints will not include schemas");
}

function requestSchema(key: string): unknown {
	return requestSchemas[key];
}

function responseSchema(key: string): unknown {
	return responseSchemas[key];
}

function parameterSchema(key: string): unknown {
	return parameterSchemas[key];
}

const _SmallPayloadSchema = {
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

const _MediumPayloadSchema = {
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

const _LargePayloadSchema = {
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

const _VeryLargePayloadSchema = {
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

const _UrlencodedSimpleSchema = {
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

const _UrlencodedComplexSchema = {
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

const _MultipartSchema = {
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

const _PathUuidParamSchema = {
	type: "object",
	properties: {
		uuid: { type: "string", format: "uuid", source: "path" },
	},
	required: ["uuid"],
} as const;

const _PathDateParamSchema = {
	type: "object",
	properties: {
		date: { type: "string", format: "date", source: "path" },
	},
	required: ["date"],
} as const;

const _QueryFewParamSchema = {
	type: "object",
	properties: {
		q: { type: "string", source: "query" },
		page: { type: "integer", source: "query" },
		limit: { type: "integer", source: "query" },
	},
	required: ["q", "page", "limit"],
} as const;

const _QueryMediumParamSchema = {
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

const _QueryManyParamSchema = {
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
	requestSchemaValue?: unknown,
	responseSchemaValue?: unknown,
	parameterSchemaValue?: unknown,
): void {
	routes.push({
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
		request_schema: requestSchemaValue,
		response_schema: responseSchemaValue,
		parameter_schema: parameterSchemaValue,
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

async function buildMultipartOptions(req: Request): Promise<{
	fields: Record<string, string>;
	files: Array<{ name: string; filename?: string; content: string; contentType?: string }>;
}> {
	const formData = await req.formData();
	const fields: Record<string, string> = {};
	const files: Array<{ name: string; filename?: string; content: string; contentType?: string }> = [];

	for (const [key, value] of formData.entries()) {
		if (typeof value === "string") {
			fields[key] = value;
			continue;
		}

		files.push({
			name: key,
			filename: value.name || undefined,
			content: "",
			contentType: value.type || undefined,
		});
	}

	return { fields, files };
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

// ============================================================================
// RAW ENDPOINTS (no validation)
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

get("/path/deep/{org}/{team}/{project}/{resource}/{id}")(async function pathDeep(
	params: PathParams,
): Promise<PathResponse> {
	return {
		org: params.org ?? "",
		team: params.team ?? "",
		project: params.project ?? "",
		resource: params.resource ?? "",
		id: params.id ?? "",
	};
});

get("/path/int/{id}")(async function pathInt(params: PathParams): Promise<PathResponse> {
	return { id: Number.parseInt(params.id ?? "0", 10) };
});

get("/path/uuid/{uuid}")(async function pathUuid(params: PathParams): Promise<PathResponse> {
	return { uuid: params.uuid ?? "" };
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

// ============================================================================
// VALIDATED ENDPOINTS (with schema validation at /validated/ prefix)
// ============================================================================

post(
	"/validated/json/small",
	requestSchema("json/small"),
	responseSchema("json/small"),
)(async function validatedJsonSmall(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/validated/json/medium",
	requestSchema("json/medium"),
	responseSchema("json/medium"),
)(async function validatedJsonMedium(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/validated/json/large",
	requestSchema("json/large"),
	responseSchema("json/large"),
)(async function validatedJsonLarge(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/validated/json/very-large",
	requestSchema("json/very-large"),
	responseSchema("json/very-large"),
)(async function validatedJsonVeryLarge(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/validated/multipart/small",
	requestSchema("multipart/small"),
	responseSchema("multipart/small"),
)(async function validatedMultipartSmall(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 1, total_bytes: 1024 };
});

post(
	"/validated/multipart/medium",
	requestSchema("multipart/medium"),
	responseSchema("multipart/medium"),
)(async function validatedMultipartMedium(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 2, total_bytes: 10240 };
});

post(
	"/validated/multipart/large",
	requestSchema("multipart/large"),
	responseSchema("multipart/large"),
)(async function validatedMultipartLarge(_body: unknown): Promise<MultipartResponse> {
	return { files_received: 5, total_bytes: 102400 };
});

post(
	"/validated/urlencoded/simple",
	requestSchema("urlencoded/simple"),
	responseSchema("urlencoded/simple"),
)(async function validatedUrlencodedSimple(body: unknown): Promise<unknown> {
	return extractBody(body);
});

post(
	"/validated/urlencoded/complex",
	requestSchema("urlencoded/complex"),
	responseSchema("urlencoded/complex"),
)(async function validatedUrlencodedComplex(body: unknown): Promise<unknown> {
	return extractBody(body);
});

get(
	"/validated/path/simple/{id}",
	responseSchema("path/simple"),
	parameterSchema("path/simple"),
)(async function validatedPathSimple(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { id: pathParams.id ?? "" };
});

get(
	"/validated/path/multiple/{user_id}/{post_id}",
	responseSchema("path/multiple"),
	parameterSchema("path/multiple"),
)(async function validatedPathMultiple(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { user_id: pathParams.user_id ?? "", post_id: pathParams.post_id ?? "" };
});

get(
	"/validated/path/deep/{org}/{team}/{project}/{resource}/{id}",
	responseSchema("path/deep"),
	parameterSchema("path/deep"),
)(async function validatedPathDeep(params: PathParams): Promise<PathResponse> {
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
	"/validated/path/int/{id}",
	responseSchema("path/int"),
	parameterSchema("path/int"),
)(async function validatedPathInt(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { id: Number.parseInt(pathParams.id ?? "0", 10) };
});

get(
	"/validated/path/uuid/{uuid}",
	responseSchema("path/uuid"),
	parameterSchema("path/uuid"),
)(async function validatedPathUuid(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { uuid: pathParams.uuid ?? "" };
});

get(
	"/validated/path/date/{date}",
	responseSchema("path/date"),
	parameterSchema("path/date"),
)(async function validatedPathDate(params: PathParams): Promise<PathResponse> {
	const pathParams = extractPathParams(params);
	return { date: pathParams.date ?? "" };
});

get(
	"/validated/query/few",
	responseSchema("query/few"),
	parameterSchema("query/few"),
)(async function validatedQueryFew(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

get(
	"/validated/query/medium",
	responseSchema("query/medium"),
	parameterSchema("query/medium"),
)(async function validatedQueryMedium(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

get(
	"/validated/query/many",
	responseSchema("query/many"),
	parameterSchema("query/many"),
)(async function validatedQueryMany(query: unknown): Promise<unknown> {
	return extractQuery(query);
});

get(
	"/validated/health",
	responseSchema("health"),
)(async function validatedHealth(): Promise<HealthResponse> {
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
		const isUrlencodedRoute = pathname.startsWith("/urlencoded/") || pathname.startsWith("/validated/urlencoded/");

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
				const multipart = await buildMultipartOptions(req);
				response = (await client.post(pathWithQuery, {
					multipart,
					headers: { "content-type": contentType },
				})) as ServerResponse;
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
		} else if (typeof response.body === "object") {
			const record = response.body as Record<string, unknown>;
			if ("0" in record) {
				const values = Object.values(record);
				if (values.length > 0 && values.every((value) => typeof value === "number")) {
					bodyBytes = Uint8Array.from(values as number[]);
				} else {
					bodyBytes = new TextEncoder().encode(JSON.stringify(response.body));
				}
			} else {
				bodyBytes = new TextEncoder().encode(JSON.stringify(response.body));
			}
		} else {
			bodyBytes = new TextEncoder().encode(String(response.body));
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
