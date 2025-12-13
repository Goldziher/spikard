import type { Request, Response } from "@cloudflare/workers-types";

/**
 * Spikard WASM Example on Cloudflare Workers
 *
 * This example demonstrates a Spikard WASM application running on Cloudflare Workers.
 * It provides a simple HTTP server with JSON endpoints and proper error handling.
 */

/**
 * Response type for JSON API responses
 */
interface JsonResponse<T = unknown> {
	readonly success: boolean;
	readonly data?: T;
	readonly error?: string;
	readonly timestamp: string;
}

/**
 * Echo request body structure
 */
interface EchoRequest {
	readonly message: string;
	readonly metadata?: Record<string, unknown>;
}

/**
 * Data response structure
 */
interface DataResponse {
	readonly id: string;
	readonly name: string;
	readonly version: string;
}

/**
 * Route handler for GET /
 * Returns welcome message and available routes
 */
function handleHome(): Response {
	const response: JsonResponse<{ message: string; routes: string[] }> = {
		success: true,
		data: {
			message: "Welcome to Spikard WASM on Cloudflare Workers",
			routes: ["GET /", "GET /api/data", "POST /api/echo", "GET /health"],
		},
		timestamp: new Date().toISOString(),
	};

	return new Response(JSON.stringify(response), {
		status: 200,
		headers: {
			"content-type": "application/json",
			"cache-control": "public, max-age=60",
		},
	});
}

/**
 * Route handler for GET /api/data
 * Returns sample data object
 */
function handleGetData(): Response {
	const data: DataResponse = {
		id: "spikard-001",
		name: "Spikard WASM Cloudflare",
		version: "0.3.7",
	};

	const response: JsonResponse<DataResponse> = {
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
 * Echoes back the request body with received timestamp
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
			const errorResponse: JsonResponse<null> = {
				success: false,
				error: "Invalid request body. Expected { message: string, metadata?: object }",
				timestamp: new Date().toISOString(),
			};

			return new Response(JSON.stringify(errorResponse), {
				status: 400,
				headers: {
					"content-type": "application/json",
				},
			});
		}

		const echoRequest = body as EchoRequest;

		const response: JsonResponse<EchoRequest & { receivedAt: string }> = {
			success: true,
			data: {
				...echoRequest,
				receivedAt: new Date().toISOString(),
			},
			timestamp: new Date().toISOString(),
		};

		return new Response(JSON.stringify(response), {
			status: 200,
			headers: {
				"content-type": "application/json",
			},
		});
	} catch (error) {
		const errorResponse: JsonResponse<null> = {
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
 * Returns health status of the service
 */
function handleHealth(): Response {
	const response: JsonResponse<{ status: string; checks: Record<string, string> }> = {
		success: true,
		data: {
			status: "healthy",
			checks: {
				wasm: "operational",
				cloudflare: "operational",
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
 * Returns error for unmatched routes
 */
function handleNotFound(): Response {
	const response: JsonResponse<null> = {
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
 * Parse URL path from request URL
 *
 * @param url - The request URL
 * @returns The pathname of the URL
 */
function getPathname(url: string): string {
	return new URL(url).pathname;
}

/**
 * Match request to handler based on method and path
 *
 * @param method - HTTP method (GET, POST, etc.)
 * @param pathname - URL pathname
 * @param request - The request object
 * @returns Response object
 */
function routeRequest(method: string, pathname: string, request: Request): Response | Promise<Response> {
	const normalizedMethod = method.toUpperCase();

	if (pathname === "/" && normalizedMethod === "GET") {
		return handleHome();
	}

	if (pathname === "/api/data" && normalizedMethod === "GET") {
		return handleGetData();
	}

	if (pathname === "/api/echo" && normalizedMethod === "POST") {
		return handleEcho(request);
	}

	if (pathname === "/health" && normalizedMethod === "GET") {
		return handleHealth();
	}

	return handleNotFound();
}

/**
 * Main fetch handler for Cloudflare Workers
 * Implements routing for Spikard WASM endpoints
 */
export default {
	async fetch(request: Request): Promise<Response> {
		const pathname = getPathname(request.url);
		const method = request.method;

		return await Promise.resolve(routeRequest(method, pathname, request));
	},
};
