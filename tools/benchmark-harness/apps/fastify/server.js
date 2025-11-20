#!/usr/bin/env node
/**
 * Fastify comparison server for benchmarking
 */

const fastify = require("fastify")({ logger: false });

fastify.get("/", async (_request, _reply) => {
	return { message: "Hello, World!" };
});

fastify.get("/health", async (_request, _reply) => {
	return { status: "healthy" };
});

fastify.get("/users/:user_id", async (request, _reply) => {
	const userId = parseInt(request.params.user_id, 10);
	return { user_id: userId, name: `User ${userId}` };
});

fastify.post("/echo", async (_request, _reply) => {
	return { echoed: true };
});

fastify.get("/items", async (_request, _reply) => {
	return {
		items: [
			{ id: 1, name: "Item 1" },
			{ id: 2, name: "Item 2" },
		],
	};
});

const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;
const start = async () => {
	try {
		await fastify.listen({ port, host: "0.0.0.0" });
		console.log(`Server listening on port ${port}`);
	} catch (err) {
		fastify.log.error(err);
		process.exit(1);
	}
};

start();
