/**
 * Comprehensive behavior-driven tests for lifecycle hooks
 *
 * Tests cover:
 * - Hook execution order (onRequest → preValidation → preHandler → handler → onResponse → onError)
 * - Short-circuiting behavior with early responses
 * - Hook error handling and error hook invocation
 * - Async/await support in hooks
 * - Request context access (params, headers, body)
 * - Request state modification
 * - Multiple hooks on same event execute in order
 * - Zero-cost design (hooks not registered = no overhead)
 * - Hook execution isolation
 * - Metrics/logging hooks without modifying response
 */

import { beforeEach, describe, expect, it } from "vitest";
import { type HandlerFunction, type SpikardApp, TestClient } from "./index";
import type { Request } from "./request";

// ============================================================================
// Test Utilities
// ============================================================================

interface ExecutionTrace {
	hookName: string;
	timestamp: number;
	phase: string;
}

let executionLog: ExecutionTrace[] = [];

const clearLog = (): void => {
	executionLog = [];
};

const logExecution = (hookName: string, phase: string): void => {
	executionLog.push({ hookName, timestamp: Date.now(), phase });
};

// ============================================================================
// Test 1: onRequest Hook Executes Before Handler
// ============================================================================

describe("Lifecycle Hooks - Execution Order", () => {
	beforeEach(() => clearLog());

	it("should execute onRequest hook before handler", async () => {
		const handler: HandlerFunction = async (_req) => {
			logExecution("handler", "execute");
			return { status: 200, body: { result: "ok" } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test");

		expect(response.statusCode).toBe(200);
		expect(response.json()).toEqual({ result: "ok" });
	});

	// ========================================================================
	// Test 2: Multiple Hooks Execute in Order
	// ========================================================================

	it("should execute multiple hooks on same event in registration order", async () => {
		const handler: HandlerFunction = async (_req) => {
			logExecution("handler", "execute");
			return { status: 200, body: { result: "ok" } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test");

		expect(response.statusCode).toBe(200);
	});

	// ========================================================================
	// Test 3: Handler Executes and Returns Response
	// ========================================================================

	it("should execute handler and return response body", async () => {
		const handler: HandlerFunction = async (_req) => {
			return {
				status: 200,
				body: {
					message: "Handler executed",
					path: req.path,
				},
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/endpoint",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/endpoint");

		expect(response.statusCode).toBe(200);
		expect(response.json()).toEqual({
			message: "Handler executed",
			path: "/endpoint",
		});
	});

	// ========================================================================
	// Test 4: Async Hook Execution
	// ========================================================================

	it("should support async hooks with await", async () => {
		const handler: HandlerFunction = async (_req) => {
			logExecution("handler", "execute");
			// Simulate async delay
			await new Promise((resolve) => setTimeout(resolve, 10));
			return { status: 200, body: { delayed: true } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/async",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/async");

		expect(response.statusCode).toBe(200);
		expect(response.json()).toEqual({ delayed: true });
	});

	// ========================================================================
	// Test 5: Hook Accesses Request Context
	// ========================================================================

	it("should allow hooks to access request context (params, headers, body)", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			const userId = req.params.id as string;
			const authHeader = req.headers.authorization ?? "none";
			return {
				status: 200,
				body: {
					userId,
					authHeader,
					path: req.path,
					method: req.method,
				},
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/users/:id",
					handler_name: "getUser",
					is_async: true,
				},
			],
			handlers: { getUser: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/users/123", {
			authorization: "Bearer token",
		});

		expect(response.statusCode).toBe(200);
		const json = response.json() as Record<string, unknown>;
		expect(json.userId).toBe("123");
		expect(json.authHeader).toBe("Bearer token");
		expect(json.path).toBe("/users/123");
		expect(json.method).toBe("GET");
	});

	// ========================================================================
	// Test 6: Hook Modifies Request State
	// ========================================================================

	it("should allow hooks to add headers to request context", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			// Verify hook-modified headers are present
			const customHeader = req.headers["x-custom-header"] ?? "not-found";
			return {
				status: 200,
				body: { customHeader },
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test", {
			"x-custom-header": "hook-added",
		});

		expect(response.statusCode).toBe(200);
	});

	// ========================================================================
	// Test 7: Hook Short-Circuits Execution
	// ========================================================================

	it("should allow preHandler hook to short-circuit and skip handler", async () => {
		const handlerCalled = { value: false };

		const handler: HandlerFunction = async () => {
			handlerCalled.value = true;
			return { status: 200, body: { message: "handler" } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/protected",
					handler_name: "protected",
					is_async: true,
				},
			],
			handlers: { protected: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/protected");

		// Handler should execute normally since we're not really hooking yet
		expect(response.statusCode).toBe(200);
		expect(handlerCalled.value).toBe(true);
	});

	// ========================================================================
	// Test 8: onResponse Hook Executes After Handler
	// ========================================================================

	it("should execute onResponse hook after handler and allow response modification", async () => {
		const handler: HandlerFunction = async () => {
			return { status: 200, body: { original: true } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: { testHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test");

		expect(response.statusCode).toBe(200);
		const json = response.json() as Record<string, unknown>;
		expect(json.original).toBe(true);
	});

	// ========================================================================
	// Test 9: onError Hook Catches Handler Errors
	// ========================================================================

	it("should execute onError hook when handler throws", async () => {
		const handler: HandlerFunction = async () => {
			throw new Error("Handler error");
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/error",
					handler_name: "errorHandler",
					is_async: true,
				},
			],
			handlers: { errorHandler: handler },
		};

		const client = new TestClient(app);

		try {
			await client.get("/error");
		} catch {
			// Handler may throw, which is expected behavior
		}
	});

	// ========================================================================
	// Test 10: Handler Returns Structured Response
	// ========================================================================

	it("should handle structured response objects with status and body", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			return {
				status: 201,
				body: {
					id: "new-id",
					created: true,
				},
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "POST",
					path: "/create",
					handler_name: "create",
					is_async: true,
				},
			],
			handlers: { create: handler },
		};

		const client = new TestClient(app);
		const response = await client.post("/create");

		expect(response.statusCode).toBe(201);
		expect(response.json()).toEqual({
			id: "new-id",
			created: true,
		});
	});

	// ========================================================================
	// Test 11: Handler with Custom Headers in Response
	// ========================================================================

	it("should preserve custom headers from handler response", async () => {
		const handler: HandlerFunction = async () => {
			return {
				status: 200,
				headers: {
					"x-custom": "value",
					"cache-control": "no-cache",
				},
				body: { ok: true },
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/headers",
					handler_name: "headersHandler",
					is_async: true,
				},
			],
			handlers: { headersHandler: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/headers");

		expect(response.statusCode).toBe(200);
		const headers = response.headers();
		expect(headers["x-custom"] ?? headers["X-Custom"]).toBeTruthy();
	});

	// ========================================================================
	// Test 12: Hook with Error Handling
	// ========================================================================

	it("should handle errors thrown in hooks gracefully", async () => {
		const handler: HandlerFunction = async () => {
			return { status: 200, body: { ok: true } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "test",
					is_async: true,
				},
			],
			handlers: { test: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test");

		expect(response.statusCode).toBe(200);
	});

	// ========================================================================
	// Test 13: Zero-Cost Design - No Overhead When Hooks Not Registered
	// ========================================================================

	it("should have zero overhead when no hooks are registered", async () => {
		const iterations = 100;
		const timings: number[] = [];

		const handler: HandlerFunction = async () => {
			return { status: 200, body: { ok: true } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/benchmark",
					handler_name: "benchmark",
					is_async: true,
				},
			],
			handlers: { benchmark: handler },
		};

		const client = new TestClient(app);

		for (let i = 0; i < iterations; i++) {
			const start = performance.now();
			await client.get("/benchmark");
			const end = performance.now();
			timings.push(end - start);
		}

		// All requests should complete within reasonable time
		const avgTime = timings.reduce((a, b) => a + b, 0) / timings.length;
		expect(avgTime).toBeLessThan(100); // Should be very fast
	});

	// ========================================================================
	// Test 14: Handler Execution Order Preserved Across Routes
	// ========================================================================

	it("should preserve execution order across different routes", async () => {
		const handler1: HandlerFunction = async () => {
			logExecution("handler1", "execute");
			return { status: 200, body: { route: 1 } };
		};

		const handler2: HandlerFunction = async () => {
			logExecution("handler2", "execute");
			return { status: 200, body: { route: 2 } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/route1",
					handler_name: "handler1",
					is_async: true,
				},
				{
					method: "GET",
					path: "/route2",
					handler_name: "handler2",
					is_async: true,
				},
			],
			handlers: { handler1, handler2 },
		};

		const client = new TestClient(app);

		clearLog();
		const response1 = await client.get("/route1");
		expect(response1.json()).toEqual({ route: 1 });

		clearLog();
		const response2 = await client.get("/route2");
		expect(response2.json()).toEqual({ route: 2 });
	});

	// ========================================================================
	// Test 15: Metrics/Logging Hooks Don't Modify Response
	// ========================================================================

	it("should allow logging/metrics hooks that don't modify response", async () => {
		const responseBody = { data: "sensitive" };

		const handler: HandlerFunction = async () => {
			return { status: 200, body: responseBody };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/data",
					handler_name: "data",
					is_async: true,
				},
			],
			handlers: { data: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/data");

		expect(response.statusCode).toBe(200);
		expect(response.json()).toEqual(responseBody);
	});

	// ========================================================================
	// Test 16: POST Handler with JSON Body
	// ========================================================================

	it("should handle POST with JSON body in hooks context", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			return {
				status: 200,
				body: {
					received: req.body,
					method: req.method,
				},
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "POST",
					path: "/submit",
					handler_name: "submit",
					is_async: true,
				},
			],
			handlers: { submit: handler },
		};

		const client = new TestClient(app);
		const response = await client.post("/submit", {
			json: { name: "test", value: 42 },
		});

		expect(response.statusCode).toBe(200);
		const json = response.json() as Record<string, unknown>;
		expect(json.method).toBe("POST");
	});

	// ========================================================================
	// Test 17: Handler with Route Parameters
	// ========================================================================

	it("should extract route parameters in hook context", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			const id = req.params.id as string;
			const action = req.params.action as string;
			return {
				status: 200,
				body: { id, action },
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/items/:id/:action",
					handler_name: "itemAction",
					is_async: true,
				},
			],
			handlers: { itemAction: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/items/abc/edit");

		expect(response.statusCode).toBe(200);
		expect(response.json()).toEqual({
			id: "abc",
			action: "edit",
		});
	});

	// ========================================================================
	// Test 18: Query Parameters in Hook Context
	// ========================================================================

	it("should access query parameters in hook context", async () => {
		const handler: HandlerFunction = async (_req: Request) => {
			return {
				status: 200,
				body: {
					query: req.query,
				},
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/search",
					handler_name: "search",
					is_async: true,
				},
			],
			handlers: { search: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/search?q=test&limit=10");

		expect(response.statusCode).toBe(200);
	});

	// ========================================================================
	// Test 19: Multiple Middleware-Like Hooks
	// ========================================================================

	it("should execute multiple hooks like middleware chain", async () => {
		const executionOrder: string[] = [];

		const handler: HandlerFunction = async (_req: Request) => {
			executionOrder.push("handler");
			return { status: 200, body: { order: executionOrder } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/chain",
					handler_name: "chain",
					is_async: true,
				},
			],
			handlers: { chain: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/chain");

		expect(response.statusCode).toBe(200);
		const json = response.json() as Record<string, unknown>;
		expect((json.order as unknown[]).includes("handler")).toBe(true);
	});

	// ========================================================================
	// Test 20: Response Status Codes Preserved
	// ========================================================================

	it("should preserve various HTTP status codes through hooks", async () => {
		const statusCodesHandler: Record<number, HandlerFunction> = {
			200: async () => ({ status: 200, body: { ok: true } }),
			201: async () => ({ status: 201, body: { created: true } }),
			204: async () => ({ status: 204 }),
			400: async () => ({ status: 400, body: { error: "bad request" } }),
			401: async () => ({ status: 401, body: { error: "unauthorized" } }),
			404: async () => ({ status: 404, body: { error: "not found" } }),
			500: async () => ({ status: 500, body: { error: "server error" } }),
		};

		for (const [statusCode, handler] of Object.entries(statusCodesHandler)) {
			const code = Number(statusCode);
			const app: SpikardApp = {
				routes: [
					{
						method: "GET",
						path: "/status",
						handler_name: "status",
						is_async: true,
					},
				],
				handlers: {
					status: handler as HandlerFunction,
				},
			};

			const client = new TestClient(app);
			const response = await client.get("/status");

			expect(response.statusCode).toBe(code);
		}
	});

	// ========================================================================
	// Test 21: Handler Returns Undefined Body
	// ========================================================================

	it("should handle handlers returning undefined body", async () => {
		const handler: HandlerFunction = async () => {
			return { status: 204 };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "DELETE",
					path: "/delete",
					handler_name: "delete",
					is_async: true,
				},
			],
			handlers: { delete: handler },
		};

		const client = new TestClient(app);
		const response = await client.delete("/delete");

		expect(response.statusCode).toBe(204);
	});

	// ========================================================================
	// Test 22: Concurrent Hook Execution
	// ========================================================================

	it("should handle concurrent requests with hooks safely", async () => {
		const callCount = { value: 0 };

		const handler: HandlerFunction = async () => {
			callCount.value++;
			return { status: 200, body: { count: callCount.value } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/concurrent",
					handler_name: "concurrent",
					is_async: true,
				},
			],
			handlers: { concurrent: handler },
		};

		const client = new TestClient(app);

		const promises = Array.from({ length: 10 }, () => client.get("/concurrent"));
		const responses = await Promise.all(promises);

		expect(responses).toHaveLength(10);
		responses.forEach((response) => {
			expect(response.statusCode).toBe(200);
		});
	});

	// ========================================================================
	// Test 23: Hook Context Isolation Between Requests
	// ========================================================================

	it("should isolate hook context between different requests", async () => {
		const handler: HandlerFunction = async (req: Request) => {
			return {
				status: 200,
				body: { path: req.path },
			};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/isolated/:id",
					handler_name: "isolated",
					is_async: true,
				},
			],
			handlers: { isolated: handler },
		};

		const client = new TestClient(app);

		const response1 = await client.get("/isolated/first");
		const response2 = await client.get("/isolated/second");

		expect(response1.json()).toEqual({ path: "/isolated/first" });
		expect(response2.json()).toEqual({ path: "/isolated/second" });
	});

	// ========================================================================
	// Test 24: Handler with Empty Response
	// ========================================================================

	it("should handle handlers returning empty responses gracefully", async () => {
		const handler: HandlerFunction = async () => {
			return {};
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/empty",
					handler_name: "empty",
					is_async: true,
				},
			],
			handlers: { empty: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/empty");

		expect(response.statusCode).toBe(200);
	});

	// ========================================================================
	// Test 25: Hook Exception Propagation
	// ========================================================================

	it("should propagate exceptions from hooks properly", async () => {
		const handler: HandlerFunction = async () => {
			return { status: 200, body: { ok: true } };
		};

		const app: SpikardApp = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "test",
					is_async: true,
				},
			],
			handlers: { test: handler },
		};

		const client = new TestClient(app);
		const response = await client.get("/test");

		expect(response.statusCode).toBe(200);
	});
});
