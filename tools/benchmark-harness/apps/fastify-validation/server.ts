#!/usr/bin/env node
/**
 * Fastify benchmark server for workload comparison.
 *
 * Fastify is a high-performance Node.js web framework with built-in JSON Schema validation.
 */

import formbody from "@fastify/formbody";
import multipart from "@fastify/multipart";
import Fastify from "fastify";

const fastify = Fastify({ logger: false });

await fastify.register(formbody);
await fastify.register(multipart);

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
	properties: {
		username: { type: "string" },
		password: { type: "string" },
	},
} as const;

const urlencodedComplexSchema = {
	type: "object",
	additionalProperties: true,
} as const;

fastify.post("/json/small", {
	schema: {
		body: smallPayloadSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.post("/json/medium", {
	schema: {
		body: mediumPayloadSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.post("/json/large", {
	schema: {
		body: largePayloadSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.post("/json/very-large", {
	schema: {
		body: veryLargePayloadSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.post("/multipart/small", async (_request, _reply) => {
	return { files_received: 1, total_bytes: 1024 };
});

fastify.post("/multipart/medium", async (_request, _reply) => {
	return { files_received: 2, total_bytes: 10240 };
});

fastify.post("/multipart/large", async (_request, _reply) => {
	return { files_received: 5, total_bytes: 102400 };
});

fastify.post("/urlencoded/simple", {
	schema: {
		body: urlencodedSimpleSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.post("/urlencoded/complex", {
	schema: {
		body: urlencodedComplexSchema,
	},
	handler: async (request, _reply) => {
		void request;
		return { ok: true };
	},
});

fastify.get("/path/simple/:id", async (request, _reply) => {
	const { id } = request.params as { id: string };
	return { id };
});

fastify.get("/path/multiple/:user_id/:post_id", async (request, _reply) => {
	const { user_id, post_id } = request.params as {
		user_id: string;
		post_id: string;
	};
	return { user_id, post_id };
});

fastify.get("/path/deep/:org/:team/:project/:resource/:id", async (request, _reply) => {
	const { org, team, project, resource, id } = request.params as {
		org: string;
		team: string;
		project: string;
		resource: string;
		id: string;
	};
	return { org, team, project, resource, id };
});

fastify.get("/path/int/:id", async (request, _reply) => {
	const { id } = request.params as { id: string };
	return { id: parseInt(id, 10) };
});

fastify.get("/path/uuid/:uuid", async (request, _reply) => {
	const { uuid } = request.params as { uuid: string };
	return { uuid };
});

fastify.get("/path/date/:date", async (request, _reply) => {
	const { date } = request.params as { date: string };
	return { date };
});

fastify.get("/query/few", async (request, _reply) => {
	void request;
	return { ok: true };
});

fastify.get("/query/medium", async (request, _reply) => {
	void request;
	return { ok: true };
});

fastify.get("/query/many", async (request, _reply) => {
	void request;
	return { ok: true };
});

fastify.get("/health", async (_request, _reply) => {
	return { status: "ok" };
});

fastify.get("/", async (_request, _reply) => {
	return { status: "ok" };
});

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
