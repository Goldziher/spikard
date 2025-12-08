#!/usr/bin/env node

/**
 * Fastify RAW comparison server for benchmarking
 *
 * NO VALIDATION - accepts any JSON body and echoes it back.
 * This measures Fastify's raw performance without validation overhead.
 */

import formbody from "@fastify/formbody";
import Fastify, { type FastifyReply, type FastifyRequest } from "fastify";

const fastify = Fastify({ logger: false });

// Register form body parser for URL-encoded forms
await fastify.register(formbody);

// ============================================================================
// Type Definitions
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

// ============================================================================
// JSON Body Workloads - NO VALIDATION
// ============================================================================

fastify.post("/json/small", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body;
});

fastify.post("/json/medium", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body;
});

fastify.post("/json/large", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body;
});

fastify.post("/json/very-large", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body;
});

// ============================================================================
// Multipart Form Workloads - NO VALIDATION
// ============================================================================

fastify.post("/multipart/small", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 1, total_bytes: 1024 };
});

fastify.post("/multipart/medium", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 2, total_bytes: 10240 };
});

fastify.post("/multipart/large", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 5, total_bytes: 102400 };
});

// ============================================================================
// URL Encoded Form Workloads - NO VALIDATION
// ============================================================================

fastify.post("/urlencoded/simple", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body ?? {};
});

fastify.post("/urlencoded/complex", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.body ?? {};
});

// ============================================================================
// Path Parameter Workloads - NO VALIDATION
// ============================================================================

fastify.get(
	"/path/simple/:id",
	async (request: FastifyRequest<{ Params: SimpleParams }>, _reply: FastifyReply): Promise<IdResponse> => {
		const { id } = request.params;
		return { id };
	},
);

fastify.get(
	"/path/multiple/:user_id/:post_id",
	async (request: FastifyRequest<{ Params: MultipleParams }>, _reply: FastifyReply): Promise<MultipleParams> => {
		const { user_id, post_id } = request.params;
		return { user_id, post_id };
	},
);

fastify.get(
	"/path/deep/:org/:team/:project/:resource/:id",
	async (request: FastifyRequest<{ Params: DeepParams }>, _reply: FastifyReply): Promise<DeepParams> => {
		const { org, team, project, resource, id } = request.params;
		return { org, team, project, resource, id };
	},
);

fastify.get(
	"/path/int/:id",
	async (request: FastifyRequest<{ Params: SimpleParams }>, _reply: FastifyReply): Promise<IdResponse> => {
		const { id } = request.params;
		return { id: Number.parseInt(id, 10) };
	},
);

fastify.get(
	"/path/uuid/:uuid",
	async (request: FastifyRequest<{ Params: UuidParams }>, _reply: FastifyReply): Promise<UuidResponse> => {
		const { uuid } = request.params;
		return { uuid };
	},
);

fastify.get(
	"/path/date/:date",
	async (request: FastifyRequest<{ Params: DateParams }>, _reply: FastifyReply): Promise<DateResponse> => {
		const { date } = request.params;
		return { date };
	},
);

// ============================================================================
// Query Parameter Workloads - NO VALIDATION
// ============================================================================

fastify.get("/query/few", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.query ?? {};
});

fastify.get("/query/medium", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.query ?? {};
});

fastify.get("/query/many", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	return request.query ?? {};
});

// ============================================================================
// Health Check
// ============================================================================

fastify.get("/health", async (_request: FastifyRequest, _reply: FastifyReply): Promise<StatusResponse> => {
	return { status: "ok" };
});

fastify.get("/", async (_request: FastifyRequest, _reply: FastifyReply): Promise<StatusResponse> => {
	return { status: "ok" };
});

// ============================================================================
// Server Startup
// ============================================================================

const port = process.argv[2]
	? Number.parseInt(process.argv[2], 10)
	: process.env.PORT
		? Number.parseInt(process.env.PORT, 10)
		: 8000;

try {
	await fastify.listen({ port, host: "0.0.0.0" });
	console.log(`Server listening on port ${port}`);
} catch (err) {
	fastify.log.error(err);
	process.exit(1);
}
