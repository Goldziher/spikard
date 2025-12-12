/**
 * Unit tests for routing decorators
 */

import { describe, expect, it } from "vitest";
import type { JsonSchema, RouteMetadata } from "./index.ts";
import { del, get, patch, post, put, route } from "./routing.ts";

/**
 * Extract route metadata from a decorated handler function.
 * Uses object property access with the readonly modifier to safely access __route_metadata__.
 */
function getRouteMetadata(handler: unknown): RouteMetadata {
	// Type the handler object with the metadata property
	const handlerWithMetadata = handler as unknown as {
		readonly __route_metadata__?: RouteMetadata;
	};
	const metadata = handlerWithMetadata.__route_metadata__;
	if (!metadata) {
		throw new TypeError("Handler does not have route metadata");
	}
	return metadata;
}

describe("Routing Decorators", () => {
	describe("route() decorator", () => {
		it("should decorate a handler function", () => {
			const handler = route("/test")(() => {
				return { message: "test" };
			});

			expect(handler).toBeDefined();
			expect(typeof handler).toBe("function");
		});

		it("should add route metadata to handler", () => {
			const handler = route("/test")(function testHandler() {
				return { message: "test" };
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata).toBeDefined();
			expect(metadata.path).toBe("/test");
			expect(metadata.handler_name).toBe("testHandler");
		});

		it("should default to GET method", () => {
			const handler = route("/test")(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("GET");
		});

		it("should accept single method string", () => {
			const handler = route("/test", { methods: "POST" })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("POST");
		});

		it("should accept multiple methods as array", () => {
			const handler = route("/test", { methods: ["GET", "POST", "PUT"] })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("GET,POST,PUT");
		});

		it("should set is_async to true", () => {
			const handler = route("/test")(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.is_async).toBe(true);
		});

		it("should preserve handler name", () => {
			const handler = route("/users")(function getUsers() {
				return [];
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata.handler_name).toBe("getUsers");
		});

		it("should handle anonymous functions", () => {
			const handler = route("/test")(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.handler_name).toBe("anonymous");
		});

		it("should accept bodySchema option", () => {
			const schema: JsonSchema = {
				type: "object",
				properties: { name: { type: "string" } },
			};

			const handler = route("/test", { bodySchema: schema })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.request_schema).toBe(schema);
		});

		it("should accept responseSchema option", () => {
			const schema: JsonSchema = { type: "object" };

			const handler = route("/test", { responseSchema: schema })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.response_schema).toBe(schema);
		});

		it("should accept parameterSchema option", () => {
			const schema: JsonSchema = { type: "object" };

			const handler = route("/test", { parameterSchema: schema })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.parameter_schema).toBe(schema);
		});

		it("should accept cors option", () => {
			const corsConfig = { allowOrigins: ["*"], allowMethods: ["GET", "POST"] };

			const handler = route("/test", { cors: corsConfig })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.cors).toBe(corsConfig);
		});

		it("should combine multiple options", () => {
			const bodySchema: JsonSchema = { type: "object" };
			const responseSchema: JsonSchema = { type: "object" };
			const corsConfig = { allowOrigins: ["*"] };

			const handler = route("/users/:id", {
				methods: ["PUT", "PATCH"],
				bodySchema,
				responseSchema,
				cors: corsConfig,
			})(function updateUser() {
				return {};
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata.path).toBe("/users/:id");
			expect(metadata.method).toBe("PUT,PATCH");
			expect(metadata.request_schema).toBe(bodySchema);
			expect(metadata.response_schema).toBe(responseSchema);
			expect(metadata.cors).toBe(corsConfig);
			expect(metadata.handler_name).toBe("updateUser");
		});

		it("should return the original handler function", () => {
			const originalHandler = () => ({ result: "test" });
			const decoratedHandler = route("/test")(originalHandler);

			expect(decoratedHandler).toBe(originalHandler);
		});
	});

	describe("HTTP method decorators", () => {
		describe("get() decorator", () => {
			it("should create a GET route", () => {
				const handler = get("/users")(function getUsers() {
					return [];
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("GET");
				expect(metadata.path).toBe("/users");
				expect(metadata.handler_name).toBe("getUsers");
			});

			it("should accept route options", () => {
				const schema: JsonSchema = { type: "object" };
				const handler = get("/users", { responseSchema: schema })(function getUsers() {
					return [];
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.response_schema).toBe(schema);
			});

			it("should not allow methods option", () => {
				// This is enforced by TypeScript, but we test the runtime behavior
				const handler = get("/test")(() => ({}));
				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("GET");
			});
		});

		describe("post() decorator", () => {
			it("should create a POST route", () => {
				const handler = post("/users")(function createUser() {
					return { id: 1 };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("POST");
				expect(metadata.path).toBe("/users");
			});

			it("should accept body schema", () => {
				const bodySchema: JsonSchema = { type: "object" };
				const handler = post("/users", { bodySchema })(function createUser() {
					return { id: 1 };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.request_schema).toBe(bodySchema);
			});
		});

		describe("put() decorator", () => {
			it("should create a PUT route", () => {
				const handler = put("/users/:id")(function updateUser() {
					return { updated: true };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("PUT");
				expect(metadata.path).toBe("/users/:id");
			});

			it("should accept multiple schemas", () => {
				const bodySchema: JsonSchema = { type: "object" };
				const responseSchema: JsonSchema = { type: "object" };

				const handler = put("/users/:id", {
					bodySchema,
					responseSchema,
				})(function updateUser() {
					return { updated: true };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.request_schema).toBe(bodySchema);
				expect(metadata.response_schema).toBe(responseSchema);
			});
		});

		describe("patch() decorator", () => {
			it("should create a PATCH route", () => {
				const handler = patch("/users/:id")(function patchUser() {
					return { patched: true };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("PATCH");
				expect(metadata.path).toBe("/users/:id");
			});
		});

		describe("del() decorator", () => {
			it("should create a DELETE route", () => {
				const handler = del("/users/:id")(function deleteUser() {
					return { deleted: true };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.method).toBe("DELETE");
				expect(metadata.path).toBe("/users/:id");
			});

			it("should work with complex paths", () => {
				const handler = del("/api/v1/users/:id/posts/:postId")(function deletePost() {
					return { deleted: true };
				});

				const metadata = getRouteMetadata(handler);
				expect(metadata.path).toBe("/api/v1/users/:id/posts/:postId");
			});
		});
	});

	describe("Path handling", () => {
		it("should preserve exact paths", () => {
			const paths = ["/", "/test", "/users/:id", "/api/v1/users/:id/posts", "/deeply/nested/path"];

			paths.forEach((path) => {
				const handler = route(path)(() => ({}));
				const metadata = getRouteMetadata(handler);
				expect(metadata.path).toBe(path);
			});
		});

		it("should handle root path", () => {
			const handler = route("/")(function root() {
				return { ok: true };
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata.path).toBe("/");
		});

		it("should handle paths with multiple parameters", () => {
			const handler = route("/users/:userId/posts/:postId/comments/:commentId")(function getComment() {
				return {};
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata.path).toContain(":userId");
			expect(metadata.path).toContain(":postId");
			expect(metadata.path).toContain(":commentId");
		});

		it("should handle query paths", () => {
			const handler = route("/search?q=:query")(function search() {
				return [];
			});

			const metadata = getRouteMetadata(handler);
			expect(metadata.path).toBe("/search?q=:query");
		});
	});

	describe("Methods formatting", () => {
		it("should format single method", () => {
			const handler = route("/test", { methods: "DELETE" })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("DELETE");
		});

		it("should format array of methods", () => {
			const handler = route("/test", { methods: ["GET", "POST"] })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("GET,POST");
		});

		it("should handle single-item array", () => {
			const handler = route("/test", { methods: ["PUT"] })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe("PUT");
		});

		it("should handle many methods", () => {
			const methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"];
			const handler = route("/test", { methods })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.method).toBe(methods.join(","));
		});
	});

	describe("Schema options", () => {
		it("should preserve complex JSON schemas", () => {
			const schema: JsonSchema = {
				type: "object",
				properties: {
					name: { type: "string" },
					age: { type: "number", minimum: 0 },
					email: { type: "string", format: "email" },
					tags: { type: "array", items: { type: "string" } },
				},
				required: ["name", "email"],
			};

			const handler = route("/test", { bodySchema: schema })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.request_schema).toEqual(schema);
		});

		it("should accept Zod-like schemas", () => {
			const schema = { parse: (x: unknown) => x } as unknown as JsonSchema;
			const handler = route("/test", { bodySchema: schema })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.request_schema).toBe(schema);
		});
	});

	describe("CORS configuration", () => {
		it("should accept CORS config", () => {
			const corsConfig = {
				allowOrigins: ["https://example.com"],
				allowMethods: ["GET", "POST"],
				allowHeaders: ["Content-Type"],
				credentials: true,
			};

			const handler = route("/test", { cors: corsConfig })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.cors).toEqual(corsConfig);
		});

		it("should accept wildcard CORS", () => {
			const corsConfig = { allowOrigins: ["*"], allowMethods: ["*"] };
			const handler = route("/test", { cors: corsConfig })(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.cors).toEqual(corsConfig);
		});
	});

	describe("Handler function preservation", () => {
		it("should preserve async handler", () => {
			const asyncHandler = async () => ({ result: "async" });
			const decorated = route("/test")(asyncHandler);

			expect(decorated).toBe(asyncHandler);
		});

		it("should preserve sync handler", () => {
			const syncHandler = () => ({ result: "sync" });
			const decorated = route("/test")(syncHandler);

			expect(decorated).toBe(syncHandler);
		});

		it("should preserve handler that returns promise", () => {
			const promiseHandler = () => Promise.resolve({ result: "promise" });
			const decorated = route("/test")(promiseHandler);

			expect(decorated).toBe(promiseHandler);
		});

		it("should allow decorated handler to be called normally", () => {
			const handler = route("/test")(function myHandler() {
				return { called: true };
			});

			const result = handler();
			expect(result).toEqual({ called: true });
		});
	});

	describe("Empty options", () => {
		it("should handle empty options object", () => {
			const handler = route("/test", {})(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.path).toBe("/test");
			expect(metadata.method).toBe("GET");
			expect(metadata.request_schema).toBeUndefined();
		});

		it("should handle undefined options", () => {
			const handler = route("/test")(() => ({}));
			const metadata = getRouteMetadata(handler);

			expect(metadata.path).toBe("/test");
		});
	});
});
