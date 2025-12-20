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

await fastify.register(formbody);

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

fastify.post("/json/small", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.post("/json/medium", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.post("/json/large", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.post("/json/very-large", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.post("/multipart/small", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 1, total_bytes: 1024 };
});

fastify.post("/multipart/medium", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 2, total_bytes: 10240 };
});

fastify.post("/multipart/large", async (_request: FastifyRequest, _reply: FastifyReply): Promise<FileResponse> => {
	return { files_received: 5, total_bytes: 102400 };
});

fastify.post("/urlencoded/simple", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.post("/urlencoded/complex", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

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

fastify.get("/query/few", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.get("/query/medium", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.get("/query/many", async (request: FastifyRequest, _reply: FastifyReply): Promise<unknown> => {
	void request;
	return { ok: true };
});

fastify.get("/health", async (_request: FastifyRequest, _reply: FastifyReply): Promise<StatusResponse> => {
	return { status: "ok" };
});

fastify.get("/", async (_request: FastifyRequest, _reply: FastifyReply): Promise<StatusResponse> => {
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

try {
	await fastify.listen({ port, host: "0.0.0.0" });
	console.log(`Server listening on port ${port}`);
} catch (err) {
	fastify.log.error(err);
	process.exit(1);
}
