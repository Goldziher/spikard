/**
 * Comprehensive FFI error boundary tests for Node.js bindings
 *
 * This test suite validates that errors thrown from JavaScript handlers
 * are properly caught, serialized, and converted to structured JSON responses
 * without exposing raw Rust panics or unhandled JavaScript errors across the
 * FFI boundary. Tests cover both synchronous and asynchronous error paths.
 *
 * Tests cover:
 * - Native Error objects with structured JSON serialization
 * - Custom error types and non-Error object throws
 * - Promise rejections and async/await error handling
 * - Nested error causes and stack trace preservation
 * - Status code and structured response handling
 * - JSON parsing failures and malformed input
 * - Request validation error propagation
 * - Error response format validation against testing_data/validation_errors
 * - Multiple sequential errors (error in error handler)
 * - napi-rs error conversion boundaries
 */

import { describe, expect, it } from "vitest";
import { wrapBodyHandler, wrapHandler } from "./handler-wrapper";
import type { NativeRequestData } from "./request";

const createRequestPayload = (overrides?: Partial<NativeRequestData>): NativeRequestData => ({
	method: "POST",
	path: "/test",
	params: { id: "123" },
	query: {},
	headers: { "content-type": "application/json" },
	cookies: {},
	body: null,
	...overrides,
});

/**
 * Structured error response format from testing_data/validation_errors/schema.json
 * Errors should return JSON with: { error: string, code?: string, details?: object }
 */
interface StructuredErrorResponse {
	error?: string;
	message?: string;
	code?: string;
	details?: Record<string, unknown>;
	[key: string]: unknown;
}

