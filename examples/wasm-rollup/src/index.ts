/**
 * Spikard WASM Example with Rollup Bundler
 *
 * This example demonstrates a minimal Spikard HTTP application compiled
 * with Rollup, TypeScript, and bundled for browsers and Node.js.
 * It includes three basic routes with JSON responses.
 */

import { Spikard, get, post } from "@spikard/wasm";
import type { HandlerFunction, Request } from "@spikard/wasm";

/**
 * Standard response wrapper for all endpoints
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
}

/**
 * Echo response data structure
 */
interface EchoData {
	readonly message: string;
	readonly receivedAt: string;
}

/**
 * GET / - Root endpoint returning a welcome message
 */
const handleRoot: HandlerFunction = async () => {
	const response: ApiResponse<{ message: string }> = {
		success: true,
		data: {
			message: "Welcome to Spikard WASM with Rollup!",
		},
		timestamp: new Date().toISOString(),
	};

	return {
		statusCode: 200,
		headers: {
			"content-type": "application/json",
		},
		body: response,
	};
};

/**
 * GET /api/data - Return sample framework data
 */
const handleData: HandlerFunction = async () => {
	const data: DataObject = {
		id: "spikard-001",
		name: "Spikard WASM Framework",
		version: "0.3.7",
	};

	const response: ApiResponse<DataObject> = {
		success: true,
		data,
		timestamp: new Date().toISOString(),
	};

	return {
		statusCode: 200,
		headers: {
			"content-type": "application/json",
		},
		body: response,
	};
};

/**
 * POST /api/echo - Echo back the request body
 */
const handleEcho: HandlerFunction = async (request: Request) => {
	let body: unknown = null;

	try {
		if (request.body) {
			body = typeof request.body === "string" ? JSON.parse(request.body) : request.body;
		}
	} catch {
		const errorResponse: ApiResponse = {
			success: false,
			error: "Invalid JSON in request body",
			timestamp: new Date().toISOString(),
		};

		return {
			statusCode: 400,
			headers: {
				"content-type": "application/json",
			},
			body: errorResponse,
		};
	}

	const echoData: EchoData = {
		message: typeof body === "string" ? body : JSON.stringify(body),
		receivedAt: new Date().toISOString(),
	};

	const response: ApiResponse<EchoData> = {
		success: true,
		data: echoData,
		timestamp: new Date().toISOString(),
	};

	return {
		statusCode: 200,
		headers: {
			"content-type": "application/json",
		},
		body: response,
	};
};

/**
 * Initialize and export the Spikard application
 */
const app = new Spikard();

get("/")(handleRoot);
get("/api/data")(handleData);
post("/api/echo")(handleEcho);

/**
 * Export the application for use in different environments
 */
export default app;

if (typeof process !== "undefined" && process.env.NODE_ENV !== "test") {
	app.run({ port: 8080, host: "127.0.0.1" });
}
