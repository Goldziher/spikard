/**
 * Adapted from https://github.com/honojs/hono/blob/main/runtime-tests/node/index.test.ts (MIT License).
 * Ensures the Spikard WASM bindings satisfy the same runtime expectations as Hono.
 */

import { Buffer } from "node:buffer";
import { gunzipSync } from "node:zlib";
import { beforeAll, describe, expect, it } from "vitest";
import type { HandlerFunction, RouteMetadata, SpikardApp } from "../../src";
import { createFetchHandler } from "../../src/server";
import { StreamingResponse } from "../../src/streaming";
import { __setGunzipImplementation, TestClient } from "../../src/testing";

interface ParsedRequest {
	method?: string;
	body?: unknown;
	json?: () => unknown;
	queryString?: string;
	params?: Record<string, unknown>;
	query?: Record<string, unknown>;
	headers?: Record<string, unknown>;
}

beforeAll(() => {
	__setGunzipImplementation((bytes) => new Uint8Array(gunzipSync(bytes)));
});

describe("Node runtime parity", () => {
	it("serves basic responses and exposes path/query params", async () => {
		const client = buildClient(
			[route("GET", "/", "root"), route("GET", "/runtime/{kind}", "runtime"), route("GET", "/search", "search")],
			{
				root: async () => jsonResponse({ message: "Hello from WASM" }),
				runtime: async (payload) => {
					const request = parseRequest(payload);
					return jsonResponse({ runtime: request.params?.kind ?? "unknown" });
				},
				search: async (payload) => {
					const request = parseRequest(payload);
					return jsonResponse({
						query: request.query?.q ?? null,
						headers: request.headers ?? {},
					});
				},
			},
		);

		const root = await client.get("/");
		expect(root.statusCode).toBe(200);
		expect(root.json()).toEqual({ message: "Hello from WASM" });

		const runtime = await client.get("/runtime/node");
		expect(runtime.statusCode).toBe(200);
		expect(runtime.json()).toEqual({ runtime: "node" });

		const search = await client.get("/search?q=spikard", {
			"X-Env": "node",
		});
		expect(search.statusCode).toBe(200);
		expect(search.json()).toEqual({
			query: "spikard",
			headers: { "X-Env": "node" },
		});
	});

	it("coerces path parameters according to schema and rejects invalid input", async () => {
		const client = buildClient(
			[
				route("GET", "/users/{id}", "userById", {
					parameter_schema: {
						type: "object",
						required: ["id"],
						properties: {
							id: { source: "path", type: "integer" },
						},
					},
				}),
			],
			{
				userById: async (payload) => {
					const request = parseRequest(payload);
					return jsonResponse({ id: request.params?.id });
				},
			},
		);

		const ok = await client.get("/users/42");
		expect(ok.statusCode).toBe(200);
		expect(ok.json()).toEqual({ id: 42 });

		const bad = await client.get("/users/not-a-number");
		expect(bad.statusCode).toBe(422);
		expect(bad.json()).toMatchObject({
			detail: "1 validation error in request",
		});
	});

	it("handles streaming responses", async () => {
		const client = buildClient([route("GET", "/stream", "streamHello")], {
			streamHello: async () =>
				new StreamingResponse(
					(async function* () {
						yield "Hello";
						yield " ";
						yield "Node";
					})(),
					{
						headers: { "content-type": "text/plain" },
					},
				),
		});

		const res = await client.get("/stream");
		expect(res.statusCode).toBe(200);
		expect(res.headers()["content-type"]).toBe("text/plain");
		expect(res.text()).toBe("Hello Node");
	});

	it("compresses large responses when accept-encoding allows it", async () => {
		const stylesheet = Array.from({ length: 64 }, () => "body { color: hotpink; }").join("\n");
		const client = buildClient(
			[route("GET", "/style.css", "style")],
			{
				style: async () =>
					jsonResponse(stylesheet, 200, {
						"content-type": "text/css",
					}),
			},
			{
				compression: {
					gzip: true,
					brotli: false,
					minSize: 32,
					quality: 4,
				},
			},
		);

		const res = await client.get("/style.css", { "Accept-Encoding": "gzip" });
		expect(res.statusCode).toBe(200);
		expect(res.headers()["content-encoding"]).toBe("gzip");
		expect(res.text()).toBe(stylesheet);
	});

	it("returns binary payloads encoded in base64", async () => {
		const pngBytes = new Uint8Array([137, 80, 78, 71]);
		const client = buildClient([route("GET", "/favicon.ico", "favicon")], {
			favicon: async () =>
				jsonResponse({ __spikard_base64__: Buffer.from(pngBytes).toString("base64") }, 200, {
					"content-type": "image/x-icon",
				}),
		});

		const res = await client.get("/favicon.ico");
		expect(res.statusCode).toBe(200);
		expect(res.headers()["content-type"]).toBe("image/x-icon");
		expect(res.bytes()).toEqual(Buffer.from(pngBytes));
	});

	it("exposes Request facade with json() helper", async () => {
		const client = buildClient([route("POST", "/users", "createUser")], {
			createUser: async (request) => {
				const req = parseRequest(request);
				const bodyData = typeof req.json === "function" ? req.json() : req.body;
				return jsonResponse({
					method: req.method,
					body: bodyData,
					query: req.queryString,
				});
			},
		});

		const res = await client.post("/users?role=admin", { json: { name: "Ada" } });
		expect(res.statusCode).toBe(200);
		expect(res.json()).toEqual({
			method: "POST",
			body: { name: "Ada" },
			query: "role=admin",
		});
	});

	it("remains compatible with legacy JSON.parse handlers", async () => {
		const client = buildClient([route("POST", "/legacy", "legacyHandler")], {
			legacyHandler: async (request) => {
				const parsed = JSON.parse(request as never as string) as { body?: unknown };
				return jsonResponse(parsed.body ?? null);
			},
		});

		const res = await client.post("/legacy", { json: { ok: true } });
		expect(res.statusCode).toBe(200);
		expect(res.json()).toEqual({ ok: true });
	});
});

