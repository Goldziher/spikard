/**
 * E2E tests for compression
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
	createAppCompressionCompressionGzipApplied,
	createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed,
} from "../app/main.js";

describe("compression", () => {
	test("Compression - payload below min_size is not compressed", async () => {
		const app = createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip",
		};
		const response = await client.get("/compression/skip", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Small payload");
		expect(responseData).toHaveProperty("payload");
		expect(responseData.payload).toBe("tiny");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-encoding"]).toBeUndefined();
	});

	test("Compression - gzip applied", async () => {
		const app = createAppCompressionCompressionGzipApplied();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip",
		};
		const response = await client.get("/compression/gzip", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Compressed payload");
		expect(responseData).toHaveProperty("payload");
		expect(responseData.payload).toBe(
			"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
		);
		const responseHeaders = response.headers();
		expect(responseHeaders.vary).toBe("Accept-Encoding");
		expect(responseHeaders["content-encoding"]).toBe("gzip");
	});
});
