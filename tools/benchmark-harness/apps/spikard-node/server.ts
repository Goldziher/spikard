#!/usr/bin/env node

/**
 * Spikard Node.js HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Node.js binding performance.
 */

import { createRequire } from "node:module";
import { z } from "zod";

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

function registerRoute(method: string, path: string, handler: HandlerFunction, requestSchema?: unknown): void {
	const metadata: RouteMetadata = {
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
		request_schema: requestSchema,
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

function get(path: string, handler: HandlerFunction): void {
	registerRoute("GET", path, handler);
}

function post(path: string, handler: HandlerFunction, requestSchema: unknown = {}): void {
	registerRoute("POST", path, handler, requestSchema);
}

function ok(body: unknown): HandlerOutput {
	return { status: 200, body };
}

const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number().optional(),
});

const MediumPayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	image: z.object({
		url: z.string(),
		name: z.string(),
	}),
});

const LargePayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	seller: z.object({
		name: z.string(),
		address: z.object({
			street: z.string(),
			city: z.string(),
			country: z.object({
				name: z.string(),
				code: z.string(),
			}),
		}),
	}),
});

const VeryLargePayloadSchema = z.object({
	name: z.string(),
	tags: z.array(z.string()),
	images: z.array(
		z.object({
			url: z.string(),
			name: z.string(),
		}),
	),
});

async function post_json_small(request: HandlerInput): Promise<HandlerOutput> {
	return ok(SmallPayloadSchema.parse(request.body));
}

async function post_json_medium(request: HandlerInput): Promise<HandlerOutput> {
	return ok(MediumPayloadSchema.parse(request.body));
}

async function post_json_large(request: HandlerInput): Promise<HandlerOutput> {
	return ok(LargePayloadSchema.parse(request.body));
}

async function post_json_very_large(request: HandlerInput): Promise<HandlerOutput> {
	return ok(VeryLargePayloadSchema.parse(request.body));
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

const port = process.argv[2] ? parseInt(process.argv[2], 10) : process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;

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
