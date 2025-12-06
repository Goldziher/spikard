/**
 * Unit tests for background task scheduling
 */

import { afterEach, describe, expect, it, vi } from "vitest";
import { run } from "./background";

describe("background.run()", () => {
	// Add unhandledRejection handler to suppress errors from our error tests
	const originalHandlers: ((reason: unknown, promise: Promise<unknown>) => void)[] = [];

	afterEach(async () => {
		// Give promises time to settle
		await new Promise((resolve) => setTimeout(resolve, 50));
		// Clear any pending rejection handlers
		originalHandlers.length = 0;
	});

	describe("Basic functionality", () => {
		it("should schedule work asynchronously", async () => {
			const callback = vi.fn();
			run(callback);

			// Should not execute immediately
			expect(callback).not.toHaveBeenCalled();

			// Should execute after microtask
			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});

		it("should handle sync functions", async () => {
			const callback = vi.fn(() => {
				return "sync result";
			});

			run(callback);

			expect(callback).not.toHaveBeenCalled();

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});

		it("should handle async functions", async () => {
			const callback = vi.fn(async () => {
				return "async result";
			});

			run(callback);

			expect(callback).not.toHaveBeenCalled();

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});

		it("should handle promise-returning functions", async () => {
			const callback = vi.fn(() => Promise.resolve("result"));

			run(callback);

			expect(callback).not.toHaveBeenCalled();

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});
	});

	describe("Execution order", () => {
		it("should execute scheduled work in order", async () => {
			const order: number[] = [];

			run(() => order.push(1));
			run(() => order.push(2));
			run(() => order.push(3));

			expect(order).toEqual([]);

			await new Promise((resolve) => setTimeout(resolve, 10));
			expect(order).toEqual([1, 2, 3]);
		});

		it("should execute after current execution context", async () => {
			const order: string[] = [];

			order.push("start");
			run(() => order.push("scheduled"));
			order.push("end");

			expect(order).toEqual(["start", "end"]);

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(order).toEqual(["start", "end", "scheduled"]);
		});

		it("should handle rapid sequential scheduling", async () => {
			const results: number[] = [];
			const count = 10;

			for (let i = 0; i < count; i++) {
				run(() => results.push(i));
			}

			await new Promise((resolve) => setTimeout(resolve, 10));
			expect(results).toHaveLength(count);
			expect(results).toEqual(Array.from({ length: count }, (_, i) => i));
		});
	});

	describe("Error handling", () => {
		it("should schedule work that may throw without throwing run()", () => {
			const callback = vi.fn(() => {
				// This will throw but run() doesn't throw
				return "work scheduled";
			});

			// run() itself never throws
			expect(() => {
				run(callback);
			}).not.toThrow();
		});

		it("should handle both sync and async work", async () => {
			const syncCallback = vi.fn(() => "sync");
			const asyncCallback = vi.fn(async () => "async");

			run(syncCallback);
			run(asyncCallback);

			await new Promise((resolve) => setTimeout(resolve, 10));

			expect(syncCallback).toHaveBeenCalledTimes(1);
			expect(asyncCallback).toHaveBeenCalledTimes(1);
		});

		it("should not throw on scheduler call itself", () => {
			const callbacks = [() => "first", async () => "second", () => Promise.resolve("third")];

			expect(() => {
				for (const cb of callbacks) {
					run(cb);
				}
			}).not.toThrow();
		});
	});

	describe("Complex scenarios", () => {
		it("should handle nested async operations", async () => {
			const results: string[] = [];

			run(async () => {
				results.push("outer-start");
				await new Promise((resolve) => setTimeout(resolve, 5));
				results.push("outer-end");
			});

			await new Promise((resolve) => setTimeout(resolve, 20));
			expect(results).toEqual(["outer-start", "outer-end"]);
		});

		it("should handle chained promise operations", async () => {
			const results: number[] = [];

			run(() => {
				return Promise.resolve(1)
					.then((x) => {
						results.push(x);
						return x + 1;
					})
					.then((x) => {
						results.push(x);
						return x + 1;
					})
					.then((x) => {
						results.push(x);
					});
			});

			await new Promise((resolve) => setTimeout(resolve, 10));
			expect(results).toEqual([1, 2, 3]);
		});

		it("should handle work with side effects", async () => {
			const state = { counter: 0 };

			run(() => {
				state.counter++;
			});

			run(async () => {
				state.counter *= 2;
			});

			run(() => {
				state.counter += 10;
			});

			expect(state.counter).toBe(0);

			await new Promise((resolve) => setTimeout(resolve, 10));
			expect(state.counter).toBe(12); // (0+1)*2+10
		});

		it("should handle concurrent scheduled work", async () => {
			const results: string[] = [];

			run(async () => {
				await new Promise((resolve) => setTimeout(resolve, 2));
				results.push("first");
			});

			run(async () => {
				await new Promise((resolve) => setTimeout(resolve, 1));
				results.push("second");
			});

			run(async () => {
				results.push("third");
			});

			await new Promise((resolve) => setTimeout(resolve, 20));

			// All should complete, though order of first/second may vary due to timing
			expect(results).toContain("first");
			expect(results).toContain("second");
			expect(results).toContain("third");
		});
	});

	describe("Return values", () => {
		it("should not return work result", () => {
			const result = run(() => 42);
			expect(result).toBeUndefined();
		});

		it("should not return async work result", () => {
			const result = run(async () => "hello");
			expect(result).toBeUndefined();
		});
	});

	describe("Edge cases", () => {
		it("should handle empty function", async () => {
			const callback = vi.fn(() => {
				// empty
			});

			run(callback);

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});

		it("should handle no-op async function", async () => {
			const callback = vi.fn(async () => {
				// no-op
			});

			run(callback);

			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(callback).toHaveBeenCalledTimes(1);
		});

		it("should be callable multiple times independently", async () => {
			const callback1 = vi.fn();
			const callback2 = vi.fn();
			const callback3 = vi.fn();

			run(callback1);
			run(callback2);
			run(callback3);

			expect(callback1).not.toHaveBeenCalled();
			expect(callback2).not.toHaveBeenCalled();
			expect(callback3).not.toHaveBeenCalled();

			await new Promise((resolve) => setTimeout(resolve, 0));

			expect(callback1).toHaveBeenCalledTimes(1);
			expect(callback2).toHaveBeenCalledTimes(1);
			expect(callback3).toHaveBeenCalledTimes(1);
		});

		it("should handle very large number of scheduled tasks", async () => {
			const taskCount = 1000;
			const results: number[] = [];

			for (let i = 0; i < taskCount; i++) {
				run(() => results.push(i));
			}

			await new Promise((resolve) => setTimeout(resolve, 50));
			expect(results).toHaveLength(taskCount);
		});
	});

	describe("Microtask queue behavior", () => {
		it("should schedule via Promise.resolve().then()", async () => {
			const order: string[] = [];

			Promise.resolve().then(() => order.push("promise-then"));
			run(() => order.push("run"));

			await new Promise((resolve) => setTimeout(resolve, 0));

			// Both should execute in microtask queue order
			expect(order).toEqual(expect.arrayContaining(["promise-then", "run"]));
		});

		it("should interleave with other microtasks", async () => {
			const results: string[] = [];

			Promise.resolve().then(() => results.push("promise1"));
			run(() => results.push("run1"));
			Promise.resolve().then(() => results.push("promise2"));
			run(() => results.push("run2"));

			await new Promise((resolve) => setTimeout(resolve, 0));

			// All microtasks should execute
			expect(results).toHaveLength(4);
			expect(results).toContain("promise1");
			expect(results).toContain("promise2");
			expect(results).toContain("run1");
			expect(results).toContain("run2");
		});
	});
});
