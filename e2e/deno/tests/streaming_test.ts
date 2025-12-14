/**
 * E2E tests for streaming
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "jsr:@std/assert@1";
import {
	createAppStreamingBinaryLogDownload,
	createAppStreamingChunkedCsvExport,
	createAppStreamingStreamJsonLines,
} from "../app/main.ts";

Deno.test("streaming: Stream JSON lines", async () => {
	const app = createAppStreamingStreamJsonLines();
	const client = new TestClient(app);

	const response = await client.get("/stream/json-lines");

	const expected = Buffer.from(
		"eyJpbmRleCI6MCwicGF5bG9hZCI6ImFscGhhIn1cbnsiaW5kZXgiOjEsInBheWxvYWQiOiJiZXRhIn1cbnsiaW5kZXgiOjIsInBheWxvYWQiOiJnYW1tYSJ9XG4=",
		"base64",
	);
	assertEquals(response.bytes(), expected);
	assertEquals(response.text(), expected.toString());
});

Deno.test("streaming: Binary log download", async () => {
	const app = createAppStreamingBinaryLogDownload();
	const client = new TestClient(app);

	const response = await client.get("/stream/logfile");

	const expected = Buffer.from("TE9HOgABAgN8VEFJTHwHXG4=", "base64");
	assertEquals(response.bytes(), expected);
});

Deno.test("streaming: Chunked CSV export", async () => {
	const app = createAppStreamingChunkedCsvExport();
	const client = new TestClient(app);

	const response = await client.get("/stream/csv-report");

	const expected = Buffer.from("aWQsbmFtZSx2YWx1ZVxuMSxBbGljZSw0MlxuMixCb2IsN1xu", "base64");
	assertEquals(response.bytes(), expected);
	assertEquals(response.text(), expected.toString());
});
