/**
 * Unit tests for Spikard application class
 */

import { beforeEach, describe, expect, it } from "vitest";
import { type LifecycleHookFunction, Spikard } from "./app";
import type { RouteMetadata, SpikardApp } from "./index";

describe("Spikard", () => {
	let app: Spikard;

	beforeEach(() => {
		app = new Spikard();
	});

	describe("Initialization", () => {
		it("should create an empty application", () => {
			expect(app.routes).toEqual([]);
			expect(app.handlers).toEqual({});
		});

		it("should initialize lifecycle hooks as empty arrays", () => {
			const hooks = app.getLifecycleHooks();
			expect(hooks.onRequest).toEqual([]);
			expect(hooks.preValidation).toEqual([]);
			expect(hooks.preHandler).toEqual([]);
			expect(hooks.onResponse).toEqual([]);
			expect(hooks.onError).toEqual([]);
		});
	});

	describe("Route Management", () => {
		it("should add a route with metadata", () => {
			const metadata: RouteMetadata = {
				method: "GET",
				path: "/test",
				handler_name: "testHandler",
				is_async: true,
			};

			const handler = async () => ({ message: "test" });
			app.addRoute(metadata, handler);

			expect(app.routes).toHaveLength(1);
			expect(app.routes[0]).toEqual(metadata);
			expect(app.handlers.testHandler).toBe(handler);
		});

		it("should add multiple routes", () => {
			const route1: RouteMetadata = {
				method: "GET",
				path: "/route1",
				handler_name: "handler1",
				is_async: true,
			};

			const route2: RouteMetadata = {
				method: "POST",
				path: "/route2",
				handler_name: "handler2",
				is_async: true,
			};

			const handler1 = async () => ({ id: 1 });
			const handler2 = async () => ({ id: 2 });

			app.addRoute(route1, handler1);
			app.addRoute(route2, handler2);

			expect(app.routes).toHaveLength(2);
			expect(app.handlers.handler1).toBe(handler1);
			expect(app.handlers.handler2).toBe(handler2);
		});

		it("should handle synchronous handlers", () => {
			const metadata: RouteMetadata = {
				method: "GET",
				path: "/sync",
				handler_name: "syncHandler",
				is_async: false,
			};

			const handler = () => ({ sync: true });
			app.addRoute(metadata, handler);

			expect(app.handlers.syncHandler).toBe(handler);
		});

		it("should preserve route order", () => {
			const routes = [
				{ method: "GET", path: "/a", handler_name: "a", is_async: true },
				{ method: "GET", path: "/b", handler_name: "b", is_async: true },
				{ method: "GET", path: "/c", handler_name: "c", is_async: true },
			];

			for (const route of routes) {
				app.addRoute(route as RouteMetadata, async () => ({}));
			}

			expect(app.routes.map((r) => r.path)).toEqual(["/a", "/b", "/c"]);
		});

		it("should allow handler name reuse (last one wins)", () => {
			const metadata1: RouteMetadata = {
				method: "GET",
				path: "/first",
				handler_name: "shared",
				is_async: true,
			};

			const metadata2: RouteMetadata = {
				method: "POST",
				path: "/second",
				handler_name: "shared",
				is_async: true,
			};

			const handler1 = async () => ({ version: 1 });
			const handler2 = async () => ({ version: 2 });

			app.addRoute(metadata1, handler1);
			app.addRoute(metadata2, handler2);

			expect(app.handlers.shared).toBe(handler2);
		});
	});

	describe("onRequest Hook", () => {
		it("should register an onRequest hook", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			const returned = app.onRequest(hook);

			expect(returned).toBe(hook);
			expect(app.getLifecycleHooks().onRequest).toContain(hook);
		});

		it("should register multiple onRequest hooks", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;

			app.onRequest(hook1);
			app.onRequest(hook2);

			const hooks = app.getLifecycleHooks().onRequest;
			expect(hooks).toHaveLength(2);
			expect(hooks).toContain(hook1);
			expect(hooks).toContain(hook2);
		});

		it("should allow sync hooks", () => {
			const hook: LifecycleHookFunction = (payload) => payload;
			app.onRequest(hook);

			const hooks = app.getLifecycleHooks().onRequest;
			expect(hooks).toHaveLength(1);
		});
	});

	describe("preValidation Hook", () => {
		it("should register a preValidation hook", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			const returned = app.preValidation(hook);

			expect(returned).toBe(hook);
			expect(app.getLifecycleHooks().preValidation).toContain(hook);
		});

		it("should register multiple preValidation hooks", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;

			app.preValidation(hook1);
			app.preValidation(hook2);

			const hooks = app.getLifecycleHooks().preValidation;
			expect(hooks).toHaveLength(2);
		});
	});

	describe("preHandler Hook", () => {
		it("should register a preHandler hook", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			const returned = app.preHandler(hook);

			expect(returned).toBe(hook);
			expect(app.getLifecycleHooks().preHandler).toContain(hook);
		});

		it("should register multiple preHandler hooks", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;
			const hook3: LifecycleHookFunction = async (payload) => payload;

			app.preHandler(hook1);
			app.preHandler(hook2);
			app.preHandler(hook3);

			const hooks = app.getLifecycleHooks().preHandler;
			expect(hooks).toHaveLength(3);
		});
	});

	describe("onResponse Hook", () => {
		it("should register an onResponse hook", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			const returned = app.onResponse(hook);

			expect(returned).toBe(hook);
			expect(app.getLifecycleHooks().onResponse).toContain(hook);
		});

		it("should register multiple onResponse hooks", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;

			app.onResponse(hook1);
			app.onResponse(hook2);

			const hooks = app.getLifecycleHooks().onResponse;
			expect(hooks).toHaveLength(2);
		});
	});

	describe("onError Hook", () => {
		it("should register an onError hook", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			const returned = app.onError(hook);

			expect(returned).toBe(hook);
			expect(app.getLifecycleHooks().onError).toContain(hook);
		});

		it("should register multiple onError hooks", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;

			app.onError(hook1);
			app.onError(hook2);

			const hooks = app.getLifecycleHooks().onError;
			expect(hooks).toHaveLength(2);
		});
	});

	describe("getLifecycleHooks", () => {
		it("should return a copy of lifecycle hooks", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			app.onRequest(hook);

			const hooks1 = app.getLifecycleHooks();
			const hooks2 = app.getLifecycleHooks();

			expect(hooks1.onRequest).not.toBe(hooks2.onRequest);

			expect(hooks1.onRequest).toEqual(hooks2.onRequest);
		});

		it("should return independent copies of all hook arrays", () => {
			const hook1: LifecycleHookFunction = async (payload) => payload;
			const hook2: LifecycleHookFunction = async (payload) => payload;
			const hook3: LifecycleHookFunction = async (payload) => payload;

			app.onRequest(hook1);
			app.preValidation(hook2);
			app.preHandler(hook3);

			const hooks = app.getLifecycleHooks();
			expect(hooks.onRequest).toHaveLength(1);
			expect(hooks.preValidation).toHaveLength(1);
			expect(hooks.preHandler).toHaveLength(1);
			expect(hooks.onResponse).toHaveLength(0);
			expect(hooks.onError).toHaveLength(0);
		});

		it("should not modify original hooks when modifying returned copy", () => {
			const hook: LifecycleHookFunction = async (payload) => payload;
			app.onRequest(hook);

			const hooks = app.getLifecycleHooks();
			const originalCount = hooks.onRequest.length;

			const newHook: LifecycleHookFunction = async (payload) => payload;
			hooks.onRequest.push(newHook);

			const originalHooks = app.getLifecycleHooks();
			expect(originalHooks.onRequest).toHaveLength(originalCount);
		});
	});

	describe("Mixed Lifecycle Operations", () => {
		it("should handle all hook types together", () => {
			const hooks = Array.from({ length: 5 }, async (_, i) => async (payload: unknown) => ({
				...payload,
				hookIndex: i,
			}));

			app.onRequest(hooks[0]);
			app.preValidation(hooks[1]);
			app.preHandler(hooks[2]);
			app.onResponse(hooks[3]);
			app.onError(hooks[4]);

			const registered = app.getLifecycleHooks();
			expect(registered.onRequest).toHaveLength(1);
			expect(registered.preValidation).toHaveLength(1);
			expect(registered.preHandler).toHaveLength(1);
			expect(registered.onResponse).toHaveLength(1);
			expect(registered.onError).toHaveLength(1);
		});

		it("should maintain hook order", () => {
			const hooks: LifecycleHookFunction[] = [];
			for (let i = 0; i < 5; i++) {
				hooks.push(async (payload) => payload);
			}

			for (const hook of hooks) {
				app.onRequest(hook);
			}

			const registered = app.getLifecycleHooks().onRequest;
			expect(registered).toHaveLength(5);
			for (let i = 0; i < hooks.length; i++) {
				expect(registered[i]).toBe(hooks[i]);
			}
		});
	});

	describe("SpikardApp interface compliance", () => {
		it("should conform to SpikardApp interface", () => {
			const app: SpikardApp = new Spikard();
			expect(app).toHaveProperty("routes");
			expect(app).toHaveProperty("handlers");
			expect(app).toHaveProperty("websocketRoutes");
			expect(app).toHaveProperty("websocketHandlers");
			expect(app.routes).toBeInstanceOf(Array);
			expect(app.websocketRoutes).toBeInstanceOf(Array);
			expect(typeof app.handlers).toBe("object");
			expect(typeof app.websocketHandlers).toBe("object");
		});
	});

	describe("WebSocket routes", () => {
		it("should register websocket handlers", () => {
			app.websocket("/ws", async (message) => ({ echo: message }));

			expect(app.websocketRoutes).toHaveLength(1);
			expect(Object.keys(app.websocketHandlers)).toHaveLength(1);
			expect(app.websocketRoutes[0]?.path).toBe("/ws");
		});
	});

	describe("Handler overwriting", () => {
		it("should overwrite existing handler with same name", () => {
			const metadata: RouteMetadata = {
				method: "GET",
				path: "/test",
				handler_name: "myHandler",
				is_async: true,
			};

			const handler1 = async () => ({ version: 1 });
			const handler2 = async () => ({ version: 2 });

			app.addRoute(metadata, handler1);
			app.addRoute(metadata, handler2);

			expect(app.handlers.myHandler).toBe(handler2);
		});
	});

	describe("Large-scale operations", () => {
		it("should handle many routes", () => {
			const routeCount = 100;
			for (let i = 0; i < routeCount; i++) {
				const metadata: RouteMetadata = {
					method: "GET",
					path: `/route${i}`,
					handler_name: `handler${i}`,
					is_async: true,
				};
				app.addRoute(metadata, async () => ({ index: i }));
			}

			expect(app.routes).toHaveLength(routeCount);
			expect(Object.keys(app.handlers)).toHaveLength(routeCount);
		});

		it("should handle many lifecycle hooks", () => {
			const hookCount = 50;
			for (let i = 0; i < hookCount; i++) {
				app.onRequest(async (payload) => payload);
			}

			const hooks = app.getLifecycleHooks();
			expect(hooks.onRequest).toHaveLength(hookCount);
		});
	});
});
