#!/usr/bin/env node

/**
 * Spikard Node.js HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Node.js binding performance.
 */

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const native = require("../../../../packages/node/index.js") as { runServer: (app: unknown, config: unknown) => void };
import { z } from "zod";

// Route registration arrays
type HandlerInput = {
	method: string;
	path: string;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	query_params: unknown;
	body: unknown;
	path_params: Record<string, string>;
};

type HandlerOutput = {
	status: number;
	headers?: Record<string, string>;
	body?: unknown;
};

type HandlerFunction = (input: HandlerInput) => Promise<HandlerOutput>;

const routes: Array<{ method: string; path: string; handler_name: string; is_async: boolean }> = [];
const handlers: Record<string, HandlerFunction> = {};

// Helper functions to register routes
function registerRoute(method: string, path: string, handler: HandlerFunction): HandlerFunction {
	const metadata = {
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
	};
	routes.push(metadata);
	handlers[handler.name] = handler;
	return handler;
}

function get(path: string): (handler: HandlerFunction) => HandlerFunction {
	return (handler) => registerRoute("GET", path, handler);
}

function post(path: string): (handler: HandlerFunction) => HandlerFunction {
	return (handler) => registerRoute("POST", path, handler);
}

function ok(body: unknown): HandlerOutput {
	return { status: 200, body };
}

// ============================================================================
// Zod Schemas for Validation
// ============================================================================

const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number().optional(),
});

const AddressSchema = z.object({
	street: z.string(),
	city: z.string(),
	state: z.string(),
	zip_code: z.string(),
});

const MediumPayloadSchema = z.object({
	user_id: z.number(),
	username: z.string(),
	email: z.string(),
	is_active: z.boolean(),
	address: AddressSchema,
	tags: z.array(z.string()),
});

const ItemSchema = z.object({
	id: z.number(),
	name: z.string(),
	price: z.number(),
	in_stock: z.boolean(),
});

const LargePayloadSchema = z.object({
	order_id: z.string(),
	customer_name: z.string(),
	items: z.array(ItemSchema),
	total: z.number(),
	notes: z.string(),
});

const VeryLargePayloadSchema = z.object({
	data: z.array(z.record(z.any())),
	metadata: z.record(z.any()),
});

// ============================================================================
// JSON Body Workloads
// ============================================================================

async function post_json_small(request: HandlerInput): Promise<HandlerOutput> {
	const validated = SmallPayloadSchema.parse(request.body);
	return ok(validated);
}

async function post_json_medium(request: HandlerInput): Promise<HandlerOutput> {
	const validated = MediumPayloadSchema.parse(request.body);
	return ok(validated);
}

async function post_json_large(request: HandlerInput): Promise<HandlerOutput> {
	const validated = LargePayloadSchema.parse(request.body);
	return ok(validated);
}

async function post_json_very_large(request: HandlerInput): Promise<HandlerOutput> {
	const validated = VeryLargePayloadSchema.parse(request.body);
	return ok(validated);
}

// ============================================================================
// Multipart Form Workloads
// ============================================================================

async function post_multipart_small(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 1, total_bytes: 1024 });
}

async function post_multipart_medium(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 2, total_bytes: 10240 });
}

async function post_multipart_large(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ files_received: 5, total_bytes: 102400 });
}

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

async function post_urlencoded_simple(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body ?? {});
}

async function post_urlencoded_complex(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.body ?? {});
}

// ============================================================================
// Path Parameter Workloads
// ============================================================================

async function get_path_simple(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ id: request.path_params.id });
}

async function get_path_multiple(request: HandlerInput): Promise<HandlerOutput> {
	return ok({
		user_id: request.path_params.user_id,
		post_id: request.path_params.post_id,
	});
}

async function get_path_deep(request: HandlerInput): Promise<HandlerOutput> {
	return ok({
		org: request.path_params.org,
		team: request.path_params.team,
		project: request.path_params.project,
		resource: request.path_params.resource,
		id: request.path_params.id,
	});
}

async function get_path_int(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ id: parseInt(request.path_params.id, 10) });
}

async function get_path_uuid(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ uuid: request.path_params.uuid });
}

async function get_path_date(request: HandlerInput): Promise<HandlerOutput> {
	return ok({ date: request.path_params.date });
}

// ============================================================================
// Query Parameter Workloads
// ============================================================================

async function get_query_few(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.query_params ?? {});
}

async function get_query_medium(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.query_params ?? {});
}

async function get_query_many(request: HandlerInput): Promise<HandlerOutput> {
	return ok(request.query_params ?? {});
}

// ============================================================================
// Health Check
// ============================================================================

async function get_health(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ status: "ok" });
}

async function get_root(_request: HandlerInput): Promise<HandlerOutput> {
	return ok({ status: "ok" });
}

// Register all routes
post("/json/small")(post_json_small);
post("/json/medium")(post_json_medium);
post("/json/large")(post_json_large);
post("/json/very-large")(post_json_very_large);

post("/multipart/small")(post_multipart_small);
post("/multipart/medium")(post_multipart_medium);
post("/multipart/large")(post_multipart_large);

post("/urlencoded/simple")(post_urlencoded_simple);
post("/urlencoded/complex")(post_urlencoded_complex);

get("/path/simple/{id}")(get_path_simple);
get("/path/multiple/{user_id}/{post_id}")(get_path_multiple);
get("/path/deep/{org}/{team}/{project}/{resource}/{id}")(get_path_deep);
get("/path/int/{id}")(get_path_int);
get("/path/uuid/{uuid}")(get_path_uuid);
get("/path/date/{date}")(get_path_date);

get("/query/few")(get_query_few);
get("/query/medium")(get_query_medium);
get("/query/many")(get_query_many);

get("/health")(get_health);
get("/")(get_root);

// Create app object
const app = {
	routes,
	handlers,
};

// Parse port from command line or environment
const port = process.argv[2] ? parseInt(process.argv[2], 10) : process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;

// Start the server
console.error(`[spikard-node] Starting server on port ${port}`);
const config = {
	host: "0.0.0.0",
	port,
};
try {
	// runServer is fire-and-forget; any error is thrown synchronously before background thread starts.
	native.runServer(app, config);
	console.error(`[spikard-node] runServer dispatched successfully`);
} catch (err) {
	console.error(`[spikard-node] Failed to start server:`, err);
	process.exit(1);
}
