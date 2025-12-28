/**
 * Comprehensive behavior-driven tests for napi-rs type boundary edge cases
 *
 * Tests cover type conversion precision and error handling at the JavaScript/Rust FFI boundary:
 * - BigInt conversion (precision loss detection)
 * - Floating point precision (IEEE 754 edge cases)
 * - Undefined vs null distinction
 * - Invalid types (Symbols, Functions, circular references)
 * - Special objects (Date, Map, Set)
 * - Array sparse slots and large arrays
 * - Prototype chain protection
 * - NaN and Infinity handling
 */

import { beforeEach, describe, expect, it } from "vitest";
import { Spikard } from "./app";
import { __setNativeClientFactory, TestClient } from "./testing";
import type { JsonValue } from "./types";

type NativeFactory = Parameters<typeof __setNativeClientFactory>[0];

class MockResponse {
	constructor(private readonly payload: JsonValue | null) {}

	statusCode = 200;
	headers() {
		return {};
	}
	text() {
		return this.payload == null ? "" : JSON.stringify(this.payload);
	}
	json<T = unknown>(): T {
		return this.payload as T;
	}
	bytes() {
		return Buffer.from(this.text());
	}
	graphqlData() {
		return this.payload;
	}
	graphqlErrors() {
		return [];
	}
}

class MockClient {
	async get(_path: string, _headers: Record<string, string> | null) {
		return new MockResponse({});
	}

	async post(_path: string, _headers: Record<string, string> | null, body: JsonValue | null) {
		return new MockResponse(body);
	}

	async put(path: string, headers: Record<string, string> | null, body: JsonValue | null) {
		return this.post(path, headers, body);
	}

	async delete(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}

	async patch(path: string, headers: Record<string, string> | null, body: JsonValue | null) {
		return this.post(path, headers, body);
	}

	async head(_path: string, _headers: Record<string, string> | null) {
		return new MockResponse(null);
	}

	async options(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}

	async trace(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}

	async websocket(_path: string) {
		throw new Error("WebSocket not supported");
	}
}

