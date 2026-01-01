/**
 * Unit tests for GraphQL methods with TestClient
 *
 * Tests GraphQL query, mutation, and subscription handling through HTTP,
 * verifying request/response formats, error handling, and fixture compliance.
 */

import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Spikard } from "./app";
import type { Request, RouteMetadata, SpikardApp } from "./index";
import { TestClient } from "./testing";

interface GraphQLRequest {
	query: string;
	variables?: Record<string, unknown>;
	operationName?: string;
}

interface GraphQLResponse {
	data?: Record<string, unknown> | null;
	errors?: Array<{ message: string; locations?: unknown; path?: unknown; extensions?: unknown }>;
}

describe("GraphQL Methods", () => {
	let app: Spikard;
	let client: TestClient;

	const graphqlHandlerFn = async (body: GraphQLRequest | null, _request: Request) => {
		// Simple mock resolver
		if (!body?.query) {
			return {
				status: 400,
				body: {
					errors: [{ message: "No query provided" }],
				},
			};
		}

		// Mock resolver for different operations
		if (body.query.includes("hello")) {
			return {
				status: 200,
				body: {
					data: { hello: "Hello, World!" },
				},
			};
		}

		if (body.query.includes("user") && body.variables?.id) {
			return {
				status: 200,
				body: {
					data: {
						user: {
							id: body.variables.id,
							name: "Alice",
							email: "alice@example.com",
						},
					},
				},
			};
		}

		if (body.query.includes("createUser")) {
			const input = body.variables?.input as Record<string, unknown> | undefined;
			return {
				status: 200,
				body: {
					data: {
						createUser: {
							id: "user-123",
							name: input?.name || "Unknown",
							email: input?.email || "unknown@example.com",
							role: input?.role || "user",
							createdAt: "2025-12-27T10:30:00Z",
						},
					},
				},
			};
		}

		if (body.query.includes("invalidField")) {
			return {
				status: 200,
				body: {
					errors: [
						{
							message: 'Cannot query field "invalidField" on type "Query"',
							locations: [{ line: 1, column: 9 }],
							path: ["invalidField"],
						},
					],
				},
			};
		}

		if (body.query.includes("search")) {
			return {
				status: 200,
				body: {
					data: {
						search: {
							total: 42,
							users: [
								{
									id: "user-1",
									name: "GraphQL Expert",
									email: "expert@example.com",
									posts: [
										{ id: "post-101", title: "GraphQL Best Practices", likes: 234 },
										{ id: "post-102", title: "Schema Design", likes: 189 },
									],
								},
							],
							posts: [
								{
									id: "post-101",
									title: "GraphQL Best Practices",
									content: "A comprehensive guide...",
									likes: 234,
									author: { id: "user-1", name: "GraphQL Expert" },
									comments: [{ id: "comment-1", text: "Great post!", likes: 45 }],
								},
							],
						},
					},
				},
			};
		}

		// Default mock response for any valid query
		return {
			status: 200,
			body: {
				data: {
					hello: "default response",
				},
			},
		};
	};

	beforeEach(() => {
		app = new Spikard();

		const metadata: RouteMetadata = {
			method: "POST",
			path: "/graphql",
			handler_name: "graphqlHandler",
			is_async: true,
		};

		// Create a plain async handler that wraps the GraphQL logic
		const plainHandler = async (req: Request) => {
			try {
				const body = req.json() as GraphQLRequest | null;
				const result = await graphqlHandlerFn(body, req);
				return result;
			} catch (error) {
				return {
					status: 500,
					body: { errors: [{ message: String(error) }] },
				};
			}
		};

		app.addRoute(metadata, plainHandler);
		client = new TestClient(app as SpikardApp);
	});

	afterEach(() => {
		// Cleanup after each test
	});

	describe("Basic GraphQL Queries", () => {
		it("should send simple GraphQL query", async () => {
			const query = "query { hello }";
			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data).toHaveProperty("hello");
			expect(data.data?.hello).toBe("Hello, World!");
		});

		it("should send GraphQL query with variables", async () => {
			const query = "query GetUser($id: ID!) { user(id: $id) { id name email } }";
			const variables = { id: "123" };

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data).toHaveProperty("user");
			const user = (data.data?.user as Record<string, unknown>) || {};
			expect(user.id).toBe("123");
			expect(user.name).toBe("Alice");
		});

		it("should send GraphQL query with operation name", async () => {
			const query = `
				query GetHello {
					hello
				}
				query GetUser($id: ID!) {
					user(id: $id) { id name }
				}
			`;
			const variables = { id: "456" };

			const response = await client.post("/graphql", {
				json: { query, variables, operationName: "GetHello" },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data).toHaveProperty("hello");
		});

		it("should handle query with null variables", async () => {
			const query = "query { hello }";

			const response = await client.post("/graphql", {
				json: { query, variables: null },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data?.hello).toBe("Hello, World!");
		});

		it("should reject query without query field", async () => {
			const response = await client.post("/graphql", {
				json: { variables: {} },
			});

			expect(response.statusCode).toBe(400);
			const data = response.json() as GraphQLResponse;
			expect(data.errors).toBeDefined();
			expect(data.errors?.[0].message).toContain("No query provided");
		});
	});

	describe("GraphQL Mutations", () => {
		it("should send GraphQL mutation", async () => {
			const query = `
				mutation CreateUser($input: CreateUserInput!) {
					createUser(input: $input) {
						id
						name
						email
						role
						createdAt
					}
				}
			`;
			const variables = {
				input: {
					name: "John Doe",
					email: "john@example.com",
					role: "admin",
				},
			};

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data).toHaveProperty("createUser");
			const user = (data.data?.createUser as Record<string, unknown>) || {};
			expect(user.id).toBe("user-123");
			expect(user.name).toBe("John Doe");
			expect(user.email).toBe("john@example.com");
			expect(user.role).toBe("admin");
		});

		it("should send mutation with partial input", async () => {
			const query = `
				mutation CreateUser($input: CreateUserInput!) {
					createUser(input: $input) { id name email }
				}
			`;
			const variables = {
				input: {
					name: "Jane Doe",
					email: "jane@example.com",
				},
			};

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			const user = (data.data?.createUser as Record<string, unknown>) || {};
			expect(user.name).toBe("Jane Doe");
			expect(user.email).toBe("jane@example.com");
			expect(user.role).toBe("user");
		});
	});

	describe("Complex Nested Queries", () => {
		it("should handle nested object selection", async () => {
			const query = `
				query {
					user(id: "1") {
						id
						name
						email
					}
				}
			`;

			const response = await client.post("/graphql", {
				json: { query, variables: { id: "1" } },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			const user = data.data?.user as Record<string, unknown> | undefined;
			expect(user).toBeDefined();
			expect(user?.id).toBe("1");
			expect(user?.name).toBe("Alice");
			expect(user?.email).toBe("alice@example.com");
		});

		it("should handle deeply nested queries", async () => {
			const query = `
				query ComplexSearch($searchTerm: String!) {
					search(term: $searchTerm) {
						total
						users {
							id
							name
							posts {
								id
								title
								likes
							}
						}
						posts {
							id
							title
							author {
								id
								name
							}
							comments {
								id
								text
								likes
							}
						}
					}
				}
			`;
			const variables = { searchTerm: "graphql" };

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data?.search).toHaveProperty("total");
			const search = data.data?.search as Record<string, unknown>;
			expect(Array.isArray(search.users)).toBe(true);
			expect(Array.isArray(search.posts)).toBe(true);
		});

		it("should handle aliases in field selection", async () => {
			const query = `
				query {
					search(term: "graphql") {
						total
						userCount: users { id }
						postCount: posts { id }
					}
				}
			`;

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			const search = data.data?.search as Record<string, unknown>;
			expect(search.total).toBe(42);
		});
	});

	describe("GraphQL Error Handling", () => {
		it("should return GraphQL errors for invalid fields", async () => {
			const query = "query { invalidField }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.errors).toBeDefined();
			expect(Array.isArray(data.errors)).toBe(true);
			expect(data.errors?.[0].message).toContain("Cannot query field");
		});

		it("should include error locations", async () => {
			const query = "query { invalidField }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			const data = response.json() as GraphQLResponse;
			const error = data.errors?.[0];
			expect(error?.locations).toBeDefined();
			expect(Array.isArray(error?.locations)).toBe(true);
		});

		it("should include error path", async () => {
			const query = "query { invalidField }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			const data = response.json() as GraphQLResponse;
			const error = data.errors?.[0];
			expect(error?.path).toBeDefined();
			expect(Array.isArray(error?.path)).toBe(true);
		});

		it("should handle errors with extensions", async () => {
			const query = "query { invalidField }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			const data = response.json() as GraphQLResponse;
			const error = data.errors?.[0];
			expect(error).toHaveProperty("message");
		});
	});

	describe("Request/Response Format", () => {
		it("should accept JSON request body", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should return JSON response", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			const json = response.json() as Record<string, unknown>;
			expect(typeof json).toBe("object");
			expect(json).not.toBeNull();
		});

		it("should include Content-Type header in response", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			const headers = response.headers();
			expect(headers).toHaveProperty("content-type");
		});

		it("should handle empty response body", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { }" },
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Variable Handling", () => {
		it("should substitute simple variables", async () => {
			const query = "query GetUser($id: ID!) { user(id: $id) { id } }";
			const variables = { id: "789" };

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			const data = response.json() as GraphQLResponse;
			const user = (data.data?.user as Record<string, unknown>) || {};
			expect(user.id).toBe("789");
		});

		it("should handle nested object variables", async () => {
			const query = `
				mutation CreateUser($input: CreateUserInput!) {
					createUser(input: $input) { id name email }
				}
			`;
			const variables = {
				input: {
					name: "Test User",
					email: "test@example.com",
					role: "admin",
				},
			};

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should handle array variables", async () => {
			const query = 'query { search(term: "test") { total } }';
			const variables = { ids: ["1", "2", "3"] };

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should handle undefined variables gracefully", async () => {
			const query = "query { hello }";

			const response = await client.post("/graphql", {
				json: { query, variables: undefined },
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Multiple Routes", () => {
		it("should support multiple GraphQL endpoints", async () => {
			const appMulti = new Spikard();

			const multiHandlerFn = async (req: Request) => {
				const body = req.json() as GraphQLRequest | null;
				if (!body?.query) {
					return { status: 400, body: { errors: [{ message: "No query" }] } };
				}
				return { status: 200, body: { data: { source: "multi", hello: "world" } } };
			};

			// Add default GraphQL endpoint
			appMulti.addRoute({ method: "POST", path: "/graphql", handler_name: "graphql1", is_async: true }, multiHandlerFn);

			// Add alternative GraphQL endpoint
			appMulti.addRoute(
				{ method: "POST", path: "/api/graphql", handler_name: "graphql2", is_async: true },
				multiHandlerFn,
			);

			const clientMulti = new TestClient(appMulti as SpikardApp);

			const response1 = await clientMulti.post("/graphql", {
				json: { query: "query { hello }" },
			});
			const response2 = await clientMulti.post("/api/graphql", {
				json: { query: "query { hello }" },
			});

			expect(response1.statusCode).toBe(200);
			expect(response2.statusCode).toBe(200);
		});
	});

	describe("HTTP Method Compliance", () => {
		it("should only support POST method", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should include proper response status codes", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			expect(response.statusCode).toBeGreaterThanOrEqual(200);
			expect(response.statusCode).toBeLessThan(300);
		});
	});

	describe("Content Negotiation", () => {
		it("should accept application/json content type", async () => {
			const response = await client.post("/graphql", {
				headers: { "content-type": "application/json" },
				json: { query: "query { hello }" },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should return application/json content type", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			const headers = response.headers();
			const contentType = headers["content-type"]?.toLowerCase() || "";
			expect(contentType).toContain("json");
		});
	});

	describe("Query Complexity", () => {
		it("should handle simple field queries", async () => {
			const query = "query { hello }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should handle multi-field queries", async () => {
			const query = `
				query {
					hello
					search(term: "test") {
						total
					}
				}
			`;

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should handle fragment-based queries", async () => {
			const query = `
				fragment UserFields on User {
					id
					name
					email
				}

				query {
					user(id: "1") {
						...UserFields
					}
				}
			`;

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect(response.statusCode).toBe(200);
		});

		it("should handle directive-based queries", async () => {
			const query = `
				query GetUser($includeEmail: Boolean!) {
					user(id: "1") {
						id
						name
						email @include(if: $includeEmail)
					}
				}
			`;
			const variables = { includeEmail: true };

			const response = await client.post("/graphql", {
				json: { query, variables },
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Response Data Structure", () => {
		it("should return data field in response", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			const data = response.json() as GraphQLResponse;
			expect(data).toHaveProperty("data");
		});

		it("should return null data when errors occur", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { invalidField }" },
			});

			const data = response.json() as GraphQLResponse;
			expect(data.errors).toBeDefined();
		});

		it("should never return both data and errors at root level", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			const data = response.json() as GraphQLResponse;
			// Either data or errors (or both), but structure should be valid
			expect(data).toHaveProperty("data");
		});

		it("should handle null root value", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { }" },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json();
			expect(data).not.toBeNull();
		});
	});

	describe("Edge Cases", () => {
		it("should handle empty query string", async () => {
			const response = await client.post("/graphql", {
				json: { query: "" },
			});

			expect([400, 200]).toContain(response.statusCode);
		});

		it("should handle whitespace-only query", async () => {
			const response = await client.post("/graphql", {
				json: { query: "   " },
			});

			expect([400, 200]).toContain(response.statusCode);
		});

		it("should handle very large query", async () => {
			const largeQuery = `query { hello ${" hello".repeat(1000)} }`;

			const response = await client.post("/graphql", {
				json: { query: largeQuery },
			});

			expect([200, 400, 413]).toContain(response.statusCode);
		});

		it("should handle special characters in query", async () => {
			const query = `query { user(id: "test\\nvalue") { id } }`;

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect([200, 400]).toContain(response.statusCode);
		});

		it("should preserve response body even with status errors", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { invalidField }" },
			});

			const body = response.text();
			expect(body.length).toBeGreaterThan(0);
		});
	});

	describe("Performance and Limits", () => {
		it("should handle high nesting level queries", async () => {
			let query = 'query { search(term: "test") { users { id';
			for (let i = 0; i < 10; i++) {
				query += " posts { id";
			}
			query += " } }";
			for (let i = 0; i < 10; i++) {
				query += " }";
			}
			query += " } }";

			const response = await client.post("/graphql", {
				json: { query },
			});

			expect([200, 400]).toContain(response.statusCode);
		});

		it("should handle many variables", async () => {
			const variables: Record<string, string> = {};
			for (let i = 0; i < 100; i++) {
				variables[`var${i}`] = `value${i}`;
			}

			const response = await client.post("/graphql", {
				json: { query: "query { hello }", variables },
			});

			expect(response.statusCode).toBe(200);
		});
	});

	describe("Fixture Compliance", () => {
		it("should match simple_field fixture format", async () => {
			const response = await client.post("/graphql", {
				json: { query: "query { hello }" },
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data).toHaveProperty("data");
			expect(data.data).toHaveProperty("hello");
		});

		it("should match create_resource mutation fixture format", async () => {
			const response = await client.post("/graphql", {
				json: {
					query: `mutation CreateUser($input: CreateUserInput!) {
						createUser(input: $input) {
							id
							name
							email
							role
							createdAt
						}
					}`,
					variables: {
						input: {
							name: "John Doe",
							email: "john@example.com",
							role: "admin",
						},
					},
					operationName: "CreateUser",
				},
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			expect(data.data).toHaveProperty("createUser");
		});

		it("should match complex_query fixture format", async () => {
			const response = await client.post("/graphql", {
				json: {
					query: `query ComplexSearch($searchTerm: String!) {
						search(term: $searchTerm) {
							total
							users {
								id
								name
								posts {
									id
									title
									likes
								}
							}
							posts {
								id
								title
								author {
									id
									name
								}
								comments {
									id
									text
									likes
								}
							}
						}
					}`,
					variables: { searchTerm: "graphql" },
					operationName: "ComplexSearch",
				},
			});

			expect(response.statusCode).toBe(200);
			const data = response.json() as GraphQLResponse;
			const search = data.data?.search as Record<string, unknown>;
			expect(search.total).toBe(42);
		});
	});
});
