/**
 * E2E tests for static_files
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
	createAppStaticFilesStaticFileServerReturnsTextFile,
	createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory,
} from "../app/main.ts";

describe("static_files", () => {
	test("Static file server returns text file", async () => {
		const app = createAppStaticFilesStaticFileServerReturnsTextFile();
		const client = new TestClient(app);

		const response = await client.get("/public/hello.txt");

		expect(response.statusCode).toBe(200);
		const responseText = response.text();
		expect(responseText.trimEnd()).toBe("Hello from static storage");
		const responseHeaders = response.headers();
		expect(responseHeaders["cache-control"]).toBe("public, max-age=60");
		expect(responseHeaders["content-type"]).toBe("text/plain");
	});

	test("Static server returns index html for directory", async () => {
		const app = createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory();
		const client = new TestClient(app);

		const response = await client.get("/app/");

		expect(response.statusCode).toBe(200);
		const responseText = response.text();
		expect(responseText.trimEnd()).toBe("<!doctype html><h1>Welcome</h1>");
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("text/html");
	});
});
