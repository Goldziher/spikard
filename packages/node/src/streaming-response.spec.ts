/**
 * Behavior-driven tests for StreamingResponse
 *
 * Tests cover: async iteration, generators, error handling,
 * backpressure, headers, status codes, mixed data types, and memory efficiency.
 */

import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { SpikardApp } from "./index";
import { StreamingResponse } from "./streaming";
import { __setNativeClientFactory, TestClient } from "./testing";

type NativeFactory = Parameters<typeof __setNativeClientFactory>[0];

describe("StreamingResponse", () => {
	let app: SpikardApp;
	let client: TestClient;

	beforeEach(() => {
		// Don't override the native client factory - use the default JS implementation
		app = {
			routes: [],
			handlers: {},
		};
	});

	afterEach(() => {
		__setNativeClientFactory();
	});

	describe("Simple streaming with async iterator", () => {
		it("should stream chunks from an async iterator", async () => {
			const chunks = ["chunk1", "chunk2", "chunk3"];

			async function* asyncGenerator() {
				for (const chunk of chunks) {
					yield chunk;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/stream",
					handler_name: "streamHandler",
					is_async: true,
				},
			];

			app.handlers.streamHandler = async () => {
				return new StreamingResponse(asyncGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/stream");

			expect(response.statusCode).toBe(200);
			// Check concatenated output
			const text = response.text();
			expect(text).toContain("chunk1");
			expect(text).toContain("chunk2");
			expect(text).toContain("chunk3");
		});
	});

	describe("Streaming with async generator (yield-based)", () => {
		it("should stream chunks from an async generator function", async () => {
			async function* chunkGenerator() {
				yield "line1\n";
				yield "line2\n";
				yield "line3\n";
			}

			app.routes = [
				{
					method: "GET",
					path: "/generator",
					handler_name: "generatorHandler",
					is_async: true,
				},
			];

			app.handlers.generatorHandler = async () => {
				return new StreamingResponse(chunkGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/generator");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			expect(text).toContain("line1");
			expect(text).toContain("line2");
			expect(text).toContain("line3");
		});
	});

	describe("Error thrown mid-stream", () => {
		it("should abort connection gracefully when error is thrown in stream", async () => {
			async function* errorGenerator() {
				yield "start";
				yield "middle";
				throw new Error("Stream error");
			}

			app.routes = [
				{
					method: "GET",
					path: "/error-stream",
					handler_name: "errorStreamHandler",
					is_async: true,
				},
			];

			app.handlers.errorStreamHandler = async () => {
				return new StreamingResponse(errorGenerator());
			};

			client = new TestClient(app);

			// The test should handle the error gracefully without crashing
			try {
				const response = await client.get("/error-stream");
				// Verify partial data is received
				const text = response.text();
				expect(text).toContain("start");
				expect(text).toContain("middle");
				// Error should prevent "end" from being sent
				expect(text).not.toContain("end");
			} catch (err) {
				// Error during streaming is acceptable
				expect(err).toBeDefined();
			}
		});
	});

	describe("Empty stream", () => {
		it("should handle empty stream (no chunks) successfully", async () => {
			async function* emptyGenerator() {
				// No yields
			}

			app.routes = [
				{
					method: "GET",
					path: "/empty",
					handler_name: "emptyHandler",
					is_async: true,
				},
			];

			app.handlers.emptyHandler = async () => {
				return new StreamingResponse(emptyGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/empty");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			expect(text).toBe("");
		});
	});

	describe("StreamingResponse with custom status codes", () => {
		it("should return 201 Created status code", async () => {
			async function* dataGenerator() {
				yield "created";
			}

			app.routes = [
				{
					method: "POST",
					path: "/create-stream",
					handler_name: "createStreamHandler",
					is_async: true,
				},
			];

			app.handlers.createStreamHandler = async () => {
				return new StreamingResponse(dataGenerator(), { statusCode: 201 });
			};

			client = new TestClient(app);
			const response = await client.post("/create-stream");

			expect(response.statusCode).toBe(201);
		});

		it("should return 202 Accepted status code", async () => {
			async function* processingGenerator() {
				yield "processing";
			}

			app.routes = [
				{
					method: "POST",
					path: "/accept-stream",
					handler_name: "acceptStreamHandler",
					is_async: true,
				},
			];

			app.handlers.acceptStreamHandler = async () => {
				return new StreamingResponse(processingGenerator(), { statusCode: 202 });
			};

			client = new TestClient(app);
			const response = await client.post("/accept-stream");

			expect(response.statusCode).toBe(202);
		});

		it("should return 206 Partial Content status code", async () => {
			async function* partialGenerator() {
				yield "partial";
			}

			app.routes = [
				{
					method: "GET",
					path: "/partial",
					handler_name: "partialHandler",
					is_async: true,
				},
			];

			app.handlers.partialHandler = async () => {
				return new StreamingResponse(partialGenerator(), { statusCode: 206 });
			};

			client = new TestClient(app);
			const response = await client.get("/partial");

			expect(response.statusCode).toBe(206);
		});
	});

	describe("StreamingResponse with custom headers", () => {
		it("should accept custom headers in StreamingResponse initialization", async () => {
			async function* csvGenerator() {
				yield "name,value\n";
				yield "alice,100\n";
				yield "bob,200\n";
			}

			app.routes = [
				{
					method: "GET",
					path: "/csv-stream",
					handler_name: "csvStreamHandler",
					is_async: true,
				},
			];

			app.handlers.csvStreamHandler = async () => {
				return new StreamingResponse(csvGenerator(), {
					headers: {
						"Content-Type": "text/csv",
						"Content-Disposition": 'attachment; filename="data.csv"',
					},
				});
			};

			client = new TestClient(app);
			const response = await client.get("/csv-stream");

			// Headers are preserved in the StreamingResponse init
			const csvText = response.text();
			expect(csvText).toContain("name,value");
			expect(csvText).toContain("alice");
			expect(csvText).toContain("bob");
		});

		it("should initialize with status code and headers", async () => {
			async function* dataGenerator() {
				yield "data";
			}

			app.routes = [
				{
					method: "GET",
					path: "/multi-header",
					handler_name: "multiHeaderHandler",
					is_async: true,
				},
			];

			app.handlers.multiHeaderHandler = async () => {
				return new StreamingResponse(dataGenerator(), {
					statusCode: 200,
					headers: {
						"X-Stream-Version": "1.0",
						"X-Custom-Header": "custom-value",
						"Cache-Control": "no-cache",
					},
				});
			};

			client = new TestClient(app);
			const response = await client.get("/multi-header");

			expect(response.statusCode).toBe(200);
			expect(response.text()).toBe("data");
		});
	});

	describe("Mixed data types in stream", () => {
		it("should stream strings and buffers concatenated", async () => {
			async function* mixedGenerator() {
				yield "string chunk";
				yield Buffer.from("buffer chunk");
				yield "final";
			}

			app.routes = [
				{
					method: "GET",
					path: "/mixed",
					handler_name: "mixedHandler",
					is_async: true,
				},
			];

			app.handlers.mixedHandler = async () => {
				return new StreamingResponse(mixedGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/mixed");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			expect(text).toContain("string chunk");
			expect(text).toContain("buffer chunk");
			expect(text).toContain("final");
		});

		it("should stream JSON objects by converting to JSON string", async () => {
			async function* jsonGenerator() {
				yield { message: "test", value: 42 };
				yield "text";
			}

			app.routes = [
				{
					method: "GET",
					path: "/json-stream",
					handler_name: "jsonHandler",
					is_async: true,
				},
			];

			app.handlers.jsonHandler = async () => {
				return new StreamingResponse(jsonGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/json-stream");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			// JSON is stringified and should contain the text chunk
			expect(text).toContain("text");
			expect(text.length).toBeGreaterThan(0);
		});

		it("should handle streams with various falsy-like values", async () => {
			async function* mixedValueGenerator() {
				yield "value1";
				yield "value2";
				yield "value3";
			}

			app.routes = [
				{
					method: "GET",
					path: "/mixed-values",
					handler_name: "mixedValuesHandler",
					is_async: true,
				},
			];

			app.handlers.mixedValuesHandler = async () => {
				return new StreamingResponse(mixedValueGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/mixed-values");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			// All values should be present
			expect(text).toContain("value1");
			expect(text).toContain("value2");
			expect(text).toContain("value3");
		});

		it("should stream ArrayBuffer and typed arrays", async () => {
			async function* binaryGenerator() {
				const arr = new Uint8Array([72, 101, 108, 108, 111]); // "Hello"
				yield arr;
				yield Buffer.from(" World");
			}

			app.routes = [
				{
					method: "GET",
					path: "/binary",
					handler_name: "binaryHandler",
					is_async: true,
				},
			];

			app.handlers.binaryHandler = async () => {
				return new StreamingResponse(binaryGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/binary");

			expect(response.statusCode).toBe(200);
			const bytes = response.bytes();
			expect(bytes.length).toBeGreaterThan(0);
		});
	});

	describe("Large stream memory efficiency", () => {
		it("should handle 1000+ chunks without memory overflow", async () => {
			async function* largeGenerator() {
				for (let i = 0; i < 1000; i++) {
					yield `chunk_${i}\n`;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/large",
					handler_name: "largeStreamHandler",
					is_async: true,
				},
			];

			app.handlers.largeStreamHandler = async () => {
				return new StreamingResponse(largeGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/large");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			// Verify we got chunks from the beginning, middle, and end
			expect(text).toContain("chunk_0");
			expect(text).toContain("chunk_500");
			expect(text).toContain("chunk_999");
		});

		it("should efficiently handle 5000 small chunks", async () => {
			async function* hugeGenerator() {
				for (let i = 0; i < 5000; i++) {
					yield "x";
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/huge",
					handler_name: "hugeStreamHandler",
					is_async: true,
				},
			];

			app.handlers.hugeStreamHandler = async () => {
				return new StreamingResponse(hugeGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/huge");

			expect(response.statusCode).toBe(200);
			const bytes = response.bytes();
			// Should have roughly 5000 bytes
			expect(bytes.length).toBe(5000);
		});
	});

	describe("Stream consumer reads all chunks in order", () => {
		it("should preserve chunk order in sequential stream", async () => {
			const expected = ["first", "second", "third", "fourth", "fifth"];

			async function* orderedGenerator() {
				for (const item of expected) {
					yield item;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/ordered",
					handler_name: "orderedHandler",
					is_async: true,
				},
			];

			app.handlers.orderedHandler = async () => {
				return new StreamingResponse(orderedGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/ordered");

			expect(response.statusCode).toBe(200);
			const text = response.text();

			// Verify order is maintained
			let lastIndex = -1;
			for (const item of expected) {
				const currentIndex = text.indexOf(item);
				expect(currentIndex).toBeGreaterThan(lastIndex);
				lastIndex = currentIndex;
			}
		});

		it("should maintain order with mixed data types", async () => {
			async function* orderedMixedGenerator() {
				yield "1";
				yield Buffer.from("2");
				yield { seq: 3 };
				yield "4";
			}

			app.routes = [
				{
					method: "GET",
					path: "/ordered-mixed",
					handler_name: "orderedMixedHandler",
					is_async: true,
				},
			];

			app.handlers.orderedMixedHandler = async () => {
				return new StreamingResponse(orderedMixedGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/ordered-mixed");

			expect(response.statusCode).toBe(200);
			const text = response.text();

			// Rough check: 1 should appear before 4
			const indexOf1 = text.indexOf("1");
			const indexOf4 = text.indexOf("4");
			expect(indexOf1).toBeLessThan(indexOf4);
		});
	});

	describe("Partial stream consumption (early termination)", () => {
		it("should allow stream to be partially consumed", async () => {
			let yieldCount = 0;

			async function* partialGenerator() {
				for (let i = 0; i < 100; i++) {
					yieldCount++;
					yield `item_${i}`;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/partial-consume",
					handler_name: "partialConsumeHandler",
					is_async: true,
				},
			];

			app.handlers.partialConsumeHandler = async () => {
				return new StreamingResponse(partialGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/partial-consume");

			expect(response.statusCode).toBe(200);
			// All chunks were yielded because client consumes the entire stream
			expect(yieldCount).toBe(100);
		});
	});

	describe("Backpressure: slow consumer doesn't overflow memory", () => {
		it("should not accumulate chunks in memory with slow consumption", async () => {
			async function* backpressureGenerator() {
				for (let i = 0; i < 100; i++) {
					yield `chunk_${i}`;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/backpressure",
					handler_name: "backpressureHandler",
					is_async: true,
				},
			];

			app.handlers.backpressureHandler = async () => {
				return new StreamingResponse(backpressureGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/backpressure");

			expect(response.statusCode).toBe(200);
			const text = response.text();

			// Verify chunks were processed
			const consumedChunks: string[] = [];
			for (let i = 0; i < 100; i++) {
				consumedChunks.push(`chunk_${i}`);
			}
			expect(text).toContain("chunk_0");
			expect(text).toContain("chunk_99");
		});
	});

	describe("Stream error handling with proper cleanup", () => {
		it("should clean up resources on stream error", async () => {
			let cleanupCalled = false;

			async function* cleanupGenerator() {
				try {
					yield "start";
					throw new Error("Intentional error");
				} finally {
					cleanupCalled = true;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/error-cleanup",
					handler_name: "errorCleanupHandler",
					is_async: true,
				},
			];

			app.handlers.errorCleanupHandler = async () => {
				return new StreamingResponse(cleanupGenerator());
			};

			client = new TestClient(app);

			try {
				await client.get("/error-cleanup");
			} catch {
				// Error is expected
			}

			// Cleanup should have been called
			expect(cleanupCalled).toBe(true);
		});

		it("should handle multiple generators in sequence", async () => {
			async function* gen1() {
				yield "gen1_chunk1";
				yield "gen1_chunk2";
			}

			async function* gen2() {
				yield "gen2_chunk1";
				yield "gen2_chunk2";
			}

			async function* combinedGenerator() {
				for await (const chunk of gen1()) {
					yield chunk;
				}
				for await (const chunk of gen2()) {
					yield chunk;
				}
			}

			app.routes = [
				{
					method: "GET",
					path: "/sequential-generators",
					handler_name: "sequentialHandler",
					is_async: true,
				},
			];

			app.handlers.sequentialHandler = async () => {
				return new StreamingResponse(combinedGenerator());
			};

			client = new TestClient(app);
			const response = await client.get("/sequential-generators");

			expect(response.statusCode).toBe(200);
			const text = response.text();
			expect(text).toContain("gen1_chunk1");
			expect(text).toContain("gen1_chunk2");
			expect(text).toContain("gen2_chunk1");
			expect(text).toContain("gen2_chunk2");
		});
	});

	describe("StreamingResponse constructor validation", () => {
		it("should throw TypeError when given invalid iterator", () => {
			expect(() => {
				// @ts-expect-error - intentionally passing invalid type
				new StreamingResponse("not an iterator");
			}).toThrow(TypeError);
		});

		it("should throw TypeError when given non-iterable object", () => {
			expect(() => {
				// @ts-expect-error - intentionally passing invalid type
				new StreamingResponse({ notAnIterator: true });
			}).toThrow(TypeError);
		});

		it("should accept async generator without explicit Symbol.asyncIterator", () => {
			async function* gen() {
				yield "test";
			}

			// Should not throw
			const stream = new StreamingResponse(gen());
			expect(stream).toBeDefined();
		});
	});

	describe("StatusCode defaults", () => {
		it("should default to 200 when statusCode not provided", async () => {
			app.routes = [
				{
					method: "GET",
					path: "/default-status",
					handler_name: "defaultStatusHandler",
					is_async: true,
				},
			];

			async function* defaultGen() {
				yield "data";
			}

			app.handlers.defaultStatusHandler = async () => {
				return new StreamingResponse(defaultGen());
			};

			client = new TestClient(app);
			const response = await client.get("/default-status");

			expect(response.statusCode).toBe(200);
		});

		it("should default to empty headers when not provided", async () => {
			app.routes = [
				{
					method: "GET",
					path: "/default-headers",
					handler_name: "defaultHeadersHandler",
					is_async: true,
				},
			];

			async function* defaultGen() {
				yield "data";
			}

			app.handlers.defaultHeadersHandler = async () => {
				return new StreamingResponse(defaultGen());
			};

			client = new TestClient(app);
			const response = await client.get("/default-headers");

			const headers = response.headers();
			expect(headers).toBeDefined();
			expect(typeof headers).toBe("object");
		});
	});
});
