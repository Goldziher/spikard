/**
 * E2E tests for compression
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import { createAppCompressionCompressionGzipApplied, createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed } from "../app/main.ts";

	Deno.test("compression: Compression - payload below min_size is not compressed", async () => {
		const app = createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip",
		};
		const response = await client.get("/compression/skip", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Small payload");
		assert(Object.hasOwn(responseData, "payload"));
		assertEquals(responseData.payload, "tiny");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-encoding"], undefined);
	});

	Deno.test("compression: Compression - gzip applied", async () => {
		const app = createAppCompressionCompressionGzipApplied();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip",
		};
		const response = await client.get("/compression/gzip", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Compressed payload");
		assert(Object.hasOwn(responseData, "payload"));
		assertEquals(responseData.payload, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Accept-Encoding");
		assertEquals(responseHeaders["content-encoding"], "gzip");
	});