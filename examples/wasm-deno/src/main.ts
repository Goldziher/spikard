/// <reference lib="deno.window" />
/// <reference lib="deno.ns" />

/**
 * Spikard WASM Deno Example Server
 *
 * This example demonstrates how to use Spikard WASM framework with Deno runtime.
 * It creates a simple HTTP server with JSON endpoints and proper error handling.
 */

/**
 * Standard API response wrapper for all endpoints
 */
interface ApiResponse<T = unknown> {
	readonly success: boolean;
	readonly data?: T;
	readonly error?: string;
	readonly timestamp: string;
}

/**
 * Data object response structure
 */
interface DataObject {
	readonly id: string;
	readonly name: string;
	readonly version: string;
	readonly timestamp: string;
}

/**
 * Echo response structure
 */
interface EchoObject {
	readonly message: string;
	readonly received_at: string;
}

/**
 * Parse URL and extract pathname and search
 */
function parseUrl(url: string): { pathname: string; search: string } {
	const urlObj = new URL(url);
	return {
		pathname: urlObj.pathname,
		search: urlObj.search,
	};
}

/**
 * Route handler for GET /
 */
function handleHome(): Response {
	const response: ApiResponse<{ message: string; routes: readonly string[] }> = {
		success: true,
		data: {
			message: "Welcome to Spikard WASM on Deno!",
			routes: ["GET /", "GET /api/data", "POST /api/echo", "GET /health"],
		},
		timestamp: new Date().toISOString(),
	};

	return new Response(JSON.stringify(response), {
		status: 200,
		headers: {
			"content-type": "application/json",
		},
	});
}

/**
 * Route handler for GET /api/data
 */
function handleData(): Response {
	const data: DataObject = {
		id: crypto.randomUUID(),
		name: "spikard-deno",
		version: "0.3.7",
		timestamp: new Date().toISOString(),
	};

	const response: ApiResponse<DataObject> = {
		success: true,
		data,
		timestamp: new Date().toISOString(),
	};

	return new Response(JSON.stringify(response), {
		status: 200,
		headers: {
			"content-type": "application/json",
			"cache-control": "public, max-age=30",
		},
	});
}

/**
 * Route handler for POST /api/echo
 */
async function handleEcho(request: Request): Promise<Response> {
	try {
		const body = (await request.json()) as unknown;

		if (
			typeof body !== "object" ||
			body === null ||
			!("message" in body) ||
			typeof (body as Record<string, unknown>).message !== "string"
		) {
			const errorResponse: ApiResponse<null> = {
				success: false,
				error: "Invalid request body. Expected { message: string }",
				timestamp: new Date().toISOString(),
			};

			return new Response(JSON.stringify(errorResponse), {
				status: 400,
				headers: {
					"content-type": "application/json",
				},
			});
		}

		const typedBody = body as { message: string };
		const echo: EchoObject = {
			message: typedBody.message,
			received_at: new Date().toISOString(),
		};

		const response: ApiResponse<EchoObject> = {
			success: true,
			data: echo,
			timestamp: new Date().toISOString(),
		};

		return new Response(JSON.stringify(response), {
			status: 200,
			headers: {
				"content-type": "application/json",
			},
		});
	} catch (error) {
		const errorResponse: ApiResponse<null> = {
			success: false,
			error: error instanceof Error ? error.message : "Failed to process request",
			timestamp: new Date().toISOString(),
		};

		return new Response(JSON.stringify(errorResponse), {
			status: 400,
			headers: {
				"content-type": "application/json",
			},
		});
	}
}

/**
 * Route handler for GET /health
 */
function handleHealth(): Response {
	const response: ApiResponse<{ status: string; checks: Record<string, string> }> = {
		success: true,
		data: {
			status: "healthy",
			checks: {
				wasm: "operational",
				deno: "operational",
			},
		},
		timestamp: new Date().toISOString(),
	};

	return new Response(JSON.stringify(response), {
		status: 200,
		headers: {
			"content-type": "application/json",
			"cache-control": "no-cache",
		},
	});
}

/**
 * Route handler for 404 Not Found
 */
function handleNotFound(): Response {
	const response: ApiResponse<null> = {
		success: false,
		error: "Route not found",
		timestamp: new Date().toISOString(),
	};

	return new Response(JSON.stringify(response), {
		status: 404,
		headers: {
			"content-type": "application/json",
		},
	});
}

/**
 * Main server startup
 */
async function startServer(): Promise<void> {
	const port = 8000;
	const hostname = "0.0.0.0";

	console.log("Starting Spikard WASM server on Deno...");
	console.log(`Listening on http://${hostname}:${port}`);
	console.log("Available routes:");
	console.log("  GET  /");
	console.log("  GET  /api/data");
	console.log("  POST /api/echo");
	console.log("  GET  /health");

	await Deno.serve(
		{
			port,
			hostname,
		},
		async (request: Request): Promise<Response> => {
			const { pathname } = parseUrl(request.url);
			const method = request.method.toUpperCase();

			if (pathname === "/" && method === "GET") {
				return handleHome();
			}

			if (pathname === "/api/data" && method === "GET") {
				return handleData();
			}

			if (pathname === "/api/echo" && method === "POST") {
				return handleEcho(request);
			}

			if (pathname === "/health" && method === "GET") {
				return handleHealth();
			}

			return handleNotFound();
		},
	);
}

async function main() {
	await startServer().catch((error: unknown) => {
		console.error("Failed to start server:", error);
		Deno.exit(1);
	});
}

main();
