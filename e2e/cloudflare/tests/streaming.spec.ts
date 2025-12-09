/**
 * E2E tests for streaming
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import {
	createAppStreamingBinaryLogDownload,
	createAppStreamingChunkedCsvExport,
	createAppStreamingStreamJsonLines,
} from "../app/main.ts";

describe("streaming", () => {
	test("Stream JSON lines", async () => {
		const app = createAppStreamingStreamJsonLines();
		const client = new TestClient(app);

		const response = await client.get("/stream/json-lines");

		const expected = Buffer.from(
			"eyJpbmRleCI6MCwicGF5bG9hZCI6ImFscGhhIn1cbnsiaW5kZXgiOjEsInBheWxvYWQiOiJiZXRhIn1cbnsiaW5kZXgiOjIsInBheWxvYWQiOiJnYW1tYSJ9XG4=",
			"base64",
		);
		expect(response.bytes()).toStrictEqual(expected);
		expect(response.text()).toBe(expected.toString());
	});

	test("Binary log download", async () => {
		const app = createAppStreamingBinaryLogDownload();
		const client = new TestClient(app);

		const response = await client.get("/stream/logfile");

		const expected = Buffer.from("TE9HOgABAgN8VEFJTHwHXG4=", "base64");
		expect(response.bytes()).toStrictEqual(expected);
	});

	test("Chunked CSV export", async () => {
		const app = createAppStreamingChunkedCsvExport();
		const client = new TestClient(app);

		const response = await client.get("/stream/csv-report");

		const expected = Buffer.from("aWQsbmFtZSx2YWx1ZVxuMSxBbGljZSw0MlxuMixCb2IsN1xu", "base64");
		expect(response.bytes()).toStrictEqual(expected);
		expect(response.text()).toBe(expected.toString());
	});
});
