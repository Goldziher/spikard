import { runServer, type HandlerInput, type HandlerOutput } from "@spikard/node";

/**
 * Node.js test application for published @spikard/node package.
 *
 * Tests the PUBLISHED package from npm (0.7.1).
 */

// Define route handlers
async function health(_request: HandlerInput): Promise<HandlerOutput> {
	return {
		status: 200,
		body: { status: "ok" },
	};
}

async function query(request: HandlerInput): Promise<HandlerOutput> {
	const name = request.queryParams?.name ?? "";
	const ageStr = request.queryParams?.age ?? "0";
	const age = parseInt(ageStr as string, 10);
	return {
		status: 200,
		body: { name, age },
	};
}

async function echo(request: HandlerInput): Promise<HandlerOutput> {
	return {
		status: 200,
		body: {
			received: request.body,
			method: "POST",
		},
	};
}

async function user(request: HandlerInput): Promise<HandlerOutput> {
	const userId = request.pathParams?.id ?? "";
	return {
		status: 200,
		body: {
			userId,
			type: "string",
		},
	};
}

export function createApp() {
	return {
		routes: [
			{
				method: "GET",
				path: "/health",
				handler_name: "health",
				is_async: true,
			},
			{
				method: "GET",
				path: "/query",
				handler_name: "query",
				is_async: true,
			},
			{
				method: "POST",
				path: "/echo",
				handler_name: "echo",
				is_async: true,
			},
			{
				method: "GET",
				path: "/users/:id",
				handler_name: "user",
				is_async: true,
			},
		],
		handlers: {
			health,
			query,
			echo,
			user,
		},
	};
}

// Run if executed directly
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

if (import.meta.url === `file://${process.argv[1]}`) {
	const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;
	const app = createApp();
	console.log(`Starting Spikard Node.js test server on http://127.0.0.1:${port}`);
	runServer(app, { host: "127.0.0.1", port });
}
