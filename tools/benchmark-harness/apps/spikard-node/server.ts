#!/usr/bin/env node

/**
 * Spikard Node HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Node.js runtime binding performance.
 * Serves both raw endpoints (no validation) and validated endpoints (at /validated/... paths).
 */

import { createRequire } from "node:module";
import process from "node:process";
import { readFileSync } from "node:fs";
import { writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const native = require("../../../../packages/node/index.js") as {
	runServer: (app: unknown, config: unknown) => void;
};

type RouteMetadata = {
	method: string;
	path: string;
	handler_name: string;
	is_async: boolean;
	request_schema?: unknown;
	response_schema?: unknown;
	parameter_schema?: unknown;
	file_params?: unknown;
	cors?: unknown;
	body_param_name?: string;
};

type HandlerInput = {
	method: string;
	path: string;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	queryParams: unknown;
	body: unknown;
	pathParams: Record<string, string>;
};

type HandlerOutput = {
	status: number;
	headers?: Record<string, string>;
	body?: unknown;
};

type HandlerFunction = (input: HandlerInput) => Promise<HandlerOutput>;

const routes: RouteMetadata[] = [];
const handlers: Record<string, HandlerFunction> = {};

const schemaDir = join(dirname(fileURLToPath(import.meta.url)), "..", "schemas");
const requestSchemas = JSON.parse(readFileSync(join(schemaDir, "request_schemas.json"), "utf8")) as Record<
	string,
	unknown
>;
const parameterSchemas = JSON.parse(readFileSync(join(schemaDir, "parameter_schemas.json"), "utf8")) as Record<
	string,
	unknown
>;
const responseSchemas = JSON.parse(readFileSync(join(schemaDir, "response_schemas.json"), "utf8")) as Record<
	string,
	unknown
>;

function requestSchema(key: string): unknown {
	return requestSchemas[key];
}

function parameterSchema(key: string): unknown {
	return parameterSchemas[key];
}

function responseSchema(key: string): unknown {
	return responseSchemas[key];
}

type NodeMetricsSnapshot = {
	heap_used_mb: number;
	gc_time_ms: number | null;
};

const profilingEnabled = process.env.SPIKARD_PROFILE_ENABLED === "1" || Boolean(process.env.SPIKARD_NODE_METRICS_FILE);

function resolveMetricsPath(): string {
	const envPath = process.env.SPIKARD_NODE_METRICS_FILE;
	return envPath && envPath.length > 0 ? envPath : `/tmp/node-metrics-${process.pid}.json`;
}

function startMetricsCollector(): void {
	const flush = async (): Promise<void> => {
		try {
			const heap = process.memoryUsage();
			const snapshot: NodeMetricsSnapshot = {
				heap_used_mb: heap.heapUsed / (1024 * 1024),
				gc_time_ms: null,
			};
			await writeFile(resolveMetricsPath(), JSON.stringify(snapshot, null, 2), "utf8");
		} catch {
			// Best-effort only.
		}
	};

	process.once("SIGTERM", () => {
		void flush().finally(() => process.exit(0));
	});
	process.once("SIGINT", () => {
		void flush().finally(() => process.exit(0));
	});
	process.once("beforeExit", () => {
		void flush();
	});
}

if (profilingEnabled) {
	startMetricsCollector();
}

function registerRoute(
	method: string,
	path: string,
	handler: HandlerFunction,
	requestSchemaValue?: unknown,
	responseSchemaValue?: unknown,
	parameterSchemaValue?: unknown,
): void {
	const metadata: RouteMetadata = {
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
		request_schema: requestSchemaValue,
		response_schema: responseSchemaValue,
		parameter_schema: parameterSchemaValue,
	};
	routes.push(metadata);
	handlers[handler.name] = async (input: HandlerInput) => {
		try {
			return await handler(input);
		} catch (error) {
			return {
				status: 500,
				body: {
					error: "handler_exception",
					code: "handler_exception",
					details: {
						message: error instanceof Error ? error.message : String(error),
					},
				},
			};
		}
	};
}

function get(
	path: string,
	handler: HandlerFunction,
	responseSchemaValue?: unknown,
	parameterSchemaValue?: unknown,
): void {
	registerRoute("GET", path, handler, undefined, responseSchemaValue, parameterSchemaValue);
}

function post(
	path: string,
	handler: HandlerFunction,
	requestSchemaValue?: unknown,
	responseSchemaValue?: unknown,
	parameterSchemaValue?: unknown,
): void {
	registerRoute("POST", path, handler, requestSchemaValue, responseSchemaValue, parameterSchemaValue);
}

function ok(body: unknown): HandlerOutput {
	return { status: 200, body };
}

async function post_json_small(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body);
}

