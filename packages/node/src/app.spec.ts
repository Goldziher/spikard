/**
 * Unit tests for Spikard application class
 */

import { beforeEach, describe, expect, it } from "vitest";
import { Spikard } from "./app";
import type { RouteMetadata } from "./index";

describe("Spikard", () => {
	let app: Spikard;

	beforeEach(() => {
		app = new Spikard();
	});

	it("should create an empty application", () => {
		expect(app.routes).toEqual([]);
		expect(app.handlers).toEqual({});
	});

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

		routes.forEach((route) => {
			app.addRoute(route as RouteMetadata, async () => ({}));
		});

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
