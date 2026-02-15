import { Spikard } from "@spikard/node";
import type { Request, HandlerResult } from "@spikard/node";

/**
 * Node.js test application for published @spikard/node package.
 *
 * Tests the PUBLISHED package from npm (0.10.1).
 */

// Health check handler
function health(request: Request): HandlerResult {
	return {
		status: 200,
		body: { status: "ok" },
	};
}

// Query parameters handler
function query(request: Request): HandlerResult {
	const name = request.query?.name ?? "";
	const ageStr = request.query?.age ?? "0";
	const age = parseInt(ageStr as string, 10);
	return {
		status: 200,
		body: { name, age },
	};
}

// Echo POST handler
function echo(request: Request): HandlerResult {
	let body: unknown = null;
	try {
		body = request.json();
	} catch {
		body = request.body?.toString();
	}
	return {
		status: 200,
		body: {
			received: body,
			method: "POST",
		},
	};
}

// Path parameters handler
function user(request: Request): HandlerResult {
	const userId = request.pathParams?.id ?? "";
	return {
		status: 200,
		body: {
			userId,
			type: "string",
		},
	};
}

// PUT handler
function putItem(request: Request): HandlerResult {
	const itemId = request.pathParams?.id ?? "";
	let body: unknown = null;
	try {
		body = request.json();
	} catch {
		body = request.body?.toString();
	}
	return {
		status: 200,
		body: { itemId, updated: body, method: "PUT" },
	};
}

// DELETE handler
function deleteItem(request: Request): HandlerResult {
	const itemId = request.pathParams?.id ?? "";
	return {
		status: 200,
		body: { itemId, deleted: true, method: "DELETE" },
	};
}

// PATCH handler
function patchItem(request: Request): HandlerResult {
	const itemId = request.pathParams?.id ?? "";
	let body: unknown = null;
	try {
		body = request.json();
	} catch {
		body = request.body?.toString();
	}
	return {
		status: 200,
		body: { itemId, patched: body, method: "PATCH" },
	};
}

// Headers handler
function headers(request: Request): HandlerResult {
	const customHeader = request.headers?.["x-custom-header"] ?? "";
	return {
		status: 200,
		body: { "x-custom-header": customHeader },
	};
}

// Cookies handler
function cookies(request: Request): HandlerResult {
	const cookieHeader = request.headers?.cookie ?? "";
	const sessionMatch = cookieHeader.match(/session=([^;]*)/);
	const session = sessionMatch ? sessionMatch[1] : "";
	return {
		status: 200,
		body: { session },
	};
}

// Error handler
function errorHandler(request: Request): HandlerResult {
	throw new Error("Intentional error");
}

export function createApp(): Spikard {
	const app = new Spikard();

	// Register all routes
	app.addRoute(
		{
			method: "GET",
			path: "/health",
			handler_name: "health",
			is_async: false,
		},
		health,
	);

	app.addRoute(
		{
			method: "GET",
			path: "/query",
			handler_name: "query",
			is_async: false,
		},
		query,
	);

	app.addRoute(
		{
			method: "POST",
			path: "/echo",
			handler_name: "echo",
			is_async: false,
		},
		echo,
	);

	app.addRoute(
		{
			method: "GET",
			path: "/users/:id",
			handler_name: "user",
			is_async: false,
		},
		user,
	);

	app.addRoute(
		{
			method: "PUT",
			path: "/items/:id",
			handler_name: "putItem",
			is_async: false,
		},
		putItem,
	);

	app.addRoute(
		{
			method: "DELETE",
			path: "/items/:id",
			handler_name: "deleteItem",
			is_async: false,
		},
		deleteItem,
	);

	app.addRoute(
		{
			method: "PATCH",
			path: "/items/:id",
			handler_name: "patchItem",
			is_async: false,
		},
		patchItem,
	);

	app.addRoute(
		{
			method: "GET",
			path: "/headers",
			handler_name: "headers",
			is_async: false,
		},
		headers,
	);

	app.addRoute(
		{
			method: "GET",
			path: "/cookies",
			handler_name: "cookies",
			is_async: false,
		},
		cookies,
	);

	app.addRoute(
		{
			method: "GET",
			path: "/error",
			handler_name: "errorHandler",
			is_async: false,
		},
		errorHandler,
	);

	return app;
}

// Run if executed directly
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import { runServer } from "@spikard/node";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

if (import.meta.url === `file://${process.argv[1]}`) {
	const port = process.argv[2] ? parseInt(process.argv[2], 10) : 8000;
	const app = createApp();
	console.log(`Starting Spikard Node.js test server on http://127.0.0.1:${port}`);
	runServer(app, { host: "127.0.0.1", port });
}
