#!/usr/bin/env node

/**
 * Spikard Node.js HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Node.js binding performance.
 */

import { Spikard, runServer, type Request } from "@spikard/node";
import { z } from "zod";

const app = new Spikard();

function addRoute(method: "GET" | "POST", path: string, handler: (req: Request) => unknown | Promise<unknown>): void {
	app.addRoute(
		{
			method,
			path,
			handler_name: handler.name,
			is_async: true,
		},
		handler,
	);
}

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

async function post_json_small(request: Request): Promise<unknown> {
	return SmallPayloadSchema.parse(request.json());
}

async function post_json_medium(request: Request): Promise<unknown> {
	return MediumPayloadSchema.parse(request.json());
}

async function post_json_large(request: Request): Promise<unknown> {
	return LargePayloadSchema.parse(request.json());
}

async function post_json_very_large(request: Request): Promise<unknown> {
	return VeryLargePayloadSchema.parse(request.json());
}

async function post_multipart_small(_request: Request): Promise<unknown> {
	return { files_received: 1, total_bytes: 1024 };
}

async function post_multipart_medium(_request: Request): Promise<unknown> {
	return { files_received: 2, total_bytes: 10240 };
}

async function post_multipart_large(_request: Request): Promise<unknown> {
	return { files_received: 5, total_bytes: 102400 };
}

async function post_urlencoded_simple(request: Request): Promise<unknown> {
	return request.form();
}

async function post_urlencoded_complex(request: Request): Promise<unknown> {
	return request.form();
}

async function get_path_simple(request: Request): Promise<unknown> {
	return { id: request.params.id };
}

async function get_path_multiple(request: Request): Promise<unknown> {
	return {
		user_id: request.params.user_id,
		post_id: request.params.post_id,
	};
}

async function get_path_deep(request: Request): Promise<unknown> {
	return {
		org: request.params.org,
		team: request.params.team,
		project: request.params.project,
		resource: request.params.resource,
		id: request.params.id,
	};
}

async function get_path_int(request: Request): Promise<unknown> {
	return { id: parseInt(request.params.id, 10) };
}

async function get_path_uuid(request: Request): Promise<unknown> {
	return { uuid: request.params.uuid };
}

async function get_path_date(request: Request): Promise<unknown> {
	return { date: request.params.date };
}

async function get_query_few(request: Request): Promise<unknown> {
	return request.query;
}

async function get_query_medium(request: Request): Promise<unknown> {
	return request.query;
}

async function get_query_many(request: Request): Promise<unknown> {
	return request.query;
}

async function get_health(_request: Request): Promise<unknown> {
	return { status: "ok" };
}

async function get_root(_request: Request): Promise<unknown> {
	return { status: "ok" };
}

addRoute("POST", "/json/small", post_json_small);
addRoute("POST", "/json/medium", post_json_medium);
addRoute("POST", "/json/large", post_json_large);
addRoute("POST", "/json/very-large", post_json_very_large);

addRoute("POST", "/multipart/small", post_multipart_small);
addRoute("POST", "/multipart/medium", post_multipart_medium);
addRoute("POST", "/multipart/large", post_multipart_large);

addRoute("POST", "/urlencoded/simple", post_urlencoded_simple);
addRoute("POST", "/urlencoded/complex", post_urlencoded_complex);

addRoute("GET", "/path/simple/{id}", get_path_simple);
addRoute("GET", "/path/multiple/{user_id}/{post_id}", get_path_multiple);
addRoute("GET", "/path/deep/{org}/{team}/{project}/{resource}/{id}", get_path_deep);
addRoute("GET", "/path/int/{id}", get_path_int);
addRoute("GET", "/path/uuid/{uuid}", get_path_uuid);
addRoute("GET", "/path/date/{date}", get_path_date);

addRoute("GET", "/query/few", get_query_few);
addRoute("GET", "/query/medium", get_query_medium);
addRoute("GET", "/query/many", get_query_many);

addRoute("GET", "/health", get_health);
addRoute("GET", "/", get_root);

const port = process.argv[2] ? parseInt(process.argv[2], 10) : process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;

console.error(`[spikard-node] Starting server on port ${port}`);
const config = {
	host: "0.0.0.0",
	port,
};
try {
	runServer(app, config);
	console.error(`[spikard-node] runServer dispatched successfully`);
} catch (err) {
	console.error(`[spikard-node] Failed to start server:`, err);
	process.exit(1);
}

// Keep the Node.js process alive (Rust server runs on a background thread).
const keepAlive = setInterval(() => {}, 1 << 30);
keepAlive.unref?.();

process.on("SIGINT", () => {
	clearInterval(keepAlive);
	process.exit(0);
});

process.on("SIGTERM", () => {
	clearInterval(keepAlive);
	process.exit(0);
});
