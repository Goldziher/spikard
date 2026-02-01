#!/usr/bin/env node

/**
 * Fastify benchmark server - unified raw + validation endpoints
 *
 * Raw endpoints: no validation (original paths)
 * Validated endpoints: JSON Schema validation (under /validated/* prefix)
 */

import formbody from "@fastify/formbody";
import multipart from "@fastify/multipart";
import Fastify, { type FastifyReply, type FastifyRequest } from "fastify";

const fastify = Fastify({ logger: false });

await fastify.register(formbody);
await fastify.register(multipart);

// ============================================================================
// RAW ENDPOINTS (No validation)
// ============================================================================

interface SimpleParams {
	id: string;
}

interface MultipleParams {
	user_id: string;
	post_id: string;
}

interface DeepParams {
	org: string;
	team: string;
	project: string;
	resource: string;
	id: string;
}

interface UuidParams {
	uuid: string;
}

interface DateParams {
	date: string;
}

interface FileResponse {
	files_received: number;
	total_bytes: number;
}

interface StatusResponse {
	status: string;
}

interface IdResponse {
	id: string | number;
}

interface UuidResponse {
	uuid: string;
}

interface DateResponse {
	date: string;
}

fastify.post("/json/small", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.post("/json/medium", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.post("/json/large", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.post("/json/very-large", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.post("/multipart/small", async (request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	return { files_received, total_bytes };
});

fastify.post("/multipart/medium", async (request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	return { files_received, total_bytes };
});

fastify.post("/multipart/large", async (request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	return { files_received, total_bytes };
});

fastify.post("/urlencoded/simple", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.post("/urlencoded/complex", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.body);
});

fastify.get(
	"/path/simple/:id",
	(request: FastifyRequest<{ Params: SimpleParams }>, reply: FastifyReply) => {
		const { id } = request.params;
		reply.send({ id });
	},
);

fastify.get(
	"/path/multiple/:user_id/:post_id",
	(request: FastifyRequest<{ Params: MultipleParams }>, reply: FastifyReply) => {
		const { user_id, post_id } = request.params;
		reply.send({ user_id, post_id });
	},
);

fastify.get(
	"/path/deep/:org/:team/:project/:resource/:id",
	(request: FastifyRequest<{ Params: DeepParams }>, reply: FastifyReply) => {
		const { org, team, project, resource, id } = request.params;
		reply.send({ org, team, project, resource, id });
	},
);

fastify.get(
	"/path/int/:id",
	(request: FastifyRequest<{ Params: SimpleParams }>, reply: FastifyReply) => {
		const { id } = request.params;
		reply.send({ id: Number.parseInt(id, 10) });
	},
);

fastify.get(
	"/path/uuid/:uuid",
	(request: FastifyRequest<{ Params: UuidParams }>, reply: FastifyReply) => {
		const { uuid } = request.params;
		reply.send({ uuid });
	},
);

fastify.get(
	"/path/date/:date",
	(request: FastifyRequest<{ Params: DateParams }>, reply: FastifyReply) => {
		const { date } = request.params;
		reply.send({ date });
	},
);

fastify.get("/query/few", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.query);
});

fastify.get("/query/medium", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.query);
});

fastify.get("/query/many", (request: FastifyRequest, reply: FastifyReply) => {
	reply.send(request.query);
});

fastify.get("/health", (_request: FastifyRequest, reply: FastifyReply) => {
	reply.send({ status: "ok" });
});

fastify.get("/", (_request: FastifyRequest, reply: FastifyReply) => {
	reply.send({ status: "ok" });
});

// ============================================================================
// VALIDATED ENDPOINTS (With JSON Schema validation)
// ============================================================================

const smallPayloadSchema = {
	type: "object",
	required: ["name", "description", "price", "tax"],
	properties: {
		name: { type: "string" },
		description: { type: "string" },
		price: { type: "number" },
		tax: { type: "number" },
	},
} as const;

const mediumPayloadSchema = {
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
		},
	},
} as const;

const largePayloadSchema = {
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
						},
					},
				},
			},
		},
	},
} as const;

const veryLargePayloadSchema = {
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
			},
		},
	},
} as const;

const urlencodedSimpleSchema = {
	type: "object",
	required: ["name", "email", "age", "subscribe"],
	properties: {
		name: { type: "string" },
		email: { type: "string" },
		age: { type: "string" },
		subscribe: { type: "string" },
	},
	additionalProperties: false,
} as const;

const urlencodedComplexSchema = {
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
		email: { type: "string" },
		first_name: { type: "string" },
		last_name: { type: "string" },
		age: { type: "string" },
		country: { type: "string" },
		state: { type: "string" },
		city: { type: "string" },
		zip: { type: "string" },
		phone: { type: "string" },
		company: { type: "string" },
		job_title: { type: "string" },
		subscribe: { type: "string" },
		newsletter: { type: "string" },
		terms_accepted: { type: "string" },
		privacy_accepted: { type: "string" },
		marketing_consent: { type: "string" },
		two_factor_enabled: { type: "string" },
	},
	additionalProperties: false,
} as const;

