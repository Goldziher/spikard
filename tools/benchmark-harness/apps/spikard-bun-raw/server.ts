#!/usr/bin/env node

/**
 * Spikard Bun HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure the Bun runtime binding performance.
 */

import { createRequire } from "node:module";
import process from "node:process";

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
		tags: {
			type: "array",
			items: { type: "string" },
		},
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

function registerRoute(
	method: string,
	path: string,
	handler: HandlerFunction,
	requestSchema?: unknown,
	parameterSchema?: unknown,
): void {
	const metadata: RouteMetadata = {
		method: method.toUpperCase(),
		path,
		handler_name: handler.name,
		is_async: true,
		request_schema: requestSchema,
		parameter_schema: parameterSchema,
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

function get(path: string, handler: HandlerFunction, parameterSchema?: unknown): void {
	registerRoute("GET", path, handler, undefined, parameterSchema);
}

function post(path: string, handler: HandlerFunction, requestSchema?: unknown, parameterSchema?: unknown): void {
	registerRoute("POST", path, handler, requestSchema, parameterSchema);
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