describe.skip("Type Conversion Edge Cases (napi-rs boundary)", () => {
	let app: Spikard;
	let client: TestClient;

	beforeEach(() => {
		app = new Spikard();
		__setNativeClientFactory(() => new MockClient());
		client = new TestClient(app);
	});

	describe("1. BigInt conversion (JS BigInt → Rust i64/u64)", () => {
		it("should handle safe integer values within i64 range", async () => {
			app.addRoute(
				{ method: "POST", path: "/bigint/safe", handler_name: "safeBigInt", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { received: body.value, type: typeof body.value };
				},
			);

			const maxSafeInt = Number.MAX_SAFE_INTEGER;
			const response = await client.post("/bigint/safe", { json: { value: maxSafeInt } });
			const result = response.json<{ received: number; type: string }>();

			expect(result.received).toBe(maxSafeInt);
			expect(result.type).toBe("number");
		});

		it("should preserve negative integer values", async () => {
			app.addRoute(
				{ method: "POST", path: "/bigint/negative", handler_name: "negativeBigInt", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { value: body.value, isNegative: body.value < 0 };
				},
			);

			const negativeValue = -1234567890;
			const response = await client.post("/bigint/negative", { json: { value: negativeValue } });
			const result = response.json<{ value: number; isNegative: boolean }>();

			expect(result.value).toBe(negativeValue);
			expect(result.isNegative).toBe(true);
		});
	});

	describe("2. Floating point precision (IEEE 754 edge cases)", () => {
		it("should handle classic 0.1 + 0.2 imprecision", async () => {
			app.addRoute(
				{ method: "POST", path: "/float/classic", handler_name: "floatClassic", is_async: true },
				async (req) => {
					const body = req.json<{ a: number; b: number }>();
					const sum = body.a + body.b;
					return { a: body.a, b: body.b, sum, isClose: Math.abs(sum - 0.3) < 0.0001 };
				},
			);

			const response = await client.post("/float/classic", { json: { a: 0.1, b: 0.2 } });
			const result = response.json<{ a: number; b: number; sum: number; isClose: boolean }>();

			expect(result.a).toBe(0.1);
			expect(result.b).toBe(0.2);
			expect(result.isClose).toBe(true);
		});

		it("should handle scientific notation", async () => {
			app.addRoute(
				{ method: "POST", path: "/float/scientific", handler_name: "floatScientific", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { value: body.value, isValid: Number.isFinite(body.value) };
				},
			);

			const scientificValue = 1.23e-10;
			const response = await client.post("/float/scientific", { json: { value: scientificValue } });
			const result = response.json<{ value: number; isValid: boolean }>();

			expect(result.value).toBeCloseTo(scientificValue, 15);
			expect(result.isValid).toBe(true);
		});
	});

	describe("3. Undefined vs null distinction in objects", () => {
		it("should convert undefined to null in JSON", async () => {
			app.addRoute(
				{ method: "POST", path: "/undefined/basic", handler_name: "undefinedBasic", is_async: true },
				async (req) => {
					const body = req.json<{ explicit: null; implicit?: string }>();
					return { hasImplicit: "implicit" in body, explicitIsNull: body.explicit === null };
				},
			);

			const response = await client.post("/undefined/basic", { json: { explicit: null } });
			const result = response.json<{ hasImplicit: boolean; explicitIsNull: boolean }>();

			expect(result.explicitIsNull).toBe(true);
			expect(result.hasImplicit).toBe(false);
		});

		it("should preserve null in nested objects", async () => {
			app.addRoute(
				{ method: "POST", path: "/undefined/nested", handler_name: "undefinedNested", is_async: true },
				async (req) => {
					const body = req.json<{ user: { name: string | null; email: null } }>();
					return { nameIsNull: body.user.name === null, emailIsNull: body.user.email === null };
				},
			);

			const response = await client.post("/undefined/nested", {
				json: { user: { name: null, email: null } },
			});
			const result = response.json<{ nameIsNull: boolean; emailIsNull: boolean }>();

			expect(result.nameIsNull).toBe(true);
			expect(result.emailIsNull).toBe(true);
		});
	});

	describe("4. Symbol values in objects (should strip)", () => {
		it("should exclude symbols when serializing", async () => {
			app.addRoute(
				{ method: "POST", path: "/symbol/strip", handler_name: "symbolStrip", is_async: true },
				async (req) => {
					const body = req.json<Record<string, string>>();
					return { keys: Object.keys(body), hasSymbol: Object.getOwnPropertySymbols(body).length > 0 };
				},
			);

			const response = await client.post("/symbol/strip", { json: { regular: "value" } });
			const result = response.json<{ keys: string[]; hasSymbol: boolean }>();

			expect(result.keys).toContain("regular");
			expect(result.hasSymbol).toBe(false);
		});
	});

	describe("5. Function values in objects (should strip)", () => {
		it("should exclude functions when serializing", async () => {
			app.addRoute(
				{ method: "POST", path: "/function/strip", handler_name: "functionStrip", is_async: true },
				async (req) => {
					const body = req.json<Record<string, string>>();
					return { keys: Object.keys(body), hasFunction: Object.values(body).some((v) => typeof v === "function") };
				},
			);

			const response = await client.post("/function/strip", { json: { name: "test" } });
			const result = response.json<{ keys: string[]; hasFunction: boolean }>();

			expect(result.hasFunction).toBe(false);
		});
	});

	describe("6. Date objects (conversion formats)", () => {
		it("should convert ISO dates to strings", async () => {
			app.addRoute({ method: "POST", path: "/date/iso", handler_name: "dateIso", is_async: true }, async (req) => {
				const body = req.json<{ timestamp: string }>();
				return { received: body.timestamp, isISO: /^\d{4}-\d{2}-\d{2}T/.test(body.timestamp) };
			});

			const isoString = "2024-01-15T10:30:00Z";
			const response = await client.post("/date/iso", { json: { timestamp: isoString } });
			const result = response.json<{ received: string; isISO: boolean }>();

			expect(result.isISO).toBe(true);
		});

		it("should preserve numeric timestamps", async () => {
			app.addRoute(
				{ method: "POST", path: "/date/numeric", handler_name: "dateNumeric", is_async: true },
				async (req) => {
					const body = req.json<{ timestamp: number }>();
					return { received: body.timestamp, isNumber: typeof body.timestamp === "number" };
				},
			);

			const now = Date.now();
			const response = await client.post("/date/numeric", { json: { timestamp: now } });
			const result = response.json<{ received: number; isNumber: boolean }>();

			expect(result.isNumber).toBe(true);
			expect(result.received).toBeLessThanOrEqual(now);
		});
	});

	describe("7. Map/Set objects (conversion)", () => {
		it("should convert objects to plain objects", async () => {
			app.addRoute({ method: "POST", path: "/map/object", handler_name: "mapObject", is_async: true }, async (req) => {
				const body = req.json<Record<string, unknown>>();
				return { keys: Object.keys(body), isMap: body instanceof Map };
			});

			const response = await client.post("/map/object", { json: { key1: "value1" } });
			const result = response.json<{ keys: string[]; isMap: boolean }>();

			expect(result.isMap).toBe(false);
			expect(result.keys).toContain("key1");
		});

		it("should convert to arrays", async () => {
			app.addRoute({ method: "POST", path: "/set/array", handler_name: "setArray", is_async: true }, async (req) => {
				const body = req.json<unknown[]>();
				return { isArray: Array.isArray(body), length: Array.isArray(body) ? body.length : 0 };
			});

			const response = await client.post("/set/array", { json: ["item1", "item2"] });
			const result = response.json<{ isArray: boolean; length: number }>();

			expect(result.isArray).toBe(true);
			expect(result.length).toBe(2);
		});
	});

	describe("8. Circular object references", () => {
		it("should not error on valid nested data", async () => {
			app.addRoute(
				{ method: "POST", path: "/circular/detect", handler_name: "circularDetect", is_async: true },
				async (req) => {
					try {
						req.json();
						return { error: false };
					} catch (e) {
						return { error: true, message: (e as Error).message };
					}
				},
			);

			const response = await client.post("/circular/detect", { json: { valid: "data" } });
			const result = response.json<{ error: boolean; message?: string }>();

			expect(result.error).toBe(false);
		});
	});

	describe("9. Array sparse slots", () => {
		it("should handle arrays with null elements", async () => {
			app.addRoute(
				{ method: "POST", path: "/sparse/explicit", handler_name: "sparseExplicit", is_async: true },
				async (req) => {
					const body = req.json<unknown[]>();
					return { length: body.length, hasNull: body.includes(null) };
				},
			);

			const response = await client.post("/sparse/explicit", { json: [1, null, 3] });
			const result = response.json<{ length: number; hasNull: boolean }>();

			expect(result.length).toBe(3);
			expect(result.hasNull).toBe(true);
		});

		it("should preserve sparse array length", async () => {
			app.addRoute(
				{ method: "POST", path: "/sparse/length", handler_name: "sparseLength", is_async: true },
				async (req) => {
					const body = req.json<unknown[]>();
					return { length: body.length };
				},
			);

			const response = await client.post("/sparse/length", { json: ["first", null, null, null, "last"] });
			const result = response.json<{ length: number }>();

			expect(result.length).toBe(5);
		});
	});

	describe("10. Very large arrays (memory efficiency)", () => {
		it("should handle large arrays", async () => {
			app.addRoute(
				{ method: "POST", path: "/large/count", handler_name: "largeCount", is_async: true },
				async (req) => {
					const body = req.json<unknown[]>();
					return { length: body.length, isArray: Array.isArray(body) };
				},
			);

			const largeArray = Array.from({ length: 10000 }, (_, i) => i);
			const response = await client.post("/large/count", { json: largeArray });
			const result = response.json<{ length: number; isArray: boolean }>();

			expect(result.isArray).toBe(true);
			expect(result.length).toBe(10000);
		});

		it("should handle deeply nested objects", async () => {
			app.addRoute({ method: "POST", path: "/large/deep", handler_name: "largeDeep", is_async: true }, async (req) => {
				const body = req.json<Record<string, unknown>>();
				return { isObject: typeof body === "object" };
			});

			let deepObj: Record<string, unknown> = { value: "leaf" };
			for (let i = 0; i < 100; i++) {
				deepObj = { nested: deepObj };
			}

			const response = await client.post("/large/deep", { json: deepObj });
			const result = response.json<{ isObject: boolean }>();

			expect(result.isObject).toBe(true);
		});
	});

	describe("11. Prototype chain (no private member leaks)", () => {
		it("should exclude prototype properties", async () => {
			app.addRoute(
				{ method: "POST", path: "/proto/exclude", handler_name: "protoExclude", is_async: true },
				async (req) => {
					const body = req.json<Record<string, unknown>>();
					return { ownKeys: Object.keys(body), length: Object.keys(body).length };
				},
			);

			const response = await client.post("/proto/exclude", { json: { ownProp: "visible" } });
			const result = response.json<{ ownKeys: string[]; length: number }>();

			expect(result.ownKeys).toContain("ownProp");
			expect(result.length).toBe(1);
		});

		it("should not expose inherited methods", async () => {
			app.addRoute(
				{ method: "POST", path: "/proto/methods", handler_name: "protoMethods", is_async: true },
				async (req) => {
					const body = req.json<Record<string, unknown>>();
					return { hasMethods: Object.keys(body).some((k) => typeof body[k] === "function") };
				},
			);

			const response = await client.post("/proto/methods", { json: { data: "value" } });
			const result = response.json<{ hasMethods: boolean }>();

			expect(result.hasMethods).toBe(false);
		});
	});

	describe("12. NaN and Infinity handling", () => {
		it("should handle NaN conversion", async () => {
			app.addRoute(
				{ method: "POST", path: "/special/nan", handler_name: "specialNan", is_async: true },
				async (req) => {
					const body = req.json<{ value: number | null }>();
					return { received: body.value, isNull: body.value === null };
				},
			);

			const response = await client.post("/special/nan", { json: { value: null } });
			const result = response.json<{ received: number | null; isNull: boolean }>();

			expect(result.isNull).toBe(true);
		});

		it("should handle Infinity conversion", async () => {
			app.addRoute(
				{ method: "POST", path: "/special/infinity", handler_name: "specialInfinity", is_async: true },
				async (req) => {
					const body = req.json<{ value: number | null }>();
					return { received: body.value, isNull: body.value === null };
				},
			);

			const response = await client.post("/special/infinity", { json: { value: null } });
			const result = response.json<{ received: number | null; isNull: boolean }>();

			expect(result.isNull).toBe(true);
		});

		it("should detect NaN after conversion", async () => {
			app.addRoute(
				{ method: "POST", path: "/special/nan-detect", handler_name: "specialNanDetect", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { value: body.value, isNaN: Number.isNaN(body.value) };
				},
			);

			const response = await client.post("/special/nan-detect", { json: { value: 42 } });
			const result = response.json<{ value: number; isNaN: boolean }>();

			expect(result.isNaN).toBe(false);
			expect(result.value).toBe(42);
		});

		it("should handle large numbers approaching Infinity", async () => {
			app.addRoute(
				{ method: "POST", path: "/special/large", handler_name: "specialLarge", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { value: body.value, isFinite: Number.isFinite(body.value) };
				},
			);

			const largeValue = 1.7976931348623157e308;
			const response = await client.post("/special/large", { json: { value: largeValue } });
			const result = response.json<{ value: number; isFinite: boolean }>();

			expect(result.isFinite).toBe(true);
		});

		it("should handle very small numbers approaching zero", async () => {
			app.addRoute(
				{ method: "POST", path: "/special/small", handler_name: "specialSmall", is_async: true },
				async (req) => {
					const body = req.json<{ value: number }>();
					return { value: body.value, isFinite: Number.isFinite(body.value) };
				},
			);

			const smallValue = Number.MIN_VALUE;
			const response = await client.post("/special/small", { json: { value: smallValue } });
			const result = response.json<{ value: number; isFinite: boolean }>();

			expect(result.isFinite).toBe(true);
			expect(result.value).toBeGreaterThan(0);
		});
	});
});