fastify.post("/validated/json/small", {
	schema: {
		body: smallPayloadSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.post("/validated/json/medium", {
	schema: {
		body: mediumPayloadSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.post("/validated/json/large", {
	schema: {
		body: largePayloadSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.post("/validated/json/very-large", {
	schema: {
		body: veryLargePayloadSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.post("/validated/multipart/small", async (request, reply) => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	if (files_received === 0) {
		reply.code(400);
		return { error: "No files received" };
	}

	return { files_received, total_bytes };
});

fastify.post("/validated/multipart/medium", async (request, reply) => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	if (files_received === 0) {
		reply.code(400);
		return { error: "No files received" };
	}

	return { files_received, total_bytes };
});

fastify.post("/validated/multipart/large", async (request, reply) => {
	const parts = request.parts();
	let files_received = 0;
	let total_bytes = 0;

	for await (const part of parts) {
		if (part.type === "file" && part.fieldname.startsWith("file")) {
			files_received++;
			const chunks: Buffer[] = [];
			for await (const chunk of part.file) {
				chunks.push(chunk);
			}
			const buffer = Buffer.concat(chunks);
			total_bytes += buffer.length;
		}
	}

	if (files_received === 0) {
		reply.code(400);
		return { error: "No files received" };
	}

	return { files_received, total_bytes };
});

fastify.post("/validated/urlencoded/simple", {
	schema: {
		body: urlencodedSimpleSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.post("/validated/urlencoded/complex", {
	schema: {
		body: urlencodedComplexSchema,
	},
	handler: (request, reply) => {
		reply.send(request.body);
	},
});

fastify.get("/validated/path/simple/:id", {
	schema: {
		params: {
			type: "object",
			properties: {
				id: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
			},
			required: ["id"],
		},
	},
	handler: (request, reply) => {
		const { id } = request.params as { id: string };
		reply.send({ id });
	},
});

fastify.get("/validated/path/multiple/:user_id/:post_id", {
	schema: {
		params: {
			type: "object",
			properties: {
				user_id: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
				post_id: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
			},
			required: ["user_id", "post_id"],
		},
	},
	handler: (request, reply) => {
		const { user_id, post_id } = request.params as {
			user_id: string;
			post_id: string;
		};
		reply.send({ user_id, post_id });
	},
});

fastify.get("/validated/path/deep/:org/:team/:project/:resource/:id", {
	schema: {
		params: {
			type: "object",
			properties: {
				org: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
				team: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
				project: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
				resource: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
				id: { type: "string", minLength: 1, maxLength: 255, pattern: "^[a-zA-Z0-9_-]+$" },
			},
			required: ["org", "team", "project", "resource", "id"],
		},
	},
	handler: (request, reply) => {
		const { org, team, project, resource, id } = request.params as {
			org: string;
			team: string;
			project: string;
			resource: string;
			id: string;
		};
		reply.send({ org, team, project, resource, id });
	},
});

fastify.get("/validated/path/int/:id", {
	schema: {
		params: {
			type: "object",
			properties: {
				id: { type: "integer" },
			},
			required: ["id"],
		},
	},
	handler: (request, reply) => {
		const { id } = request.params as { id: number };
		reply.send({ id });
	},
});

fastify.get("/validated/path/uuid/:uuid", {
	schema: {
		params: {
			type: "object",
			properties: {
				uuid: { type: "string", format: "uuid" },
			},
			required: ["uuid"],
		},
	},
	handler: (request, reply) => {
		const { uuid } = request.params as { uuid: string };
		reply.send({ uuid });
	},
});

fastify.get("/validated/path/date/:date", {
	schema: {
		params: {
			type: "object",
			properties: {
				date: { type: "string", format: "date" },
			},
			required: ["date"],
		},
	},
	handler: (request, reply) => {
		const { date } = request.params as { date: string };
		reply.send({ date });
	},
});

fastify.get("/validated/query/few", {
	schema: {
		querystring: {
			type: "object",
			required: ["q"],
			properties: {
				q: { type: "string" },
				page: { type: "integer" },
				limit: { type: "integer" },
			},
		},
	},
	handler: (request, reply) => {
		reply.send(request.query);
	},
});

fastify.get("/validated/query/medium", {
	schema: {
		querystring: {
			type: "object",
			required: ["search"],
			properties: {
				search: { type: "string" },
				category: { type: "string" },
				sort: { type: "string" },
				order: { type: "string" },
				page: { type: "integer" },
				limit: { type: "integer" },
				filter: { type: "string" },
			},
		},
	},
	handler: (request, reply) => {
		reply.send(request.query);
	},
});

fastify.get("/validated/query/many", {
	schema: {
		querystring: {
			type: "object",
			required: ["q"],
			properties: {
				q: { type: "string" },
				category: { type: "string" },
				subcategory: { type: "string" },
				brand: { type: "string" },
				min_price: { type: "number" },
				max_price: { type: "number" },
				color: { type: "string" },
				size: { type: "string" },
				material: { type: "string" },
				rating: { type: "integer" },
				sort: { type: "string" },
				order: { type: "string" },
				page: { type: "integer" },
				limit: { type: "integer" },
				in_stock: { type: "boolean" },
				on_sale: { type: "boolean" },
			},
		},
	},
	handler: (request, reply) => {
		reply.send(request.query);
	},
});

// ============================================================================
// Server Startup
// ============================================================================

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

const start = async () => {
	try {
		await fastify.listen({ port, host: "0.0.0.0" });
		console.error(`[fastify] Starting server on port ${port}`);
	} catch (err) {
		console.error(err);
		process.exit(1);
	}
};

start();