async function post_json_medium(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body);
}

async function post_json_large(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body);
}

async function post_json_very_large(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body);
}

async function post_multipart_small(request: HandlerInput): Promise<HandlerOutput> {
	const body = request.body;
	if (!body || typeof body !== "object" || !("files" in body)) {
		return ok({ files_received: 0, total_bytes: 0 });
	}

	const files = body.files as Record<string, { size: number }>;
	let files_received = 0;
	let total_bytes = 0;

	for (const key in files) {
		if (key.startsWith("file")) {
			files_received++;
			total_bytes += files[key].size || 0;
		}
	}

	return ok({ files_received, total_bytes });
}

async function post_multipart_medium(request: HandlerInput): Promise<HandlerOutput> {
	const body = request.body;
	if (!body || typeof body !== "object" || !("files" in body)) {
		return ok({ files_received: 0, total_bytes: 0 });
	}

	const files = body.files as Record<string, { size: number }>;
	let files_received = 0;
	let total_bytes = 0;

	for (const key in files) {
		if (key.startsWith("file")) {
			files_received++;
			total_bytes += files[key].size || 0;
		}
	}

	return ok({ files_received, total_bytes });
}

async function post_multipart_large(request: HandlerInput): Promise<HandlerOutput> {
	const body = request.body;
	if (!body || typeof body !== "object" || !("files" in body)) {
		return ok({ files_received: 0, total_bytes: 0 });
	}

	const files = body.files as Record<string, { size: number }>;
	let files_received = 0;
	let total_bytes = 0;

	for (const key in files) {
		if (key.startsWith("file")) {
			files_received++;
			total_bytes += files[key].size || 0;
		}
	}

	return ok({ files_received, total_bytes });
}

async function post_urlencoded_simple(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body ?? {});
}

async function post_urlencoded_complex(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body ?? {});
}

async function get_path_simple(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ id: request.pathParams.id });
}

async function get_path_multiple(request: HandlerInput): Promise<HandlerOutput> {
	return ok({
		user_id: request.pathParams.user_id,
		post_id: request.pathParams.post_id,
	});
}

async function get_path_deep(request: HandlerInput): Promise<HandlerOutput> {
	return ok({
		org: request.pathParams.org,
		team: request.pathParams.team,
		project: request.pathParams.project,
		resource: request.pathParams.resource,
		id: request.pathParams.id,
	});
}

async function get_path_int(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ id: parseInt(request.pathParams.id, 10) });
}

async function get_path_uuid(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ uuid: request.pathParams.uuid });
}

async function get_path_date(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ date: request.pathParams.date });
}

async function get_query_few(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.queryParams ?? {});
}

async function get_query_medium(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.queryParams ?? {});
}

async function get_query_many(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.queryParams ?? {});
}

async function get_health(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ status: "ok" });
}

async function get_root(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ status: "ok" });
}

// Raw endpoints (no validation)
post("/json/small", post_json_small);
post("/json/medium", post_json_medium);
post("/json/large", post_json_large);
post("/json/very-large", post_json_very_large);

post("/multipart/small", post_multipart_small);
post("/multipart/medium", post_multipart_medium);
post("/multipart/large", post_multipart_large);

post("/urlencoded/simple", post_urlencoded_simple);
post("/urlencoded/complex", post_urlencoded_complex);

