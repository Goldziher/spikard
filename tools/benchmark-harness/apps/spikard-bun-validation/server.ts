#!/usr/bin/env node

/**
 * Spikard Bun HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure the Bun runtime binding performance.
 */

import { createRequire } from "node:module";
import process from "node:process";
import { readFileSync } from "node:fs";
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

async function post_multipart_small(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 1, total_bytes: 1024 });
}

async function post_multipart_medium(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 2, total_bytes: 10240 });
}

async function post_multipart_large(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 5, total_bytes: 102400 });
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

post("/json/small", post_json_small, requestSchema("json/small"), responseSchema("json/small"));
post("/json/medium", post_json_medium, requestSchema("json/medium"), responseSchema("json/medium"));
post("/json/large", post_json_large, requestSchema("json/large"), responseSchema("json/large"));
post("/json/very-large", post_json_very_large, requestSchema("json/very-large"), responseSchema("json/very-large"));

post("/multipart/small", post_multipart_small, requestSchema("multipart/small"), responseSchema("multipart/small"));
post("/multipart/medium", post_multipart_medium, requestSchema("multipart/medium"), responseSchema("multipart/medium"));
post("/multipart/large", post_multipart_large, requestSchema("multipart/large"), responseSchema("multipart/large"));

post(
	"/urlencoded/simple",
	post_urlencoded_simple,
	requestSchema("urlencoded/simple"),
	responseSchema("urlencoded/simple"),
);
post(
	"/urlencoded/complex",
	post_urlencoded_complex,
	requestSchema("urlencoded/complex"),
	responseSchema("urlencoded/complex"),
);

get("/path/simple/{id}", get_path_simple, responseSchema("path/simple"), parameterSchema("path/simple"));
get(
	"/path/multiple/{user_id}/{post_id}",
	get_path_multiple,
	responseSchema("path/multiple"),
	parameterSchema("path/multiple"),
);
get(
	"/path/deep/{org}/{team}/{project}/{resource}/{id}",
	get_path_deep,
	responseSchema("path/deep"),
	parameterSchema("path/deep"),
);
get("/path/int/{id}", get_path_int, responseSchema("path/int"), parameterSchema("path/int"));
get("/path/uuid/{uuid}", get_path_uuid, responseSchema("path/uuid"), parameterSchema("path/uuid"));
get("/path/date/{date}", get_path_date, responseSchema("path/date"), parameterSchema("path/date"));

get("/query/few", get_query_few, responseSchema("query/few"), parameterSchema("query/few"));
get("/query/medium", get_query_medium, responseSchema("query/medium"), parameterSchema("query/medium"));
get("/query/many", get_query_many, responseSchema("query/many"), parameterSchema("query/many"));

get("/health", get_health, responseSchema("health"));
get("/", get_root, responseSchema("root"));

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

console.error(`[spikard-bun] Starting server on port ${port}`);
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
	console.error(`[spikard-bun] runServer dispatched successfully`);
} catch (err) {
	console.error(`[spikard-bun] Failed to start server:`, err);
	process.exit(1);
}

// Ensure the process stays alive while benchmarks run.
setInterval(() => {}, 1 << 30);