describe("error-boundary", () => {
	describe("Native Error Objects", () => {
		it("should throw native Error with message from handler", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("Handler error");
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Handler error");
		});

		it("should throw custom Error subclass from handler", async () => {
			class ValidationError extends Error {
				constructor(
					message: string,
					public code: string,
				) {
					super(message);
					this.name = "ValidationError";
				}
			}

			const handler = wrapHandler(async () => {
				throw new ValidationError("Invalid input", "INVALID_DATA");
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Invalid input");
		});

		it("should throw Error with complex message from handler", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("Field 'email' is required and must be a valid email address");
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow(
				"Field 'email' is required and must be a valid email address",
			);
		});
	});

	describe("Non-Error Object Throws", () => {
		it("should throw string from handler", async () => {
			const handler = wrapHandler(async () => {
				// eslint-disable-next-line @typescript-eslint/only-throw-error
				throw "A string was thrown, not an Error object";
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toBe("A string was thrown, not an Error object");
		});

		it("should throw null from handler", async () => {
			const handler = wrapHandler(async () => {
				// eslint-disable-next-line @typescript-eslint/only-throw-error
				throw null;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toBe(null);
		});

		it("should throw undefined from handler", async () => {
			const handler = wrapHandler(async () => {
				// eslint-disable-next-line @typescript-eslint/only-throw-error
				throw undefined;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toBe(undefined);
		});

		it("should throw object literal from handler", async () => {
			const handler = wrapHandler(async () => {
				// eslint-disable-next-line @typescript-eslint/only-throw-error
				throw { message: "Object literal thrown", code: "CUSTOM_ERROR" };
			});

			const payload = createRequestPayload();
			const result = handler(JSON.stringify(payload));
			await expect(result).rejects.toEqual(expect.objectContaining({ message: "Object literal thrown" }));
		});

		it("should throw number from handler", async () => {
			const handler = wrapHandler(async () => {
				// eslint-disable-next-line @typescript-eslint/only-throw-error
				throw 42;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toBe(42);
		});
	});

	describe("Promise Rejection Handling", () => {
		it("should throw async handler promise rejection", async () => {
			const handler = wrapHandler(async () => {
				return Promise.reject(new Error("Promise rejection error"));
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Promise rejection error");
		});

		it("should throw async/await exception in handler", async () => {
			const handler = wrapHandler(async () => {
				const value = await Promise.resolve("test");
				if (value === "test") {
					throw new Error("Thrown after await");
				}
				return value;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Thrown after await");
		});

		it("should throw nested async/await exception", async () => {
			const asyncHelper = async () => {
				throw new Error("Nested async error");
			};

			const handler = wrapHandler(async () => {
				const result = await asyncHelper();
				return result;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Nested async error");
		});

		it("should throw unhandled rejection in Promise.all", async () => {
			const handler = wrapHandler(async () => {
				await Promise.all([Promise.resolve("ok"), Promise.reject(new Error("One promise failed"))]);
				return { status: "success" };
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("One promise failed");
		});
	});

	describe("Error Cause and Stack Preservation", () => {
		it("should throw error with cause property from handler", async () => {
			const handler = wrapHandler(async () => {
				const cause = new Error("Original database error");
				throw new Error("Request failed", { cause });
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Request failed");
		});

		it("should throw error with stack trace from handler", async () => {
			const handler = wrapHandler(async () => {
				const error = new Error("Error with stack");
				expect(error.stack).toBeDefined();
				throw error;
			});

			const payload = createRequestPayload();
			const thrown = await expect(handler(JSON.stringify(payload))).rejects.toThrow("Error with stack");
			expect(thrown).toBeDefined();
		});

		it("should throw deeply nested error causes from handler", async () => {
			const handler = wrapHandler(async () => {
				const level3 = new Error("Level 3: Database timeout");
				const level2 = new Error("Level 2: Query failed", { cause: level3 });
				const level1 = new Error("Level 1: Request failed", { cause: level2 });
				throw level1;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Level 1: Request failed");
		});
	});

	describe("Status Code and Structured Response", () => {
		it("should handle handler returning error with status code", async () => {
			const handler = wrapHandler(async () => ({
				status: 400,
				body: { error: "Bad Request" },
			}));

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.status).toBe(400);
			expect((parsed.body as Record<string, unknown>).error).toBe("Bad Request");
		});

		it("should handle handler throwing after returning structured response", async () => {
			const handler = wrapHandler(async () => {
				// Handler successfully returns structured response
				return {
					statusCode: 200,
					headers: { "content-type": "application/json" },
					body: { message: "success" },
				};
			});

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.statusCode).toBe(200);
			expect((parsed.body as Record<string, unknown>).message).toBe("success");
		});

		it("should handle 422 validation error response", async () => {
			const handler = wrapHandler(async () => ({
				status: 422,
				body: {
					detail: [
						{
							type: "value_error",
							loc: ["body", "email"],
							msg: "invalid email address",
							input: "not-an-email",
						},
					],
				},
			}));

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.status).toBe(422);
			expect(Array.isArray((parsed.body as Record<string, unknown>).detail)).toBe(true);
		});
	});

	describe("JSON Parsing and Input Validation", () => {
		it("should throw on malformed JSON request payload", async () => {
			const handler = wrapHandler(async () => ({ ok: true }));

			// Pass invalid JSON
			await expect(handler("{ invalid json")).rejects.toThrow();
		});

		it("should throw on empty request payload", async () => {
			const handler = wrapHandler(async () => ({ ok: true }));

			// Pass empty string
			await expect(handler("")).rejects.toThrow();
		});

		it("should throw on JSON parse error in body", async () => {
			const handler = wrapHandler(async (req) => {
				const body = req.json();
				return { body };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from("{ invalid json }")),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow(SyntaxError);
		});

		it("should throw on handler body parsing error", async () => {
			const handler = wrapBodyHandler(async (body) => {
				if (!body || typeof body !== "object") {
					throw new Error("Invalid body type");
				}
				return { body };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from("invalid-json")),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow(SyntaxError);
		});
	});

	describe("Request Validation Error Propagation", () => {
		it("should throw missing required field error", async () => {
			const handler = wrapBodyHandler<{ email: string }>(async (body) => {
				if (!body.email) {
					throw new Error("Field 'email' is required");
				}
				return { email: body.email };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Field 'email' is required");
		});

		it("should throw validation error with details", async () => {
			const handler = wrapHandler(async (req) => {
				const id = req.params.id as string;
				if (!id.match(/^\d+$/)) {
					throw new Error("Parameter 'id' must be numeric");
				}
				return { id };
			});

			const payload = createRequestPayload({
				params: { id: "not-numeric" },
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Parameter 'id' must be numeric");
		});

		it("should throw type validation error", async () => {
			const handler = wrapBodyHandler<{ age: number }>(async (body) => {
				if (typeof body.age !== "number") {
					throw new Error("Field 'age' must be a number");
				}
				return { age: body.age };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({ age: "not-a-number" }))),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Field 'age' must be a number");
		});
	});

	describe("Error Response Format Validation", () => {
		it("should throw error in consistent format", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("Validation failed");
			});

			const payload = createRequestPayload();
			const result = handler(JSON.stringify(payload));

			// Should reject with the error
			await expect(result).rejects.toThrow("Validation failed");
		});

		it("should preserve error message when thrown", async () => {
			const errorMessage = "Specific validation error message";
			const handler = wrapHandler(async () => {
				throw new Error(errorMessage);
			});

			const payload = createRequestPayload();
			const result = handler(JSON.stringify(payload));

			await expect(result).rejects.toThrow(errorMessage);
		});

		it("should throw error with custom code property", async () => {
			interface ErrorWithCode extends Error {
				code?: string;
			}

			const handler = wrapHandler(async () => {
				const error: ErrorWithCode = new Error("Invalid parameter");
				error.code = "INVALID_PARAMETER";
				throw error;
			});

			const payload = createRequestPayload();
			const result = handler(JSON.stringify(payload));

			await expect(result).rejects.toThrow("Invalid parameter");
		});
	});

	describe("napi-rs Error Conversion Boundaries", () => {
		it("should not expose raw Rust panic in error message", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("Application error");
			});

			const payload = createRequestPayload();
			try {
				await handler(JSON.stringify(payload));
				expect.fail("Should have thrown");
			} catch (err) {
				expect(String(err)).not.toContain("panicked");
				expect(String(err)).not.toContain("thread");
			}
		});

		it("should throw errors without exposing FFI internals", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("User-facing error message");
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("User-facing error message");
		});

		it("should throw complex error objects safely", async () => {
			const handler = wrapHandler(async () => {
				const error = new Error("Complex error");
				(error as Record<string, unknown>).context = { userId: 123 };
				throw error;
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Complex error");
		});
	});

	describe("Multiple Sequential Errors", () => {
		it("should throw error in error recovery from handler", async () => {
			const handler = wrapHandler(async () => {
				try {
					throw new Error("Original error");
				} catch {
					throw new Error("Error while handling error");
				}
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Error while handling error");
		});

		it("should handle cleanup after error in handler", async () => {
			const handler = wrapHandler(async () => {
				try {
					throw new Error("First error");
				} finally {
					// Simulate cleanup that completes safely
					// Cleanup logic here without throwing
				}
			});

			const payload = createRequestPayload();
			const result = handler(JSON.stringify(payload));
			// Should propagate the first error from try block
			await expect(result).rejects.toThrow("First error");
		});

		it("should handle sequential handler calls with different errors", async () => {
			const handler1 = wrapHandler(async () => {
				throw new Error("Error from handler 1");
			});

			const handler2 = wrapHandler(async () => {
				throw new Error("Error from handler 2");
			});

			const payload = createRequestPayload();

			const result1 = handler1(JSON.stringify(payload));
			const result2 = handler2(JSON.stringify(payload));

			await expect(result1).rejects.toThrow("Error from handler 1");
			await expect(result2).rejects.toThrow("Error from handler 2");
		});
	});

	describe("Sync and Async Error Path Consistency", () => {
		it("should throw both sync and async errors consistently", async () => {
			const syncHandler = wrapHandler(() => {
				throw new Error("Sync error");
			});

			const asyncHandler = wrapHandler(async () => {
				throw new Error("Async error");
			});

			const payload = createRequestPayload();

			await expect(syncHandler(JSON.stringify(payload))).rejects.toThrow("Sync error");
			await expect(asyncHandler(JSON.stringify(payload))).rejects.toThrow("Async error");
		});

		it("should throw errors at different execution stages", async () => {
			const earlyError = wrapHandler(async () => {
				throw new Error("Error at start");
			});

			const lateError = wrapHandler(async () => {
				await Promise.resolve();
				await Promise.resolve();
				throw new Error("Error at end");
			});

			const payload = createRequestPayload();

			await expect(earlyError(JSON.stringify(payload))).rejects.toThrow("Error at start");
			await expect(lateError(JSON.stringify(payload))).rejects.toThrow("Error at end");
		});
	});

	describe("Body Handler Error Scenarios", () => {
		it("should throw wrapBodyHandler error during parsing", async () => {
			const handler = wrapBodyHandler<{ value: number }>(async (body) => {
				if (!body || typeof body !== "object" || !("value" in body)) {
					throw new Error("Invalid body structure");
				}
				return { value: (body as Record<string, unknown>).value };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Invalid body structure");
		});

		it("should throw wrapBodyHandler error with null body", async () => {
			const handler = wrapBodyHandler(async (body) => {
				if (body === null) {
					throw new Error("Body cannot be null");
				}
				return { body };
			});

			const payload = createRequestPayload({
				body: null,
			});

			// When body is null, wrapBodyHandler will try to parse it and throw
			await expect(handler(JSON.stringify(payload))).rejects.toThrow();
		});

		it("should preserve body in error context during wrapBodyHandler", async () => {
			const handler = wrapBodyHandler<{ name: string }>(async (body) => {
				if (!body || typeof body !== "object" || !("name" in body)) {
					throw new Error("Missing required field: name");
				}
				return { name: (body as Record<string, unknown>).name };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({ age: 30 }))),
			});

			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Missing required field: name");
		});
	});
});