get("/path/simple/{id}", get_path_simple);
get("/path/multiple/{user_id}/{post_id}", get_path_multiple);
get("/path/deep/{org}/{team}/{project}/{resource}/{id}", get_path_deep);
get("/path/int/{id}", get_path_int);
get("/path/uuid/{uuid}", get_path_uuid);
get("/path/date/{date}", get_path_date);

get("/query/few", get_query_few);
get("/query/medium", get_query_medium);
get("/query/many", get_query_many);

get("/health", get_health);
get("/", get_root);

// Validated endpoints (with schemas at /validated/... paths)
post("/validated/json/small", post_json_small, requestSchema("json/small"), responseSchema("json/small"));
post("/validated/json/medium", post_json_medium, requestSchema("json/medium"), responseSchema("json/medium"));
post("/validated/json/large", post_json_large, requestSchema("json/large"), responseSchema("json/large"));
post(
	"/validated/json/very-large",
	post_json_very_large,
	requestSchema("json/very-large"),
	responseSchema("json/very-large"),
);

post(
	"/validated/multipart/small",
	post_multipart_small,
	requestSchema("multipart/small"),
	responseSchema("multipart/small"),
);
post(
	"/validated/multipart/medium",
	post_multipart_medium,
	requestSchema("multipart/medium"),
	responseSchema("multipart/medium"),
);
post(
	"/validated/multipart/large",
	post_multipart_large,
	requestSchema("multipart/large"),
	responseSchema("multipart/large"),
);

post(
	"/validated/urlencoded/simple",
	post_urlencoded_simple,
	requestSchema("urlencoded/simple"),
	responseSchema("urlencoded/simple"),
);
post(
	"/validated/urlencoded/complex",
	post_urlencoded_complex,
	requestSchema("urlencoded/complex"),
	responseSchema("urlencoded/complex"),
);

get("/validated/path/simple/{id}", get_path_simple, responseSchema("path/simple"), parameterSchema("path/simple"));
get(
	"/validated/path/multiple/{user_id}/{post_id}",
	get_path_multiple,
	responseSchema("path/multiple"),
	parameterSchema("path/multiple"),
);
get(
	"/validated/path/deep/{org}/{team}/{project}/{resource}/{id}",
	get_path_deep,
	responseSchema("path/deep"),
	parameterSchema("path/deep"),
);
get("/validated/path/int/{id}", get_path_int, responseSchema("path/int"), parameterSchema("path/int"));
get("/validated/path/uuid/{uuid}", get_path_uuid, responseSchema("path/uuid"), parameterSchema("path/uuid"));
get("/validated/path/date/{date}", get_path_date, responseSchema("path/date"), parameterSchema("path/date"));

get("/validated/query/few", get_query_few, responseSchema("query/few"), parameterSchema("query/few"));
get("/validated/query/medium", get_query_medium, responseSchema("query/medium"), parameterSchema("query/medium"));
get("/validated/query/many", get_query_many, responseSchema("query/many"), parameterSchema("query/many"));

get("/validated/health", get_health, responseSchema("health"));
get("/validated/", get_root, responseSchema("root"));

function resolvePort(defaultPort = 8000): number {
	for (const arg of process.argv.slice(2)) {
		const parsed = Number.parseInt(arg, 10);
		if (Number.isFinite(parsed) && parsed >= 0 && parsed < 65536) {
			return parsed;
		}
	}

	const envPort = process.env.PORT ? Number.parseInt(process.env.PORT, 10) : Number.NaN;
	if (Number.isFinite(envPort) && envPort >= 0 && envPort < 65536) {
		return envPort;
	}

	return defaultPort;
}

const port = resolvePort();

console.error(`[spikard-node] Starting server on port ${port}`);
const config = {
	host: "0.0.0.0",
	port,
};
const app = {
	routes,
	handlers,
};
try {
	native.runServer(app, config);
	console.error(`[spikard-node] runServer dispatched successfully`);
} catch (err) {
	console.error(`[spikard-node] Failed to start server:`, err);
	process.exit(1);
}

// Ensure the process stays alive while benchmarks run.
setInterval(() => {}, 1 << 30);
