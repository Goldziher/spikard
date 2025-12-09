/**
 * E2E tests for static_files
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "@std/assert";
import {
	createAppStaticFilesStaticFileServerReturnsTextFile,
	createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory,
} from "../app/main.ts";
Deno.test("static_files: Static file server returns text file", async () => {
	const app = createAppStaticFilesStaticFileServerReturnsTextFile();
	const client = new TestClient(app);

	const response = await client.get("/public/hello.txt");

	assertEquals(response.statusCode, 200);
	const responseText = response.text();
	assertEquals(responseText, "Hello from static storage");
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["content-type"], "text/plain");
	assertEquals(responseHeaders["cache-control"], "public, max-age=60");
});

Deno.test("static_files: Static server returns index html for directory", async () => {
	const app = createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory();
	const client = new TestClient(app);

	const response = await client.get("/app/");

	assertEquals(response.statusCode, 200);
	const responseText = response.text();
	assertEquals(responseText, "<!doctype html><h1>Welcome</h1>");
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["content-type"], "text/html");
});
