/**
 * Unit tests for streaming responses
 */

import { describe, expect, it } from "vitest";
import { isStreamingResponse, StreamingResponse } from "./streaming";

describe("StreamingResponse", () => {
	describe("Constructor and initialization", () => {
		it("should create streaming response from async iterator", async () => {
			async function* generator() {
				yield "hello";
				yield "world";
			}

			const response = new StreamingResponse(generator());
			expect(response).toBeInstanceOf(StreamingResponse);
			expect(response.__spikard_streaming__).toBe(true);
		});

		it("should create streaming response from async iterable", async () => {
			const iterable = {
				[Symbol.asyncIterator]: async function* () {
					yield "hello";
				},
			};

			const response = new StreamingResponse(iterable);
			expect(response).toBeInstanceOf(StreamingResponse);
		});

		it("should default to statusCode 200", () => {
			async function* generator() {
				yield "test";
			}

			const response = new StreamingResponse(generator());
			expect(response.statusCode).toBe(200);
		});

		it("should default to empty headers", () => {
			async function* generator() {
				yield "test";
			}

			const response = new StreamingResponse(generator());
			expect(response.headers).toEqual({});
		});

		it("should accept custom statusCode", () => {
			async function* generator() {
				yield "test";
			}

			const response = new StreamingResponse(generator(), { statusCode: 201 });
			expect(response.statusCode).toBe(201);
		});

		it("should accept custom headers", () => {
			async function* generator() {
				yield "test";
			}

			const headers = { "X-Custom": "value", "Content-Type": "text/plain" };
			const response = new StreamingResponse(generator(), { headers });
			expect(response.headers).toEqual(headers);
		});

		it("should accept both statusCode and headers", () => {
			async function* generator() {
				yield "test";
			}

			const response = new StreamingResponse(generator(), {
				statusCode: 202,
				headers: { "X-Test": "test" },
			});

			expect(response.statusCode).toBe(202);
			expect(response.headers).toEqual({ "X-Test": "test" });
		});

		it("should throw on invalid iterator input", () => {
			expect(() => {
				new StreamingResponse(null as never);
			}).toThrow(TypeError);
		});

		it("should throw on non-object input", () => {
			expect(() => {
				new StreamingResponse("not an iterator" as never);
			}).toThrow(TypeError);
		});

		it("should throw on plain object without iterator methods", () => {
			expect(() => {
				new StreamingResponse({ foo: "bar" } as never);
			}).toThrow(TypeError);
		});

		it("should throw on undefined input", () => {
			expect(() => {
				new StreamingResponse(undefined as never);
			}).toThrow(TypeError);
		});
	});

	describe("Collecting stream data", () => {
		it("should collect string chunks", async () => {
			async function* generator() {
				yield "hello";
				yield " ";
				yield "world";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("hello world");
		});

		it("should collect Uint8Array chunks", async () => {
			async function* generator() {
				yield new Uint8Array([72, 101, 108, 108, 111]);
				yield new Uint8Array([87, 111, 114, 108, 100]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("HelloWorld");
		});

		it("should collect JSON object chunks", async () => {
			async function* generator() {
				yield { type: "message", content: "hello" };
				yield { type: "message", content: "world" };
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			const chunks = text.split(/(?=\{)/);
			expect(chunks).toHaveLength(2);
		});

		it("should collect null and undefined chunks as empty", async () => {
			async function* generator() {
				yield "start";
				yield null;
				yield undefined;
				yield "end";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("startend");
		});

		it("should handle empty stream", async () => {
			async function* generator() {}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBe(0);
		});

		it("should collect large streams", async () => {
			async function* generator() {
				for (let i = 0; i < 1000; i++) {
					yield `chunk ${i}\n`;
				}
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toContain("chunk 0");
			expect(text).toContain("chunk 999");
		});

		it("should preserve order of collected chunks", async () => {
			async function* generator() {
				for (let i = 0; i < 10; i++) {
					yield `${i}`;
				}
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("0123456789");
		});
	});

	describe("Chunk normalization", () => {
		it("should normalize string to Uint8Array", async () => {
			async function* generator() {
				yield "hello";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected).toBeInstanceOf(Uint8Array);
		});

		it("should handle numbers as strings", async () => {
			async function* generator() {
				yield 42;
				yield 3.14;
				yield -10;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toContain("42");
			expect(text).toContain("3.14");
			expect(text).toContain("-10");
		});

		it("should handle booleans as strings", async () => {
			async function* generator() {
				yield true;
				yield false;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toContain("true");
			expect(text).toContain("false");
		});

		it("should handle ArrayBuffer", async () => {
			async function* generator() {
				const buffer = new ArrayBuffer(4);
				const view = new Uint8Array(buffer);
				view[0] = 72;
				view[1] = 105;
				yield buffer;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});

		it("should handle DataView", async () => {
			async function* generator() {
				const buffer = new ArrayBuffer(4);
				const view = new DataView(buffer);
				view.setUint8(0, 72);
				view.setUint8(1, 105);
				yield view;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected).toBeInstanceOf(Uint8Array);
		});

		it("should handle Int8Array", async () => {
			async function* generator() {
				yield new Int8Array([1, 2, 3]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});

		it("should handle Int16Array", async () => {
			async function* generator() {
				yield new Int16Array([256, 512]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected).toBeInstanceOf(Uint8Array);
		});

		it("should handle Float32Array", async () => {
			async function* generator() {
				yield new Float32Array([1.5, 2.5, 3.5]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});

		it("should handle Float64Array", async () => {
			async function* generator() {
				yield new Float64Array([1.5, 2.5, 3.5]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});
	});

	describe("isStreamingResponse type guard", () => {
		it("should return true for StreamingResponse instances", () => {
			async function* generator() {
				yield "test";
			}

			const response = new StreamingResponse(generator());
			expect(isStreamingResponse(response)).toBe(true);
		});

		it("should return false for null", () => {
			expect(isStreamingResponse(null)).toBe(false);
		});

		it("should return false for undefined", () => {
			expect(isStreamingResponse(undefined)).toBe(false);
		});

		it("should return false for plain objects", () => {
			expect(isStreamingResponse({})).toBe(false);
		});

		it("should return false for regular objects with similar properties", () => {
			const fakeResponse = {
				statusCode: 200,
				headers: {},
				__spikard_streaming__: true,
			};

			expect(isStreamingResponse(fakeResponse)).toBe(false);
		});

		it("should return false for strings", () => {
			expect(isStreamingResponse("not a response")).toBe(false);
		});

		it("should return false for numbers", () => {
			expect(isStreamingResponse(200)).toBe(false);
		});

		it("should return false for arrays", () => {
			expect(isStreamingResponse([])).toBe(false);
		});

		it("should return false for functions", () => {
			expect(isStreamingResponse(() => {})).toBe(false);
		});

		it("should return false for falsy values", () => {
			expect(isStreamingResponse(0)).toBe(false);
			expect(isStreamingResponse("")).toBe(false);
			expect(isStreamingResponse(false)).toBe(false);
		});
	});

	describe("Complex streaming scenarios", () => {
		it("should handle mixed chunk types", async () => {
			async function* generator() {
				yield "start ";
				yield 123;
				yield " ";
				yield { status: "ok" };
				yield " ";
				yield new Uint8Array([101, 110, 100]);
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toContain("start");
			expect(text).toContain("123");
			expect(text).toContain("status");
			expect(text).toContain("end");
		});

		it("should handle error in generator", async () => {
			async function* generator() {
				yield "start";
				throw new Error("Stream error");
			}

			const response = new StreamingResponse(generator());

			try {
				await response.collect();
			} catch (e) {
				expect(e).toBeInstanceOf(Error);
			}
		});

		it("should handle very large individual chunks", async () => {
			async function* generator() {
				const largeString = "x".repeat(1000000);
				yield largeString;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBe(1000000);
		});

		it("should handle binary and text mixed", async () => {
			async function* generator() {
				yield "text";
				yield new Uint8Array([0, 1, 2, 3, 4]);
				yield "more text";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});

		it("should handle zero-length chunks", async () => {
			async function* generator() {
				yield "a";
				yield "";
				yield new Uint8Array();
				yield "b";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("ab");
		});
	});

	describe("Custom status codes and headers", () => {
		it("should preserve custom status codes", () => {
			async function* generator() {
				yield "data";
			}

			const statusCodes = [200, 201, 202, 206, 400, 404, 500];

			statusCodes.forEach((code) => {
				const response = new StreamingResponse(generator(), { statusCode: code });
				expect(response.statusCode).toBe(code);
			});
		});

		it("should preserve complex headers", () => {
			async function* generator() {
				yield "data";
			}

			const headers = {
				"Content-Type": "application/octet-stream",
				"Content-Disposition": 'attachment; filename="data.bin"',
				"Cache-Control": "no-cache",
				"X-Custom-Header": "custom-value",
				"Set-Cookie": "session=abc123; Path=/",
			};

			const response = new StreamingResponse(generator(), { headers });
			expect(response.headers).toEqual(headers);
		});

		it("should allow empty headers object", () => {
			async function* generator() {
				yield "data";
			}

			const response = new StreamingResponse(generator(), { headers: {} });
			expect(response.headers).toEqual({});
		});

		it("should handle undefined init parameter", () => {
			async function* generator() {
				yield "data";
			}

			const response = new StreamingResponse(generator());
			expect(response.statusCode).toBe(200);
			expect(response.headers).toEqual({});
		});
	});

	describe("Iterator wrapping", () => {
		it("should handle proper async iterator", async () => {
			const iterator = {
				next: async () => ({ done: false, value: "chunk" }),
				[Symbol.asyncIterator]() {
					return this;
				},
			};

			const response = new StreamingResponse(iterator);
			expect(response).toBeInstanceOf(StreamingResponse);
		});

		it("should wrap iterator without asyncIterator method", () => {
			const iterator = {
				next: async () => ({ done: false, value: "chunk" }),
			};

			expect(() => {
				new StreamingResponse(iterator as AsyncIterator<string>);
			}).not.toThrow();
		});

		it("should handle iterable that returns iterator", async () => {
			const iterable = {
				[Symbol.asyncIterator]: async function* () {
					yield "chunk";
				},
			};

			const response = new StreamingResponse(iterable);
			expect(response).toBeInstanceOf(StreamingResponse);
		});
	});

	describe("Edge cases", () => {
		it("should handle single chunk stream", async () => {
			async function* generator() {
				yield "only chunk";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toBe("only chunk");
		});

		it("should optimize single chunk in concatChunks", async () => {
			async function* generator() {
				yield "single";
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected).toBeInstanceOf(Uint8Array);
			expect(collected.length).toBe(6);
		});

		it("should handle chunk with byte offset", async () => {
			async function* generator() {
				const buffer = new ArrayBuffer(10);
				const view = new Uint8Array(buffer, 2, 5);
				view[0] = 72;
				view[1] = 101;
				view[2] = 108;
				view[3] = 108;
				view[4] = 111;
				yield view;
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			expect(collected.length).toBeGreaterThan(0);
		});

		it("should handle JSON.stringify on objects", async () => {
			async function* generator() {
				yield { nested: { deep: { object: "value" } } };
			}

			const response = new StreamingResponse(generator());
			const collected = await response.collect();

			const text = new TextDecoder().decode(collected);
			expect(text).toContain("nested");
			expect(text).toContain("deep");
		});
	});
});
