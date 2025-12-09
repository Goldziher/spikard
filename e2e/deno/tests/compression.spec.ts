/**
 * E2E tests for compression
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "@std/assert";
import {
	createAppCompressionCompressionGzipApplied,
	createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed,
} from "../app/main.ts";
Deno.test("compression: Compression - payload below min_size is not compressed", async () => {
	const app = createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed();
	const client = new TestClient(app);

	const headers = {
		"Accept-Encoding": "gzip",
	};
	const response = await client.get("/compression/skip", headers);

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("message");
	expect(responseData.message).toBe("Small payload");
	expect(responseData).toHaveProperty("payload");
	expect(responseData.payload).toBe("tiny");
	const responseHeaders = response.headers();
	expect(responseHeaders["content-encoding"]).toBeUndefined();
});

Deno.test("compression: Compression - gzip applied", async () => {
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
