import { Spikard, TestClient } from "@spikard/wasm";

/**
 * WASM test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 *
 * Uses TestClient for in-memory testing without actual HTTP server.
 */

export function createApp() {
	const app = new Spikard();

	// Health check endpoint
	const health = async (_req) => {
		return {
			status: 200,
			body: { status: "ok" },
		};
	};
	app.addRoute({ method: "GET", path: "/health", handler_name: "health", is_async: true }, health);

	// Query parameters endpoint
	const query = async (req) => {
		return {
			name: req.query?.name ?? null,
			age: req.query?.age ? parseInt(String(req.query.age)) : null,
		};
	};
	app.addRoute({ method: "GET", path: "/query", handler_name: "query", is_async: true }, query);

	// JSON echo endpoint
	const echo = async (req) => {
		const body = req.json();
		return {
			received: body,
			method: req.method,
		};
	};
	app.addRoute({ method: "POST", path: "/echo", handler_name: "echo", is_async: true }, echo);

	// Path parameters endpoint
	const user = async (req) => {
		const userId = req.pathParams?.id;
		return {
			userId,
			type: typeof userId,
		};
	};
	app.addRoute({ method: "GET", path: "/users/{id}", handler_name: "user", is_async: true }, user);

	// Return app and test client
	return {
		app,
		async start() {
			// TestClient doesn't require start
		},
		async stop() {
			// TestClient doesn't require stop
		},
		address() {
			return { host: "127.0.0.1", port: 8000 };
		},
		async getTestClient() {
			return new TestClient(app);
		},
	};
}