describe("Fetch handler bridge", () => {
	it("handles GET requests via createFetchHandler", async () => {
		const app = buildApp([route("GET", "/hello", "hello")], {
			hello: async () => jsonResponse({ message: "hi" }),
		});
		const handler = createFetchHandler(app);
		const res = await handler(new Request("http://localhost/hello"));
		expect(res.status).toBe(200);
		expect(await res.json()).toEqual({ message: "hi" });
	});

	it("handles JSON bodies via fetch handler", async () => {
		const app = buildApp([route("POST", "/echo", "echo")], {
			echo: async (request) => {
				const req = parseRequest(request) as ParsedRequest;
				return req.json?.();
			},
		});
		const handler = createFetchHandler(app);
		const res = await handler(
			new Request("http://localhost/echo", {
				method: "POST",
				headers: { "content-type": "application/json" },
				body: JSON.stringify({ hello: "world" }),
			}),
		);
		expect(res.status).toBe(200);
		expect(await res.json()).toEqual({ hello: "world" });
	});
});

function buildClient(
	routes: RouteMetadata[],
	handlers: Record<string, HandlerFunction>,
	config?: SpikardApp["config"],
) {
	return new TestClient(buildApp(routes, handlers, config));
}

function buildApp(
	routes: RouteMetadata[],
	handlers: Record<string, HandlerFunction>,
	config?: SpikardApp["config"],
): SpikardApp {
	return {
		routes,
		handlers,
		config: config ?? undefined,
	};
}

function route(method: string, path: string, handlerName: string, extra: Partial<RouteMetadata> = {}): RouteMetadata {
	return {
		method,
		path,
		handler_name: handlerName,
		is_async: true,
		...extra,
	};
}

function parseRequest(payload: unknown): ParsedRequest {
	if (typeof payload === "string") {
		return JSON.parse(payload) as ParsedRequest;
	}
	if (payload && typeof payload === "object") {
		const request = payload as ParsedRequest;
		// If it doesn't have a json() method but has a body, create one
		// (for backwards compatibility with plain object payloads)
		if (!request.json && request.body !== undefined) {
			request.json = () => request.body;
		}
		return request;
	}
	return {};
}

function jsonResponse(body: unknown, status = 200, headers: Record<string, string> = {}) {
	return JSON.stringify({
		status,
		headers,
		body,
	});
}
